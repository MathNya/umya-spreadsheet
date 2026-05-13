use std::{
    collections::HashMap,
    fs::File,
    io,
};

use quick_xml::{
    Reader,
    escape,
    events::Event,
};

use super::{
    XlsxError,
    driver::{
        get_attribute,
        get_attribute_value,
        xml_read_loop,
        zip_by_name,
    },
};
use crate::{
    helper::formula::FormulaToken,
    structs::{
        Cell,
        Cells,
        Columns,
        ConditionalFormatting,
        DataValidations,
        Hyperlink,
        OleObjects,
        Row,
        SharedStringTable,
        SheetProtection,
        Stylesheet,
        Worksheet,
        office2010::excel::DataValidations as DataValidations2010,
        raw::{
            RawRelationships,
            RawWorksheet,
        },
    },
};

pub(crate) fn read(
    worksheet: &mut Worksheet,
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Result<(), XlsxError> {
    if let Some(source_file) = raw_data_of_worksheet.worksheet_file().source_file() {
        if !raw_data_of_worksheet.worksheet_file().has_file_data() {
            let file = File::open(source_file)?;
            let mut archive = zip::read::ZipArchive::new(file)?;
            let source = zip_by_name(
                &mut archive,
                raw_data_of_worksheet.worksheet_file().file_target(),
            )?;
            let reader = io::BufReader::new(source);
            return read_from_reader(
                worksheet,
                reader,
                raw_data_of_worksheet,
                shared_string_table,
                stylesheet,
            );
        }
    }

    let data = io::Cursor::new(raw_data_of_worksheet.worksheet_file().file_data());
    read_from_reader(
        worksheet,
        data,
        raw_data_of_worksheet,
        shared_string_table,
        stylesheet,
    )
}

fn read_from_reader<R: io::BufRead>(
    worksheet: &mut Worksheet,
    data: R,
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Result<(), XlsxError> {
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);
    let mut formula_shared_list: HashMap<u32, (String, Vec<FormulaToken>)> = HashMap::new();
    xml_read_loop!(
        reader,
        Event::Start(ref e) => match e.name().into_inner() {
            b"sheetPr" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"codeName" => {
                            worksheet.set_code_name(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"sheetViews" => {
                worksheet
                    .sheet_views_mut()
                    .set_attributes(&mut reader, e);
            }
            b"sheetFormatPr" => {
                worksheet
                    .sheet_format_properties_mut()
                    .set_attributes(&mut reader, e);
            }
            b"selection" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"activeCell" => {
                            worksheet.set_active_cell(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"row" => {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    worksheet.cells_crate_mut(),
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    false,
                );
                worksheet.set_row_dimension(obj);
            }
            b"autoFilter" => {
                worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
            }
            b"cols" => {
                let mut obj = Columns::default();
                obj.set_attributes(&mut reader, e, stylesheet);
                worksheet.set_column_dimensions_crate(obj);
            }
            b"mergeCells" => {
                worksheet
                    .merge_cells_crate_mut()
                    .set_attributes(&mut reader, e);
            }
            b"conditionalFormatting" => {
                let mut obj = ConditionalFormatting::default();
                obj.set_attributes(&mut reader, e, stylesheet.differential_formats());
                worksheet.add_conditional_formatting_collection(obj);
            }
            b"dataValidations" => {
                let mut obj = DataValidations::default();
                obj.set_attributes(&mut reader, e);
                worksheet.set_data_validations(obj);
            }
            b"x14:dataValidations" => {
                let mut obj = DataValidations2010::default();
                obj.set_attributes(&mut reader, e);
                worksheet.set_data_validations_2010(obj);
            }
            b"oleObjects" => {
                let mut obj = OleObjects::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    raw_data_of_worksheet.worksheet_relationships().unwrap(),
                );
                worksheet.set_ole_objects(obj);
            }
            b"headerFooter" => {
                worksheet
                    .header_footer_mut()
                    .set_attributes(&mut reader, e);
            }
            b"rowBreaks" => {
                worksheet
                    .row_breaks_mut()
                    .set_attributes(&mut reader, e);
            }
            b"colBreaks" => {
                worksheet
                    .column_breaks_mut()
                    .set_attributes(&mut reader, e);
            }
            _ => (),
        },
        Event::Empty(ref e) => match e.name().into_inner() {
            b"sheetPr" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"codeName" => {
                            worksheet.set_code_name(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"tabColor" => {
                worksheet
                    .tab_color_mut()
                    .set_attributes(&mut reader, e, true);
            }
            b"sheetFormatPr" => {
                worksheet
                    .sheet_format_properties_mut()
                    .set_attributes(&mut reader, e);
            }
            b"selection" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"activeCell" => {
                            worksheet.set_active_cell(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"row" => {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    worksheet.cells_crate_mut(),
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    true,
                );
                worksheet.set_row_dimension(obj);
            }
            b"autoFilter" => {
                worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
            }
            b"pageMargins" => {
                worksheet
                    .page_margins_mut()
                    .set_attributes(&mut reader, e);
            }
            b"hyperlink" => {
                let (coor, hyperlink) = get_hyperlink(
                    e,
                    raw_data_of_worksheet.worksheet_relationships()
                );
                worksheet.cell_mut(coor).set_hyperlink(hyperlink);
            }
            b"printOptions" => {
                worksheet
                    .print_options_mut()
                    .set_attributes(&mut reader, e);
            }
            b"pageSetup" => {
                worksheet.page_setup_mut().set_attributes(
                    &mut reader,
                    e,
                    raw_data_of_worksheet.worksheet_relationships(),
                );
            }
            b"sheetProtection" => {
                let mut obj = SheetProtection::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                );
                worksheet.set_sheet_protection(obj);
            }
            _ => (),
        },
        Event::Eof => break,
    );

    Ok(())
}

pub(crate) fn read_lite(
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Cells {
    if let Some(source_file) = raw_data_of_worksheet.worksheet_file().source_file() {
        if !raw_data_of_worksheet.worksheet_file().has_file_data() {
            let file = File::open(source_file).unwrap();
            let mut archive = zip::read::ZipArchive::new(file).unwrap();
            let source = zip_by_name(
                &mut archive,
                raw_data_of_worksheet.worksheet_file().file_target(),
            )
            .unwrap();
            let reader = io::BufReader::new(source);
            return read_lite_from_reader(reader, shared_string_table, stylesheet);
        }
    }

    let data = io::Cursor::new(raw_data_of_worksheet.worksheet_file().file_data());
    read_lite_from_reader(data, shared_string_table, stylesheet)
}

fn read_lite_from_reader<R: io::BufRead>(
    data: R,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Cells {
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);

    let mut cells = Cells::default();
    let mut formula_shared_list: HashMap<u32, (String, Vec<FormulaToken>)> = HashMap::new();
    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"row" {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    &mut cells,
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    false,
                );
            }
        },
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"row" {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    &mut cells,
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    true,
                );
            }
        },
        Event::Eof => break,
    );

    cells
}

pub(crate) fn read_cells_stream<F>(
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
    callback: F,
) -> Result<(), XlsxError>
where
    F: FnMut(&Cell),
{
    if let Some(source_file) = raw_data_of_worksheet.worksheet_file().source_file() {
        if !raw_data_of_worksheet.worksheet_file().has_file_data() {
            let file = File::open(source_file)?;
            let mut archive = zip::read::ZipArchive::new(file)?;
            let source = zip_by_name(
                &mut archive,
                raw_data_of_worksheet.worksheet_file().file_target(),
            )?;
            let reader = io::BufReader::new(source);
            read_cells_stream_from_reader(reader, shared_string_table, stylesheet, callback);
            return Ok(());
        }
    }

    let data = io::Cursor::new(raw_data_of_worksheet.worksheet_file().file_data());
    read_cells_stream_from_reader(data, shared_string_table, stylesheet, callback);
    Ok(())
}

fn read_cells_stream_from_reader<R, F>(
    data: R,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
    mut callback: F,
) where
    R: io::BufRead,
    F: FnMut(&Cell),
{
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);
    let mut formula_shared_list: HashMap<u32, (String, Vec<FormulaToken>)> = HashMap::new();

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"row" {
                read_row_cells_stream(
                    &mut reader,
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    &mut callback,
                );
            }
        },
        Event::Eof => break,
    );
}

fn read_row_cells_stream<R, F>(
    reader: &mut Reader<R>,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
    formula_shared_list: &mut HashMap<u32, (String, Vec<FormulaToken>)>,
    callback: &mut F,
) where
    R: io::BufRead,
    F: FnMut(&Cell),
{
    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"c" {
                let mut obj = Cell::default();
                obj.set_attributes(
                    reader,
                    e,
                    shared_string_table,
                    stylesheet,
                    true,
                    formula_shared_list,
                );
                callback(&obj);
            }
        },
        Event::Start(ref e) => {
            if e.name().into_inner() == b"c" {
                let mut obj = Cell::default();
                obj.set_attributes(
                    reader,
                    e,
                    shared_string_table,
                    stylesheet,
                    false,
                    formula_shared_list,
                );
                callback(&obj);
            }
        },
        Event::End(ref e) => {
            if e.name().into_inner() == b"row" {
                return
            }
        },
        Event::Eof => panic!("Error: Could not find {} end element", "row")
    );
}

fn get_hyperlink(
    e: &quick_xml::events::BytesStart<'_>,
    raw_relationships: Option<&RawRelationships>,
) -> (String, Hyperlink) {
    let mut hyperlink = Hyperlink::default();

    let coordition = get_attribute(e, b"ref").unwrap_or_default();
    if let Some(v) = get_attribute(e, b"location") {
        hyperlink.set_url(escape::unescape(&v).unwrap().to_string());
        hyperlink.set_location(true);
    }
    if let Some(v) = get_attribute(e, b"tooltip") {
        hyperlink.set_tooltip(escape::unescape(&v).unwrap().to_string());
    }
    if let Some(v) = get_attribute(e, b"r:id") {
        let relationship = raw_relationships.unwrap().relationship_by_rid(&v);
        hyperlink.set_url(escape::unescape(relationship.target()).unwrap().to_string());
    }
    (coordition, hyperlink)
}
