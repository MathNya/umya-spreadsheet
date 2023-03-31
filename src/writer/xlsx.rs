use super::driver;
use helper::crypt::*;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::string::FromUtf8Error;
use structs::Spreadsheet;
use structs::WriterManager;

mod chart;
mod comment;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod drawing;
mod drawing_rels;
mod embeddings;
mod media;
mod printer_settings;
mod rels;
mod shared_strings;
mod styles;
mod theme;
mod vba_project_bin;
mod vml_drawing;
mod vml_drawing_rels;
mod workbook;
mod workbook_rels;
mod worksheet;
mod worksheet_rels;

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

impl fmt::Display for XlsxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::XlsxError::*;
        match self {
            Io(i) => write!(f, "IoError: {}", i),
            Xml(s) => write!(f, "XmlError: {}", s),
            Zip(s) => write!(f, "ZipError: {}", s),
            Uft8(s) => write!(f, "Uft8Error: {}", s),
        }
    }
}

impl Error for XlsxError {}

fn make_buffer(spreadsheet: &Spreadsheet, is_light: bool) -> Result<std::vec::Vec<u8>, XlsxError> {
    let arv = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let mut writer_manager = WriterManager::new(arv);
    writer_manager.set_is_light(is_light);

    // Add docProps App
    doc_props_app::write(spreadsheet, &mut writer_manager)?;

    // Add docProps Core
    doc_props_core::write(spreadsheet, &mut writer_manager)?;

    // Add vbaProject.bin
    vba_project_bin::write(spreadsheet, &mut writer_manager)?;

    // Add relationships
    rels::write(spreadsheet, &mut writer_manager)?;

    // Add theme
    theme::write(spreadsheet.get_theme(), &mut writer_manager)?;

    // worksheet
    let shared_string_table = spreadsheet.get_shared_string_table();
    let mut stylesheet = spreadsheet.get_stylesheet().clone();
    let mut worksheet_no = 1;
    for worksheet in spreadsheet.get_sheet_collection_no_check() {
        match worksheet.is_deserialized() {
            false => {
                // from no deserialized.
                worksheet
                    .get_raw_data_of_worksheet()
                    .write(&worksheet_no, &mut writer_manager)?;
            }
            true => {
                // from deserialized.
                worksheet::write(
                    &worksheet_no,
                    worksheet,
                    shared_string_table.clone(),
                    &mut stylesheet,
                    spreadsheet.get_has_macros(),
                    &mut writer_manager,
                )?;
            }
        }
        worksheet_no += 1;
    }

    // Objects associated with worksheets
    let mut worksheet_no: i32 = 1;
    for worksheet in spreadsheet.get_sheet_collection_no_check() {
        match worksheet.is_deserialized() {
            false => {
                // from no deserialized.
            }
            true => {
                // from deserialized.
                // Add charts
                let mut chart_no_list: Vec<String> = Vec::new();
                for chart in worksheet.get_worksheet_drawing().get_chart_collection() {
                    let chart_space = chart.get_chart_space();
                    let chart_no = chart::write(chart_space, spreadsheet, &mut writer_manager)?;
                    chart_no_list.push(chart_no);
                }

                // Add drawing
                let drawing_no = drawing::write(worksheet, &mut writer_manager)?;

                // Add drawing rels
                drawing_rels::write(worksheet, &drawing_no, &chart_no_list, &mut writer_manager)?;

                // Add vml drawing
                let vml_drawing_no = vml_drawing::write(worksheet, &mut writer_manager)?;

                // Add vml drawing rels
                vml_drawing_rels::write(worksheet, &vml_drawing_no, &mut writer_manager)?;

                // Add comment
                let comment_no = comment::write(worksheet, &mut writer_manager)?;

                // Add ole_object and excel
                let (ole_object_no_list, excel_no_list) =
                    embeddings::write(worksheet, &mut writer_manager)?;

                // Add Media
                media::write(worksheet, &mut writer_manager)?;

                // Add printer_settings
                let printer_settings_no = match worksheet.get_page_setup().get_object_data() {
                    Some(_) => printer_settings::write(worksheet, &mut writer_manager)?,
                    None => String::from(""),
                };

                // Add worksheet rels
                worksheet_rels::write(
                    worksheet,
                    &worksheet_no.to_string(),
                    &drawing_no,
                    &vml_drawing_no,
                    &comment_no,
                    &ole_object_no_list,
                    &excel_no_list,
                    &printer_settings_no,
                    &mut writer_manager,
                )?;
            }
        }
        worksheet_no += 1;
    }

    // file list sort
    writer_manager.file_list_sort();

    // Add SharedStrings
    shared_strings::write(shared_string_table.clone(), &mut writer_manager)?;

    // Add Styles
    styles::write(&stylesheet, &mut writer_manager)?;

    // Add workbook
    workbook::write(spreadsheet, &mut writer_manager)?;

    // Add workbook relationships
    let has_shared_string_table = shared_string_table.read().unwrap().has_value();
    workbook_rels::write(spreadsheet, has_shared_string_table, &mut writer_manager)?;

    // Add Content_Types
    content_types::write(spreadsheet, &mut writer_manager)?;

    let result = writer_manager.get_arv_mut().finish()?;
    Ok(result.into_inner())
}

/// write spreadsheet file to arbitrary writer.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `writer` - writer to write to.
/// # Return value
/// * `Result` - OK is void. Err is error message.
pub fn write_writer<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer: W,
) -> Result<(), XlsxError> {
    let buffer = make_buffer(spreadsheet, false)?;
    let mut writer = writer;
    writer.write_all(&buffer)?;
    Ok(())
}

/// write spreadsheet file to arbitrary writer.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `writer` - writer to write to.
/// # Return value
/// * `Result` - OK is void. Err is error message.
pub fn write_writer_light<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer: W,
) -> Result<(), XlsxError> {
    let buffer = make_buffer(spreadsheet, true)?;
    let mut writer = writer;
    writer.write_all(&buffer)?;
    Ok(())
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
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
/// ```
pub fn write<P: AsRef<Path>>(spreadsheet: &Spreadsheet, path: P) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    match write_writer(
        spreadsheet,
        &mut io::BufWriter::new(fs::File::create(&path_tmp)?),
    ) {
        Ok(_) => {}
        Err(v) => {
            fs::remove_file(path_tmp)?;
            return Err(v);
        }
    }
    fs::rename(path_tmp, path)?;
    Ok(())
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
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write_light(&book, path);
/// ```
pub fn write_light<P: AsRef<Path>>(spreadsheet: &Spreadsheet, path: P) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    match write_writer_light(
        spreadsheet,
        &mut io::BufWriter::new(fs::File::create(&path_tmp)?),
    ) {
        Ok(_) => {}
        Err(v) => {
            fs::remove_file(path_tmp)?;
            return Err(v);
        }
    }
    fs::rename(path_tmp, path)?;
    Ok(())
}

/// write spreadsheet file with password.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `path` - file path to save.
/// * `password` - password.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz_password.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write_with_password(&book, path, "password");
/// ```
pub fn write_with_password<P: AsRef<Path>>(
    spreadsheet: &Spreadsheet,
    path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    let buffer = match make_buffer(spreadsheet, false) {
        Ok(v) => v,
        Err(v) => {
            fs::remove_file(path_tmp)?;
            return Err(v);
        }
    };

    // set password
    encrypt(&path_tmp, &buffer, password);

    fs::rename(path_tmp, path)?;
    Ok(())
}

/// write spreadsheet file with password.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `path` - file path to save.
/// * `password` - password.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz_password.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write_with_password_light(&book, path, "password");
/// ```
pub fn write_with_password_light<P: AsRef<Path>>(
    spreadsheet: &Spreadsheet,
    path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    let buffer = match make_buffer(spreadsheet, true) {
        Ok(v) => v,
        Err(v) => {
            fs::remove_file(path_tmp)?;
            return Err(v);
        }
    };

    // set password
    encrypt(&path_tmp, &buffer, password);

    fs::rename(path_tmp, path)?;
    Ok(())
}

/// write spreadsheet file with password.
/// # Arguments
/// * `from_path` - file path from readfile.
/// * `to_path` - file path to save.
/// * `password` - password.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let from_path = std::path::Path::new("./tests/test_files/aaa.xlsx");
/// let to_path = std::path::Path::new("./tests/result_files/zzz_password2.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::set_password(&from_path, &to_path, "password");
/// ```
pub fn set_password<P: AsRef<Path>>(
    from_path: P,
    to_path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let mut file = File::open(from_path).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    // set password
    encrypt(&to_path, &buffer, password);

    Ok(())
}
