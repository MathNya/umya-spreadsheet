use std::{
    fs::File,
    io,
    path::Path,
    sync::RwLock,
};

use super::driver;
use crate::{
    XlsxError,
    helper::const_str::{
        COMMENTS_NS,
        DRAWINGS_NS,
        PIVOT_TABLE_NS,
        TABLE_NS,
        THEME_NS,
        THREADED_COMMENT_NS,
        VML_DRAWING_NS,
    },
    structs::{
        Cell,
        SharedStringTable,
        Stylesheet,
        Workbook,
        Worksheet,
        drawing::Theme,
        raw::RawWorksheet,
    },
};

pub(crate) mod chart;
pub(crate) mod comment;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod doc_props_custom;
pub(crate) mod drawing;
mod jsa_project_bin;
mod pivot_cache;
mod pivot_table;
mod rels;
mod shared_strings;
mod styles;
pub(crate) mod table;
mod theme;
pub(crate) mod threaded_comment;
mod vba_project_bin;
pub(crate) mod vml_drawing;
mod workbook;
mod workbook_rels;
pub(crate) mod worksheet;

/// read spreadsheet from arbitrary reader.
/// # Arguments
/// * `reader` - reader to read from.
/// # Return value
/// * `Result` - OK is `Workbook`. Err is error message.
pub fn read_reader<R: io::Read + io::Seek>(
    reader: R,
    with_sheet_read: bool,
) -> Result<Workbook, XlsxError> {
    read_reader_with_source(reader, with_sheet_read, None)
}

fn read_reader_with_source<R: io::Read + io::Seek>(
    reader: R,
    with_sheet_read: bool,
    source_file: Option<&Path>,
) -> Result<Workbook, XlsxError> {
    let mut arv = zip::read::ZipArchive::new(reader)?;

    let mut book = workbook::read(&mut arv)?;
    doc_props_app::read(&mut arv, &mut book)?;
    doc_props_core::read(&mut arv, &mut book)?;
    doc_props_custom::read(&mut arv, &mut book)?;
    vba_project_bin::read(&mut arv, &mut book)?;
    jsa_project_bin::read(&mut arv, &mut book)?;
    content_types::read(&mut arv, &mut book)?;
    let workbook_rel = workbook_rels::read(&mut arv, &mut book)?;

    book.set_theme(Theme::default_value());
    for (_, type_value, rel_target) in &workbook_rel {
        if type_value == THEME_NS {
            let theme = theme::read(&mut arv, rel_target)?;
            book.set_theme(theme);
        }
    }

    shared_strings::read(&mut arv, &mut book)?;
    styles::read(&mut arv, &mut book)?;

    for sheet in book.sheet_collection_mut() {
        for (rel_id, _, rel_target) in &workbook_rel {
            if sheet.r_id() != rel_id {
                continue;
            }
            let mut raw_worksheet = RawWorksheet::default();
            match source_file {
                Some(source_file) => raw_worksheet.read_lazy(&mut arv, rel_target, source_file),
                None => raw_worksheet.read(&mut arv, rel_target),
            }
            sheet.set_raw_data_of_worksheet(raw_worksheet);
        }
    }

    if with_sheet_read {
        book.read_sheet_collection();
    }

    Ok(book)
}

/// read spreadsheet file.
/// # Arguments
/// * `path` - file path to read.
/// # Return value
/// * `Result` - OK is Workbook. Err is error message.
/// # Examples
/// ```
/// let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
/// let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
/// ```
#[inline]
pub fn read<P: AsRef<Path>>(path: P) -> Result<Workbook, XlsxError> {
    let file = File::open(path)?;
    read_reader(file, true)
}

/// lazy read spreadsheet file.
/// Delays the loading of the worksheet until it is needed.
/// When loading a file with a large amount of data, response improvement can be
/// expected. # Arguments
/// * `path` - file path to read.
/// # Return value
/// * `Result` - OK is Workbook. Err is error message.
/// # Examples
/// ```
/// let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
/// let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
/// ```
#[inline]
pub fn lazy_read(path: &Path) -> Result<Workbook, XlsxError> {
    let file = File::open(path)?;
    read_reader_with_source(file, false, Some(path))
}

/// Stream cells from a worksheet without deserializing the worksheet into
/// memory.
///
/// This is intended for very large sheets where building a full [`Worksheet`]
/// would allocate too much memory. The callback receives one parsed [`Cell`] at
/// a time; the cell is dropped before the next cell is parsed.
pub fn read_sheet_by_name_stream<P, F>(
    path: P,
    sheet_name: &str,
    callback: F,
) -> Result<(), XlsxError>
where
    P: AsRef<Path>,
    F: FnMut(&Cell),
{
    let book = lazy_read(path.as_ref())?;
    let Some(worksheet) = book
        .sheet_collection_no_check()
        .iter()
        .find(|worksheet| worksheet.name() == sheet_name)
    else {
        return Err(XlsxError::CellError(format!(
            "Worksheet '{sheet_name}' not found."
        )));
    };
    let shared_string_table = book.shared_string_table();
    let shared_string_table = &*shared_string_table.read().unwrap();
    worksheet::read_cells_stream(
        worksheet.raw_data_of_worksheet(),
        shared_string_table,
        book.stylesheet(),
        callback,
    )
}

pub(crate) fn raw_to_deserialize_by_worksheet(
    worksheet: &mut Worksheet,
    shared_string_table: &RwLock<SharedStringTable>,
    stylesheet: &Stylesheet,
) {
    if worksheet.is_deserialized() {
        return;
    }

    let mut raw_data_of_worksheet = worksheet.take_raw_data_of_worksheet();
    let shared_string_table = &*shared_string_table.read().unwrap();
    worksheet::read(
        worksheet,
        &raw_data_of_worksheet,
        shared_string_table,
        stylesheet,
    )
    .unwrap();
    raw_data_of_worksheet
        .load_relationship_file_data_from_source()
        .unwrap();

    if let Some(v) = raw_data_of_worksheet.worksheet_relationships() {
        for relationship in v.relationship_list() {
            match relationship.get_type() {
                // drawing, chart
                DRAWINGS_NS => {
                    drawing::read(
                        worksheet,
                        relationship.raw_file(),
                        raw_data_of_worksheet.drawing_relationships(),
                    );
                }
                // comment
                COMMENTS_NS => {
                    comment::read(worksheet, relationship.raw_file());
                }
                // threaded_comment
                THREADED_COMMENT_NS => {
                    threaded_comment::read(worksheet, relationship.raw_file());
                }
                // table
                TABLE_NS => {
                    table::read(worksheet, relationship.raw_file()).unwrap();
                }
                // pivot table, pivot cache
                PIVOT_TABLE_NS => {
                    pivot_table::read(
                        worksheet,
                        relationship.raw_file(),
                        raw_data_of_worksheet.pivot_table_relationships(),
                    );
                }
                _ => {}
            }
        }
        for relationship in v.relationship_list() {
            // vmlDrawing
            if relationship.get_type() == VML_DRAWING_NS {
                vml_drawing::read(
                    worksheet,
                    relationship.raw_file(),
                    raw_data_of_worksheet.vml_drawing_relationships(),
                );
            }
        }
    }
}
