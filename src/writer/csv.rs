use std::fs;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use structs::Spreadsheet;

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

/// write spreadsheet file to arbitrary writer.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `writer` - writer to write to.
/// # Return value
/// * `Result` - OK is void. Err is error message.
pub fn write_writer<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer: &mut W,
) -> Result<(), XlsxError> {
    // get worksheet.
    let worksheet = spreadsheet.get_active_sheet().unwrap();

    // get max column and row.
    let highest = worksheet.get_highest_row_and_column();
    let max_column = &highest["column"];
    let max_row = &highest["row"];

    let mut data = String::from("");
    for row in 1u32..max_row.clone() {
        let mut row_vec: Vec<&str> = Vec::new();
        for column in 1u32..max_column.clone() {
            // get value.
            let value = match worksheet.get_cell_by_column_and_row(column, row) {
                Some(cell) => cell.get_cell_value().get_value(),
                None => "",
            };
            row_vec.push(value);
        }
        data += row_vec.join(",").as_str();
        data += "\r\n";
    }

    // encording.
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&data.as_bytes());
    data = res.into_owned();

    // output.
    writer.write(data.as_bytes()).unwrap();
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
/// let _ = umya_spreadsheet::writer::csv::write(&book, path);
/// ```
pub fn write(spreadsheet: &Spreadsheet, path: &Path) -> Result<(), XlsxError> {
    let path_tmp = format!("{}.tmp", path.to_str().unwrap());
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
    fs::rename(path_tmp, path.to_str().unwrap())?;
    Ok(())
}
