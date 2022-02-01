use std::fs::File;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use super::driver;
use structs::Spreadsheet;

pub(crate) mod chart;
mod comment;
mod doc_props_app;
mod doc_props_core;
mod drawing;
pub(crate) mod drawing_rels;
pub(crate) mod embeddings;
pub(crate) mod media;
mod rels;
mod shared_strings;
mod styles;
mod theme;
mod vba_project_bin;
mod vml_drawing;
pub(crate) mod vml_drawing_rels;
mod workbook;
mod workbook_rels;
mod worksheet;
pub(crate) mod worksheet_rels;

#[derive(Debug)]
pub enum XlsxError {
    Io(io::Error),
    Xml(quick_xml::Error),
    Zip(zip::result::ZipError),
    Uft8(FromUtf8Error),
}

impl From<io::Error> for XlsxError {
    fn from(err: io::Error) -> XlsxError {
        XlsxError::Io(err)
    }
}

impl From<quick_xml::Error> for XlsxError {
    fn from(err: quick_xml::Error) -> XlsxError {
        XlsxError::Xml(err)
    }
}

impl From<zip::result::ZipError> for XlsxError {
    fn from(err: zip::result::ZipError) -> XlsxError {
        XlsxError::Zip(err)
    }
}

impl From<FromUtf8Error> for XlsxError {
    fn from(err: FromUtf8Error) -> XlsxError {
        XlsxError::Uft8(err)
    }
}

/// read spreadsheet from arbitrary reader.
/// # Arguments
/// * `reader` - reader to read from.
/// # Return value
/// * `Result` - OK is Spreadsheet. Err is error message.
pub fn read_reader<R: io::Read + io::Seek>(reader: R) -> Result<Spreadsheet, XlsxError> {
    let mut arv = zip::read::ZipArchive::new(reader)?;

    let (mut book, sheets, defined_names) = workbook::read(&mut arv).unwrap();
    doc_props_app::read(&mut arv, &mut book).unwrap();
    doc_props_core::read(&mut arv, &mut book).unwrap();
    vba_project_bin::read(&mut arv, &mut book).unwrap();
    let workbook_rel = workbook_rels::read(&mut arv).unwrap();

    for (_, type_value, rel_target) in &workbook_rel {
        match type_value.as_str() {
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
                let theme = theme::read(&mut arv, rel_target).unwrap();
                book.set_theme(theme);
            }
            _ => {}
        }
    }
    let theme = book.get_theme().clone();

    shared_strings::read(&mut arv, &mut book).unwrap();
    styles::read(&mut arv, &mut book).unwrap();

    for (sheets_name, sheets_sheet_id, sheets_rid) in &sheets {
        for (rel_id, _, rel_target) in &workbook_rel {
            if sheets_rid == rel_id {
                let (_drawing_id, legacy_drawing_id, hyperlink_vec) = worksheet::read(
                    &mut arv,
                    &rel_target,
                    &mut book,
                    sheets_sheet_id,
                    sheets_name,
                )
                .unwrap();
                let worksheet = book.get_sheet_by_sheet_id_mut(sheets_sheet_id).unwrap();
                let worksheet_rel =
                    worksheet_rels::read(&mut arv, &rel_target, &hyperlink_vec, worksheet).unwrap();
                for (_worksheet_id, type_value, worksheet_target) in &worksheet_rel {
                    match type_value.as_str() {
                        // drawing, chart
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
                            drawing::read(&mut arv, &worksheet_target, worksheet).unwrap();
                        },
                        // comment
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
                            let _ = comment::read(&mut arv, &worksheet_target, worksheet, &theme).unwrap();
                        },
                        _ => {}
                    }
                }
                for (worksheet_id, _type_value, worksheet_target) in &worksheet_rel {
                    match &legacy_drawing_id {
                        Some(v) => {
                            if v == worksheet_id {
                                let _ = vml_drawing::read(&mut arv, worksheet_target, worksheet)
                                    .unwrap();
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }

    for sheet in book.get_sheet_collection_mut() {
        for defined_name in &defined_names {
            let def_sheet_name = defined_name.get_address_obj().get_sheet_name();
            if sheet.get_title() == def_sheet_name {
                sheet.add_defined_names(defined_name.clone());
            }
        }
    }

    book.remove_shared_string_table();
    book.remove_stylesheet();

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
pub fn read(path: &Path) -> Result<Spreadsheet, XlsxError> {
    let file = File::open(path)?;
    read_reader(file)
}

fn _get_vml_drawing_target(worksheet_rel: &Vec<(String, String, String)>) -> &str {
    for (_, type_value, worksheet_target) in worksheet_rel {
        if type_value
            == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
        {
            return worksheet_target;
        }
    }
    ""
}
