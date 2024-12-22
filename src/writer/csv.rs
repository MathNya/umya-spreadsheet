use std::fmt::Write;
use std::fs;
use std::io;
use std::path::Path;

use crate::structs::CsvEncodeValues;
use crate::structs::CsvWriterOption;
use crate::structs::Spreadsheet;
use crate::structs::XlsxError;

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

    let mut data = String::new();
    for row in 0u32..max_row {
        let mut row_vec: Vec<String> = Vec::new();
        for column in 0u32..max_column {
            // get value.
            let mut value = match worksheet.get_cell((column + 1, row + 1)) {
                Some(cell) => cell.get_cell_value().get_value().into(),
                None => String::new(),
            };
            // do trim.
            if option.get_do_trim() {
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
    let data_bytes = match *option.get_csv_encode_value() {
        CsvEncodeValues::ShiftJis => encoding_rs::SHIFT_JIS.encode(&data).0.into_owned(),
        CsvEncodeValues::Koi8u => encoding_rs::KOI8_U.encode(&data).0.into_owned(),
        CsvEncodeValues::Koi8r => encoding_rs::KOI8_R.encode(&data).0.into_owned(),
        CsvEncodeValues::Iso88598i => encoding_rs::ISO_8859_8_I.encode(&data).0.into_owned(),
        CsvEncodeValues::Gbk => encoding_rs::GBK.encode(&data).0.into_owned(),
        CsvEncodeValues::EucKr => encoding_rs::EUC_KR.encode(&data).0.into_owned(),
        CsvEncodeValues::Big5 => encoding_rs::BIG5.encode(&data).0.into_owned(),
        CsvEncodeValues::Utf16Le => encoding_rs::UTF_16LE.encode(&data).0.into_owned(),
        CsvEncodeValues::Utf16Be => encoding_rs::UTF_16BE.encode(&data).0.into_owned(),
        CsvEncodeValues::Utf8 => data.into_bytes(),
    };

    // output.
    writer.write_all(&data_bytes).unwrap();
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
/// let _unused = writer::csv::write(&book, path, Some(&option));
/// ```
pub fn write<P: AsRef<Path>>(
    spreadsheet: &Spreadsheet,
    path: P,
    option: Option<&CsvWriterOption>,
) -> Result<(), XlsxError> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap();
    let path_tmp = path.as_ref().with_extension(format!("{}{}", extension, "tmp"));
    let def_option = CsvWriterOption::default();
    let option = match option {
        Some(v) => v,
        None => &def_option,
    };
    if let Err(v) = write_writer(
        spreadsheet,
        &mut io::BufWriter::new(fs::File::create::<&Path>(path_tmp.as_ref())?),
        option,
    ) {
        fs::remove_file(path_tmp)?;
        return Err(v);
    }
    fs::rename(path_tmp, path)?;
    Ok(())
}
