use tempdir::TempDir;
use std::path::Path;
use std::io;
use std::string::FromUtf8Error;

use super::structs::spreadsheet::Spreadsheet;
use super::driver;

mod chart;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod workbook;
mod worksheet;
mod rels;
mod workbook_rels;
mod worksheet_rels;
mod theme;
mod shared_strings;
mod styles;
mod drawing;
mod drawing_rels;
mod vba_project_bin;
mod comment;
mod vml_drawing;

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

/// write spreadsheet file.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `path` - file path to save.
/// # Return value
/// * `Result` - OK is void. Err is error message. 
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("C:/spread_test_data/zzz.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
/// ```
pub fn write(spreadsheet: &Spreadsheet, path: &Path) -> Result<(), XlsxError> {
    let dir = TempDir::new("shreadsheet")?;

    // Add Content_Types
    let _= content_types::write(spreadsheet, &dir, "[Content_Types].xml");

    // Add docProps App
    let _= doc_props_app::write(spreadsheet, &dir, "docProps", "app.xml");

    // Add docProps Core
    let _= doc_props_core::write(spreadsheet, &dir, "docProps", "core.xml");

    // Add vbaProject.bin
    let _= vba_project_bin::write(spreadsheet, &dir, "xl", "vbaProject.bin");

    // Add relationships
    let _ = rels::write(spreadsheet, &dir, "_rels", ".rels");
    let _ = workbook_rels::write(spreadsheet, &dir, "xl/_rels", "workbook.xml.rels");

    // Add theme
    let _ = theme::write(&dir, "xl/theme", "theme1.xml");

    // Add workbook
    let _ = workbook::write(spreadsheet, &dir, "xl", "workbook.xml");

    // Add SharedStrings
    let shared = shared_strings::write(spreadsheet, &dir).unwrap();

    // Add Styles
    let _ = styles::write(spreadsheet, &dir).unwrap();

    // Add worksheets and relationships (drawings, ...)
    let mut chart_id = 1;
    for i in 0..spreadsheet.get_sheet_count() {
        let p_worksheet_id:&str = &(i+1).to_string();
        let worksheet = &spreadsheet.get_sheet_collection()[i];
        let is_selected = spreadsheet.get_active_sheet_index() == &i;
        let has_macros = spreadsheet.get_has_macros();
        let all_cell_xf_list = spreadsheet.get_all_cell_style();
        let conditonal_style_list = spreadsheet.get_all_conditional_style_list();
        let _ = worksheet::write(
            worksheet,
            &(i+1),
            &is_selected,
            has_macros,
            all_cell_xf_list,
            conditonal_style_list,
            shared.clone(),
            &dir
        );
        let _ = worksheet_rels::write(worksheet, p_worksheet_id,  &dir);
        let _ = drawing::write(worksheet, p_worksheet_id, &chart_id, &dir);
        let _ = drawing_rels::write(worksheet, p_worksheet_id, &chart_id, &dir);
        let _ = comment::write(worksheet, p_worksheet_id,  &dir);
        let _ = vml_drawing::write(worksheet, p_worksheet_id,  &dir);

        for ct in worksheet.get_chart_collection(){
            let _ = chart::write(ct, &chart_id.to_string(), &dir);
            chart_id += 1;
        }
    }

    driver::write_to_file(path, &dir)?;
    dir.close()?;
    Ok(())
}
