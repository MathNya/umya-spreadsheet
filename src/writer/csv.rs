use std::fs;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use std::fmt::Write;
use structs::CsvEncodeValues;
use structs::CsvWriterOption;
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
    option: &CsvWriterOption,
) -> Result<(), XlsxError> {
    // get worksheet.
    let worksheet = spreadsheet.get_active_sheet();

    // get max column and row.
    let (max_column, max_row) = worksheet.get_highest_column_and_row();

    let mut data = String::from("");
    for row in 0u32..max_row {
        let mut row_vec: Vec<String> = Vec::new();
        for column in 0u32..max_column {
            // get value.
            let mut value = match worksheet.get_cell((column + 1, row + 1)) {
                Some(cell) => cell.get_cell_value().get_value().into(),
                None => String::from(""),
            };
            // do trim.
            if option.get_do_trim() == &true {
                value = value.trim().to_string();
            }
            // wrap_with_char.
            if option.get_wrap_with_char() != "" {
                value = format! {"{}{}{}", option.get_wrap_with_char(), value, option.get_wrap_with_char()};
            }
            row_vec.push(value);
        }
        write!(data, "{}", row_vec.join(",")).unwrap();
        write!(data, "\r\n").unwrap();
    }

    // encoding.
    let res_into: Vec<u8>;
    let data_bytes = match option.get_csv_encode_value() {
        &CsvEncodeValues::ShiftJis => {
            let (res, _, _) = encoding_rs::SHIFT_JIS.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Koi8u => {
            let (res, _, _) = encoding_rs::KOI8_U.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Koi8r => {
            let (res, _, _) = encoding_rs::KOI8_R.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Iso88598i => {
            let (res, _, _) = encoding_rs::ISO_8859_8_I.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Gbk => {
            let (res, _, _) = encoding_rs::GBK.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::EucKr => {
            let (res, _, _) = encoding_rs::EUC_KR.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Big5 => {
            let (res, _, _) = encoding_rs::BIG5.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Utf16Le => {
            let (res, _, _) = encoding_rs::UTF_16LE.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        &CsvEncodeValues::Utf16Be => {
            let (res, _, _) = encoding_rs::UTF_16BE.encode(&data);
            res_into = res.into_owned();
            &res_into[..]
        }
        _ => data.as_bytes(),
    };

    // output.
    writer.write(data_bytes).unwrap();
    Ok(())
}

/// write spreadsheet file.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `path` - file path to save.
/// * `option` - options.
/// # Return value
/// * `Result` - OK is void. Err is error message.
/// # Examples
/// ```
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let mut option = structs::CsvWriterOption::default();
/// option.set_csv_encode_value(structs::CsvEncodeValues::ShiftJis);
/// option.set_do_trim(true);
/// option.set_wrap_with_char("\"");
/// let _ = writer::csv::write(&book, path, Some(&option));
/// ```
pub fn write<P: AsRef<Path>>(
    spreadsheet: &Spreadsheet,
    path: P,
    option: Option<&CsvWriterOption>,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path
        .as_ref()
        .with_extension(format!("{}{}", extension, "tmp"));
    let def_option = CsvWriterOption::default();
    let option = match option {
        Some(v) => v,
        None => &def_option,
    };
    match write_writer(
        spreadsheet,
        &mut io::BufWriter::new(fs::File::create(path_tmp.as_ref() as &Path)?),
        option,
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
