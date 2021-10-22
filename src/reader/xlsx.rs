use tempdir::TempDir;
use std::path::Path;
use std::io;
use std::string::FromUtf8Error;
use std::fs::File;

use ::structs::Spreadsheet;
use super::driver;

mod doc_props_app;
mod doc_props_core;
mod workbook;
mod workbook_rels;
mod worksheet;
mod rels;
mod theme;
mod shared_strings;
mod styles;
mod worksheet_rels;
mod vml_drawing;
mod drawing;
mod vba_project_bin;
mod comment;
pub(crate) mod chart;
pub(crate) mod drawing_rels;
pub(crate) mod media;

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
pub fn read(path: &Path)->Result<Spreadsheet, XlsxError> {
    let file = File::open(path)?;
    let dir = TempDir::new("shreadsheet")?;
    match driver::unzip(&file, &dir) {
        Ok(_) => {},
        Err(err) => {
            dir.close()?;
            return Err(XlsxError::Zip(err));
        }
    }

    let (mut book, sheets) = workbook::read(&dir).unwrap();
    doc_props_app::read(&dir, &mut book).unwrap();
    doc_props_core::read(&dir, &mut book).unwrap(); 
    vba_project_bin::read(&dir, &mut book).unwrap();
    let workbook_rel = workbook_rels::read(&dir).unwrap();

    for (_, type_value, rel_target) in &workbook_rel {
        match type_value.as_str() {
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
                let theme = theme::read(&dir, rel_target).unwrap();
                book.set_theme(theme);
            },
            _ => {}
        }
    }
    let theme = book.get_theme().clone();

    shared_strings::read(&dir, &mut book).unwrap();
    styles::read(&dir, &mut book).unwrap();

    let mut sheet_count = 0;
    for (sheets_name, sheets_sheet_id, sheets_rid) in &sheets {
        for (rel_id, _, rel_target) in &workbook_rel {
            if sheets_rid == rel_id {
                let (drawing_id, _legacy_drawing_id, hyperlink_vec) = worksheet::read(&dir, &rel_target, &mut book, sheets_sheet_id, sheets_name).unwrap();
                let worksheet = book.get_sheet_by_sheet_id_mut(sheets_sheet_id).unwrap();
                let worksheet_rel = worksheet_rels::read(&dir, &rel_target, &hyperlink_vec, worksheet).unwrap();
                for (_worksheet_id, type_value, worksheet_target) in &worksheet_rel {
                    match type_value.as_str() {
                        // drawing, chart
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
                            drawing::read(&dir, &worksheet_target, worksheet).unwrap();
                        },
                        // comment
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
                            let vml_drawing_target = get_vml_drawing_target(&worksheet_rel);
                            let mut vml_drawing_list = vml_drawing::read(&dir, vml_drawing_target).unwrap();
                            let _ = comment::read(&dir, &worksheet_target, worksheet, &mut vml_drawing_list, &theme).unwrap();
                        },
                        _ => {}
                    }
                }
            }
        }
        sheet_count += 1;
    }

    book.remove_shared_string_table();
    book.remove_stylesheet();

    dir.close()?;
    Ok(book)
}

fn get_vml_drawing_target(worksheet_rel: &Vec<(String, String, String)>) -> &str
{
    for (_, type_value, worksheet_target) in worksheet_rel {
        if type_value == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing" {
            return worksheet_target;
        }
    }
    ""
}
