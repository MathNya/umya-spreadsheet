use std::{
    collections::HashMap,
    io,
    sync::{
        Arc,
        RwLock,
    },
};

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    Row,
    helper::const_str::{
        MC_NS,
        PKG_SHEET,
        REL_OFC_NS,
        SHEET_DRAWING_NS,
        SHEET_MAIN_NS,
        SHEET_MS_MAIN_NS,
        SHEETML_AC_NS,
    },
    structs::{
        Cell,
        SharedStringTable,
        Stylesheet,
        Worksheet,
        WriterManager,
    },
};

type InternalWriter = Writer<io::Cursor<Vec<u8>>>;

/// Writes a worksheet to the XLSX file format.
///
/// # Arguments
///
/// * `sheet_no` - The sheet number
/// * `worksheet` - The worksheet to write
/// * `shared_string_table` - Table containing shared strings
/// * `stylesheet` - The workbook's stylesheet
/// * `has_macros` - Whether the workbook contains macros
/// * `writer_mng` - The writer manager handling file output
///
/// # Returns
///
/// Returns `Result<(), XlsxError>` indicating success or failure
pub(crate) fn write<W: io::Seek + io::Write>(
    sheet_no: i32,
    worksheet: &Worksheet,
    shared_string_table: &Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
    has_macros: bool,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));

    write_worksheet_header(&mut writer);
    write_worksheet_properties(&mut writer, worksheet, has_macros);
    write_dimension_and_views(&mut writer, worksheet);
    write_columns_and_rows(&mut writer, worksheet, shared_string_table, stylesheet);
    write_worksheet_features(&mut writer, worksheet, stylesheet);
    write_worksheet_extensions(&mut writer, worksheet);

    write_end_tag(&mut writer, "worksheet");

    let target = format!("{PKG_SHEET}{sheet_no}.xml");
    writer_mng.add_writer(&target, writer)
}

/// Writes the XML header and worksheet opening tag with required namespace
/// declarations.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
fn write_worksheet_header(writer: &mut InternalWriter) {
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(writer);

    write_start_tag(
        writer,
        "worksheet",
        vec![
            ("xmlns", SHEET_MAIN_NS).into(),
            ("xmlns:r", REL_OFC_NS).into(),
            ("xmlns:xdr", SHEET_DRAWING_NS).into(),
            ("xmlns:x14", SHEET_MS_MAIN_NS).into(),
            ("xmlns:mc", MC_NS).into(),
            ("mc:Ignorable", "x14ac").into(),
            ("xmlns:x14ac", SHEETML_AC_NS).into(),
        ],
        false,
    );
}

/// Writes worksheet properties including sheet protection and tab color
/// settings.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing the properties
/// * `has_macros` - Whether the workbook contains VBA macros
fn write_worksheet_properties(
    writer: &mut InternalWriter,
    worksheet: &Worksheet,
    has_macros: bool,
) {
    let mut attributes: crate::structs::AttrCollection = Vec::new();
    if has_macros {
        let code_name = if worksheet.has_code_name() {
            worksheet.code_name().as_ref().unwrap()
        } else {
            worksheet.name()
        };
        attributes.push(("codeName", code_name).into());
    }

    match worksheet.tab_color() {
        Some(v) => {
            write_start_tag(writer, "sheetPr", attributes, false);
            v.write_to_tab_color(writer);
            write_end_tag(writer, "sheetPr");
        }
        None => {
            if !attributes.is_empty() {
                write_start_tag(writer, "sheetPr", attributes, true);
            }
        }
    }
}

/// Writes the worksheet dimension (used range) and sheet view settings.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing dimension and view settings
fn write_dimension_and_views(writer: &mut InternalWriter, worksheet: &Worksheet) {
    write_start_tag(
        writer,
        "dimension",
        vec![("ref", worksheet.calculate_worksheet_dimension().as_str()).into()],
        true,
    );

    worksheet.sheets_views().write_to(writer);
    worksheet.sheet_format_properties().write_to(writer);
}

/// Writes column definitions and row data including cell contents.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing the data
/// * `shared_string_table` - Table containing shared string values
/// * `stylesheet` - The workbook's stylesheet for formatting
fn write_columns_and_rows(
    writer: &mut InternalWriter,
    worksheet: &Worksheet,
    shared_string_table: &Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
) {
    let mut column_dimensions = worksheet.column_dimensions_crate().clone();
    column_dimensions
        .calculation_auto_width(worksheet.cells_crate(), worksheet.merge_cells_crate());
    column_dimensions.write_to(writer, stylesheet);

    write_sheet_data(writer, worksheet, shared_string_table, stylesheet);
}

/// Writes the sheet data section containing rows and cells.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing the data
/// * `shared_string_table` - Table containing shared string values
/// * `stylesheet` - The workbook's stylesheet for formatting
fn write_sheet_data(
    writer: &mut InternalWriter,
    worksheet: &Worksheet,
    shared_string_table: &Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
) {
    let has_sheet_data = worksheet.has_sheet_data();
    write_start_tag(writer, "sheetData", vec![], !has_sheet_data);

    let mut row_dimensions = worksheet.row_dimensions();
    row_dimensions.sort_by_key(|a| a.get_row_num());
    let cells = worksheet.cells_sorted();
    let formula_shared_list = build_formula_shared_list(&cells);
    write_rows_and_cells(
        writer,
        &row_dimensions,
        &cells,
        shared_string_table,
        stylesheet,
        &formula_shared_list,
    );

    if has_sheet_data {
        write_end_tag(writer, "sheetData");
    }
}

/// Creates a map of shared formulas indexed by their shared formula ID.
///
/// # Arguments
///
/// * `cells` - Slice of cell references to process
///
/// # Returns
///
/// A `HashMap` mapping formula IDs to tuples of (`start_cell_ref`,
/// `end_cell_ref`)
fn build_formula_shared_list(cells: &[&Cell]) -> HashMap<u32, (String, Option<String>)> {
    let mut formula_shared_list: HashMap<u32, (String, Option<String>)> = HashMap::new();
    for cell in cells {
        if let Some(si) = cell.formula_shared_index() {
            match formula_shared_list.get(&si) {
                Some((start_cell, _)) => {
                    formula_shared_list.insert(
                        si,
                        (start_cell.clone(), Some(cell.coordinate().get_coordinate())),
                    );
                }
                None => {
                    formula_shared_list.insert(si, (cell.coordinate().get_coordinate(), None));
                }
            }
        }
    }
    formula_shared_list
}

/// Writes worksheet features including protection, filters, merged cells, and
/// conditional formatting.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing the features
/// * `stylesheet` - The workbook's stylesheet for formatting
fn write_worksheet_features(
    writer: &mut InternalWriter,
    worksheet: &Worksheet,
    stylesheet: &mut Stylesheet,
) {
    if let Some(v) = worksheet.sheet_protection() {
        v.write_to(writer);
    }

    if let Some(v) = worksheet.auto_filter() {
        write_start_tag(
            writer,
            "autoFilter",
            vec![("ref", &v.range().range()).into()],
            true,
        );
    }

    worksheet.merge_cells_crate().write_to(writer);
    write_start_tag(writer, "phoneticPr", vec![("fontId", "1").into()], true);

    for conditional_formatting in worksheet.conditional_formatting_collection() {
        conditional_formatting.write_to(writer, stylesheet.get_differential_formats_mut());
    }

    if let Some(v) = worksheet.data_validations() {
        v.write_to(writer);
    }
}

/// Writes worksheet extensions including hyperlinks, print settings, drawings,
/// and tables.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing the extensions
fn write_worksheet_extensions(writer: &mut InternalWriter, worksheet: &Worksheet) {
    let mut r_id = write_hyperlinks(writer, worksheet);
    r_id = write_print_settings(writer, worksheet, r_id);
    r_id = write_drawings(writer, worksheet, r_id);
    write_tables_and_objects(writer, worksheet, r_id);

    if worksheet.data_validations_2010().is_some() {
        write_start_tag(writer, "extLst", vec![], false);
        if let Some(v) = worksheet.data_validations_2010() {
            v.write_to(writer);
        }
        write_end_tag(writer, "extLst");
    }
}

/// Writes rows and their contained cells to the worksheet.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `row_dimensions` - Slice of row references to write
/// * `cells` - Slice of cell references to write
/// * `shared_string_table` - Table containing shared string values
/// * `stylesheet` - The workbook's stylesheet for formatting
/// * `formula_shared_list` - Map of shared formula definitions
fn write_rows_and_cells(
    writer: &mut InternalWriter,
    row_dimensions: &[&Row],
    cells: &[&Cell],
    shared_string_table: &Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
    formula_shared_list: &HashMap<u32, (String, Option<String>)>,
) {
    let mut cells_iter = cells.iter().peekable();

    for row in row_dimensions {
        let mut cells_in_row: Vec<&Cell> = Vec::new();

        while let Some(cell) = cells_iter.peek() {
            if row.get_row_num() != cell.coordinate().row_num() {
                break;
            }
            cells_in_row.push(cells_iter.next().unwrap());
        }

        write_row_with_cells(
            writer,
            row,
            &cells_in_row,
            shared_string_table,
            stylesheet,
            formula_shared_list,
        );
    }
}

/// Writes a single row and its contained cells.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `row` - The row to write
/// * `cells_in_row` - Slice of cells belonging to this row
/// * `shared_string_table` - Table containing shared string values
/// * `stylesheet` - The workbook's stylesheet for formatting
/// * `formula_shared_list` - Map of shared formula definitions
fn write_row_with_cells(
    writer: &mut InternalWriter,
    row: &Row,
    cells_in_row: &[&Cell],
    shared_string_table: &Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
    formula_shared_list: &HashMap<u32, (String, Option<String>)>,
) {
    if cells_in_row.is_empty() {
        let spans = "0:0";
        row.write_to(writer, stylesheet, spans, true);
    } else {
        let (first_num, last_num) = (
            cells_in_row.first().unwrap().coordinate().col_num(),
            cells_in_row.last().unwrap().coordinate().col_num(),
        );
        let spans = format!("{first_num}:{last_num}");

        row.write_to(writer, stylesheet, &spans, false);

        for cell in cells_in_row {
            cell.write_to(writer, shared_string_table, stylesheet, formula_shared_list);
        }

        write_end_tag(writer, "row");
    }
}

/// Writes hyperlinks and returns the next available relationship ID.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing hyperlinks
///
/// # Returns
///
/// The next available relationship ID
fn write_hyperlinks(writer: &mut InternalWriter, worksheet: &Worksheet) -> i32 {
    let mut r_id = 1;

    if worksheet.has_hyperlink() {
        write_start_tag(writer, "hyperlinks", vec![], false);

        for (coordition, hyperlink) in worksheet.hyperlink_collection_to_hashmap() {
            let r_id_str = format!("rId{}", &r_id);
            let mut attributes: crate::structs::AttrCollection = Vec::new();
            attributes.push(("ref", &coordition).into());
            if hyperlink.get_location() {
                attributes.push(("location", hyperlink.get_url()).into());
            } else {
                attributes.push(("r:id", r_id_str.as_str()).into());
                r_id += 1;
            }
            write_start_tag(writer, "hyperlink", attributes, true);
        }

        write_end_tag(writer, "hyperlinks");
    }

    r_id
}

/// Writes print settings and returns the next available relationship ID.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing print settings
/// * `r_id` - The current relationship ID
///
/// # Returns
///
/// The next available relationship ID
fn write_print_settings(writer: &mut InternalWriter, worksheet: &Worksheet, r_id: i32) -> i32 {
    worksheet.print_options().write_to(writer);
    worksheet.page_margins().write_to(writer);

    if worksheet.page_setup().has_param() {
        worksheet
            .page_setup()
            .write_to(writer, &mut num_traits::cast(r_id).unwrap());
    }

    worksheet.header_footer().write_to(writer);
    worksheet.row_breaks().write_to(writer);
    worksheet.column_breaks().write_to(writer);

    r_id
}

/// Writes drawing objects and returns the next available relationship ID.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing drawings
/// * `r_id` - The current relationship ID
///
/// # Returns
///
/// The next available relationship ID
fn write_drawings(writer: &mut InternalWriter, worksheet: &Worksheet, mut r_id: i32) -> i32 {
    if worksheet.has_drawing_object() {
        let r_id_str = format!("rId{}", &r_id);
        write_start_tag(
            writer,
            "drawing",
            vec![("r:id", r_id_str.as_str()).into()],
            true,
        );
        r_id += 1;
    }

    if worksheet.has_legacy_drawing() {
        let r_id_str = format!("rId{}", &r_id);
        write_start_tag(
            writer,
            "legacyDrawing",
            vec![("r:id", r_id_str.as_str()).into()],
            true,
        );
        r_id += 1;
    }

    r_id
}

/// Writes tables and OLE objects.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `worksheet` - The worksheet containing tables and objects
/// * `r_id` - The current relationship ID
fn write_tables_and_objects(writer: &mut InternalWriter, worksheet: &Worksheet, mut r_id: i32) {
    if worksheet.has_table() {
        let tables = worksheet.tables();
        write_start_tag(
            writer,
            "tableParts",
            vec![("count", &tables.len().to_string()).into()],
            false,
        );
        for _table in worksheet.tables() {
            let r_id_str = format!("rId{}", &r_id);
            write_start_tag(
                writer,
                "tablePart",
                vec![("r:id", r_id_str.as_str()).into()],
                true,
            );
            r_id += 1;
        }
        write_end_tag(writer, "tableParts");
    }

    let ole_id = 1000 + 25;
    worksheet
        .ole_objects()
        .write_to(writer, num_traits::cast(r_id).unwrap(), ole_id);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn setup_test_writer() -> InternalWriter {
        Writer::new(Cursor::new(Vec::new()))
    }

    fn setup_test_worksheet() -> Worksheet {
        let mut ws = Worksheet::default();
        ws.set_name("Test Sheet");
        ws
    }

    fn setup_shared_string_table() -> Arc<RwLock<SharedStringTable>> {
        Arc::new(RwLock::new(SharedStringTable::default()))
    }

    fn setup_stylesheet() -> Stylesheet {
        Stylesheet::default()
    }

    #[test]
    fn test_write_worksheet_header() {
        let mut writer = setup_test_writer();
        write_worksheet_header(&mut writer);

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("<?xml"));
        assert!(result.contains("worksheet"));
        assert!(result.contains(SHEET_MAIN_NS));
    }

    #[test]
    fn test_write_worksheet_properties() {
        let mut writer = setup_test_writer();
        let worksheet = setup_test_worksheet();

        // Test without macros
        write_worksheet_properties(&mut writer, &worksheet, false);
        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(!result.contains("codeName"));

        // Test with macros
        let mut writer = setup_test_writer();
        write_worksheet_properties(&mut writer, &worksheet, true);
        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("codeName"));
    }

    #[test]
    fn test_write_dimension_and_views() {
        let mut writer = setup_test_writer();
        let worksheet = setup_test_worksheet();

        write_dimension_and_views(&mut writer, &worksheet);
        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("dimension"));
    }

    #[test]
    fn test_write_rows_and_cells() {
        let mut writer = setup_test_writer();
        let mut row = Row::default();
        row.set_row_num(1);
        let mut cell = Cell::default();
        cell.set_coordinate("C1");
        let shared_string_table = setup_shared_string_table();
        let mut stylesheet = setup_stylesheet();
        let formula_shared_list = HashMap::new();

        write_rows_and_cells(
            &mut writer,
            &[&row],
            &[&cell],
            &shared_string_table,
            &mut stylesheet,
            &formula_shared_list,
        );

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("<row r=\"1\" spans=\"3:3\">"));
    }

    #[test]
    fn test_write_row_with_cells() {
        let mut writer = setup_test_writer();
        let mut row = Row::default();
        row.set_row_num(1);
        let mut cell = Cell::default();
        cell.set_coordinate("A1");
        let shared_string_table = setup_shared_string_table();
        let mut stylesheet = setup_stylesheet();
        let formula_shared_list = HashMap::new();

        write_row_with_cells(
            &mut writer,
            &row,
            &[&cell],
            &shared_string_table,
            &mut stylesheet,
            &formula_shared_list,
        );

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("<row r=\"1\" spans=\"1:1\">"));
    }

    // fn test_write_hyperlinks() {
    //     let mut writer = setup_test_writer();
    //     let mut worksheet = setup_test_worksheet();

    //     // Add a test hyperlink
    //     worksheet.add_hyperlink("A1", "http://example.com");

    //     let r_id = write_hyperlinks(&mut writer, &worksheet);

    //     let result =
    // String::from_utf8(writer.into_inner().into_inner()).unwrap();     assert!
    // (result.contains("hyperlinks"));     assert!(result.contains("hyperlink"
    // ));     assert!(r_id > 1);
    // }
    #[test]
    fn test_write_print_settings() {
        let mut writer = setup_test_writer();
        let worksheet = setup_test_worksheet();

        let r_id = write_print_settings(&mut writer, &worksheet, 1);

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(result.contains("pageMargins"));
        assert_eq!(r_id, 1);
    }

    #[test]
    fn test_write_drawings() {
        let mut writer = setup_test_writer();
        let worksheet = setup_test_worksheet();

        let r_id = write_drawings(&mut writer, &worksheet, 1);

        // Should return same r_id if no drawings
        assert_eq!(r_id, 1);
    }

    #[test]
    fn test_write_tables_and_objects() {
        let mut writer = setup_test_writer();
        let worksheet = setup_test_worksheet();

        write_tables_and_objects(&mut writer, &worksheet, 1);

        let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        // Should be empty if no tables/objects
        assert!(!result.contains("tableParts"));
    }

    #[test]
    fn test_write_complete_worksheet() {
        let cursor = Cursor::new(Vec::new());
        let mut arv = zip::ZipWriter::new(cursor);
        let mut writer_manager = WriterManager::new(&mut arv);
        let worksheet = setup_test_worksheet();
        let shared_string_table = setup_shared_string_table();
        let mut stylesheet = setup_stylesheet();

        let result = write(
            1,
            &worksheet,
            &shared_string_table,
            &mut stylesheet,
            false,
            &mut writer_manager,
        );

        assert!(result.is_ok());
    }
}
