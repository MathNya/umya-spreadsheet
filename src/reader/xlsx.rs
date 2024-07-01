use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::sync::RwLock;

use super::driver;
use helper::const_str::*;
use structs::drawing::Theme;
use structs::raw::RawWorksheet;
use structs::SharedStringTable;
use structs::Spreadsheet;
use structs::Stylesheet;
use structs::Worksheet;
use XlsxError;

pub(crate) mod chart;
pub(crate) mod comment;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod doc_props_custom;
pub(crate) mod drawing;
mod rels;
mod shared_strings;
mod styles;
pub(crate) mod table;
mod theme;
mod vba_project_bin;
pub(crate) mod vml_drawing;
mod workbook;
mod workbook_rels;
pub(crate) mod worksheet;

/// read spreadsheet from arbitrary reader.
/// # Arguments
/// * `reader` - reader to read from.
/// # Return value
/// * `Result` - OK is Spreadsheet. Err is error message.
pub fn read_reader<R: io::Read + io::Seek>(
    reader: R,
    with_sheet_read: bool,
) -> Result<Spreadsheet, XlsxError> {
    let mut arv = zip::read::ZipArchive::new(reader)?;

    let mut book = workbook::read(&mut arv)?;
    doc_props_app::read(&mut arv, &mut book)?;
    doc_props_core::read(&mut arv, &mut book)?;
    doc_props_custom::read(&mut arv, &mut book)?;
    vba_project_bin::read(&mut arv, &mut book)?;
    content_types::read(&mut arv, &mut book)?;
    let workbook_rel = workbook_rels::read(&mut arv, &mut book)?;

    book.set_theme(Theme::get_default_value());
    for (_, type_value, rel_target) in &workbook_rel {
        if type_value == THEME_NS {
            let theme = theme::read(&mut arv, rel_target)?;
            book.set_theme(theme);
        }
    }

    shared_strings::read(&mut arv, &mut book)?;
    styles::read(&mut arv, &mut book)?;

    for sheet in book.get_sheet_collection_mut() {
        for (rel_id, _, rel_target) in &workbook_rel {
            if sheet.get_r_id() != rel_id {
                continue;
            }
            let mut raw_worksheet = RawWorksheet::default();
            raw_worksheet.read(&mut arv, rel_target);
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
/// * `Result` - OK is Spreadsheet. Err is error message.
/// # Examples
/// ```
/// let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
/// let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> Result<Spreadsheet, XlsxError> {
    let file = File::open(path)?;
    read_reader(file, true)
}

/// lazy read spreadsheet file.
/// Delays the loading of the worksheet until it is needed.
/// When loading a file with a large amount of data, response improvement can be expected.
/// # Arguments
/// * `path` - file path to read.
/// # Return value
/// * `Result` - OK is Spreadsheet. Err is error message.
/// # Examples
/// ```
/// let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
/// let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
/// ```
pub fn lazy_read(path: &Path) -> Result<Spreadsheet, XlsxError> {
    let file = File::open(path)?;
    read_reader(file, false)
}

pub(crate) fn raw_to_deserialize_by_worksheet(
    worksheet: &mut Worksheet,
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    stylesheet: &Stylesheet,
) {
    if worksheet.is_deserialized() {
        return;
    }

    let raw_data_of_worksheet = worksheet.get_raw_data_of_worksheet().clone();
    let shared_string_table = &*shared_string_table.read().unwrap();
    worksheet::read(
        worksheet,
        &raw_data_of_worksheet,
        shared_string_table,
        stylesheet,
    )
    .unwrap();

    if let Some(v) = raw_data_of_worksheet.get_worksheet_relationships() {
        for relationship in v.get_relationship_list() {
            match relationship.get_type() {
                // drawing, chart
                DRAWINGS_NS => {
                    drawing::read(
                        worksheet,
                        relationship.get_raw_file(),
                        raw_data_of_worksheet.get_drawing_relationships(),
                    )
                    .unwrap();
                }
                // comment
                COMMENTS_NS => {
                    comment::read(worksheet, relationship.get_raw_file()).unwrap();
                }
                // table
                TABLE_NS => {
                    table::read(worksheet, relationship.get_raw_file()).unwrap();
                }
                _ => {}
            }
        }
        for relationship in v.get_relationship_list() {
            // vmlDrawing
            if relationship.get_type() == VML_DRAWING_NS {
                vml_drawing::read(
                    worksheet,
                    relationship.get_raw_file(),
                    raw_data_of_worksheet.get_vml_drawing_relationships(),
                )
                .unwrap();
            }
        }
    }

    worksheet.remove_raw_data_of_worksheet();
}
