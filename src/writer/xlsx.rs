use std::{
    fs,
    fs::File,
    io,
    io::Read,
    path::Path,
};

use super::driver;
use crate::{
    XlsxError,
    helper::crypt::encrypt,
    structs::{
        Workbook,
        WriterManager,
    },
};

mod chart;
mod comment;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod doc_props_custom;
mod drawing;
mod drawing_rels;
mod embeddings;
mod jsa_project_bin;
mod media;
mod printer_settings;
mod rels;
mod shared_strings;
mod styles;
mod table;
mod theme;
mod vba_project_bin;
mod vml_drawing;
mod vml_drawing_rels;
mod workbook;
mod workbook_rels;
mod worksheet;
mod worksheet_rels;

fn make_buffer(wb: &Workbook, is_light: bool) -> Result<Vec<u8>, XlsxError> {
    let cursor = io::Cursor::new(Vec::new());
    let mut arv = zip::ZipWriter::new(cursor);
    let mut writer_manager = WriterManager::new(&mut arv);
    writer_manager.set_is_light(is_light);

    // Add docProps
    doc_props_app::write(wb, &mut writer_manager)?;
    doc_props_core::write(wb, &mut writer_manager)?;
    doc_props_custom::write(wb, &mut writer_manager)?;
    vba_project_bin::write(wb, &mut writer_manager)?;
    jsa_project_bin::write(wb, &mut writer_manager)?;
    rels::write(wb, &mut writer_manager)?;
    theme::write(wb.theme(), &mut writer_manager)?;

    let shared_string_table = wb.shared_string_table();
    let mut stylesheet = wb.stylesheet().clone();

    // Process each worksheet
    wb.sheet_collection_no_check()
        .iter()
        .enumerate()
        .try_for_each(|(index, worksheet)| {
            let worksheet_no = index + 1;
            if worksheet.is_deserialized() {
                worksheet::write(
                    worksheet_no.try_into().unwrap(),
                    worksheet,
                    &shared_string_table,
                    &mut stylesheet,
                    wb.has_macros(),
                    &mut writer_manager,
                )
            } else {
                worksheet
                    .raw_data_of_worksheet()
                    .write(worksheet_no.try_into().unwrap(), &mut writer_manager)
            }
        })?;

    // Process objects associated with worksheets
    wb.sheet_collection_no_check()
        .iter()
        .enumerate()
        .try_for_each(|(index, worksheet)| {
            let worksheet_no = index + 1;
            if !worksheet.is_deserialized() {
                return Ok(());
            }

            // Add charts
            let chart_no_list: Result<Vec<String>, XlsxError> = worksheet
                .worksheet_drawing()
                .chart_collection()
                .iter()
                .map(|chart| chart::write(chart.chart_space(), wb, &mut writer_manager))
                .collect();

            let chart_no_list = chart_no_list?;

            // Add drawing and its relationships
            let (drawing_no, rel_list) = drawing::write(worksheet, &mut writer_manager)?;
            drawing_rels::write(
                worksheet,
                &drawing_no,
                &chart_no_list,
                &rel_list,
                &mut writer_manager,
            )?;

            // Add vml drawing and its relationships
            let (vml_drawing_no, rel_list) = vml_drawing::write(worksheet, &mut writer_manager)?;
            vml_drawing_rels::write(worksheet, &vml_drawing_no, &rel_list, &mut writer_manager)?;

            // Add comments
            let comment_no = comment::write(worksheet, &mut writer_manager)?;

            // Add ole_object and excel
            let (ole_object_no_list, excel_no_list) =
                embeddings::write(worksheet, &mut writer_manager)?;

            // Add Media
            media::write(worksheet, &mut writer_manager)?;

            // Add printer settings
            let printer_settings_no = worksheet
                .page_setup()
                .object_data()
                .map_or_else(String::new, |_| {
                    printer_settings::write(worksheet, &mut writer_manager).unwrap_or_default()
                });

            // Add tables
            let table_no_list = table::write(worksheet, &mut writer_manager)?;

            // Add worksheet relationships
            worksheet_rels::write(
                worksheet,
                &worksheet_no.to_string(),
                &drawing_no,
                &vml_drawing_no,
                &comment_no,
                &ole_object_no_list,
                &excel_no_list,
                &printer_settings_no,
                &table_no_list,
                &mut writer_manager,
            )
        })?;

    // Finalize file list and add remaining components
    writer_manager.file_list_sort();
    shared_strings::write(&shared_string_table, &mut writer_manager)?;
    styles::write(&stylesheet, &mut writer_manager)?;
    workbook::write(wb, &mut writer_manager)?;

    let has_shared_string_table = shared_string_table.read().unwrap().has_value();
    workbook_rels::write(wb, has_shared_string_table, &mut writer_manager)?;
    content_types::write(wb, &mut writer_manager)?;

    arv.finish().map(io::Cursor::into_inner).map_err(Into::into)
}

/// write spreadsheet file to arbitrary writer.
/// # Arguments
/// * `wb` - Workbook structs object.
/// * `writer` - writer to write to.
/// # Return value
/// * `Result` - OK is void. Err is error message.
#[inline]
pub fn write_writer<W: io::Write>(wb: &Workbook, mut writer: W) -> Result<(), XlsxError> {
    let buffer = make_buffer(wb, false)?;
    writer.write_all(&buffer)?;
    Ok(())
}

/// write spreadsheet file to arbitrary writer.
/// # Arguments
/// * `wb` - Workbook structs object.
/// * `writer` - writer to write to.
/// # Return value
/// * `Result` - OK is void. Err is error message.
#[inline]
pub fn write_writer_light<W: io::Write>(wb: &Workbook, mut writer: W) -> Result<(), XlsxError> {
    let buffer = make_buffer(wb, true)?;
    writer.write_all(&buffer)?;
    Ok(())
}

/// write spreadsheet file.
/// # Arguments
/// * `wb` - Workbook structs object.
/// * `path` - file path to save.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let _unused = umya_spreadsheet::writer::xlsx::write(&book, path);
/// ```
pub fn write<P: AsRef<Path>>(wb: &Workbook, path: P) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    if let Err(v) = write_writer(wb, &mut io::BufWriter::new(File::create(&path_tmp)?)) {
        fs::remove_file(path_tmp)?;
        return Err(v);
    }
    fs::rename(path_tmp, path)?;
    Ok(())
}

/// write spreadsheet file.
/// # Arguments
/// * `wb` - Workbook structs object.
/// * `path` - file path to save.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let _unused = umya_spreadsheet::writer::xlsx::write_light(&book, path);
/// ```
pub fn write_light<P: AsRef<Path>>(wb: &Workbook, path: P) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    if let Err(v) = write_writer_light(wb, &mut io::BufWriter::new(File::create(&path_tmp)?)) {
        fs::remove_file(path_tmp)?;
        return Err(v);
    }
    fs::rename(path_tmp, path)?;
    Ok(())
}

/// write spreadsheet file with password.
/// # Arguments
/// * `wb` - Workbook structs object.
/// * `path` - file path to save.
/// * `password` - password.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz_password.xlsx");
/// let _unused = umya_spreadsheet::writer::xlsx::write_with_password(&book, path, "password");
/// ```
pub fn write_with_password<P: AsRef<Path>>(
    wb: &Workbook,
    path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    let buffer = match make_buffer(wb, false) {
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
/// * `wb` - Workbook structs object.
/// * `path` - file path to save.
/// * `password` - password.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz_password.xlsx");
/// let _unused =
///     umya_spreadsheet::writer::xlsx::write_with_password_light(&book, path, "password");
/// ```
pub fn write_with_password_light<P: AsRef<Path>>(
    wb: &Workbook,
    path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    let buffer = match make_buffer(wb, true) {
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
/// let _unused = umya_spreadsheet::writer::xlsx::set_password(&from_path, &to_path, "password");
/// ```
pub fn set_password<P: AsRef<Path>>(
    from_path: P,
    to_path: P,
    password: &str,
) -> Result<(), XlsxError> {
    let mut file = File::open(from_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // set password
    encrypt(&to_path, &buffer, password);

    Ok(())
}
