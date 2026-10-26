use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    reader,
    writer,
};
use zip::ZipArchive;

// Strings in SpreadsheetML are ST_Xstring: characters that XML 1.0 forbids are
// written as _xHHHH_, and text that already reads as such an escape has its
// underscore written as _x005F_.
#[test]
fn shared_strings_escape_forbidden_characters_and_literal_escapes() {
    let values = [
        "a\u{1}b\u{b}c\u{1f}d",
        "_x0041_ stays literal",
        "_x005F_ and _x00",
        "nonchar \u{fffe}",
        "tab\tnew line\n",
    ];
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    for (i, value) in values.iter().enumerate() {
        sheet.cell_mut((1, i as u32 + 1)).set_value_string(*value);
    }
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();

    let mut archive = ZipArchive::new(Cursor::new(bytes.clone())).unwrap();
    let mut shared_strings = String::new();
    archive
        .by_name("xl/sharedStrings.xml")
        .unwrap()
        .read_to_string(&mut shared_strings)
        .unwrap();
    assert!(
        shared_strings.contains("a_x0001_b_x000B_c_x001F_d"),
        "{shared_strings}"
    );
    assert!(
        shared_strings.contains("_x005F_x0041_ stays literal"),
        "{shared_strings}"
    );
    assert!(
        shared_strings.contains("_x005F_x005F_ and _x00"),
        "{shared_strings}"
    );
    assert!(
        shared_strings.contains("nonchar _xFFFE_"),
        "{shared_strings}"
    );
    assert!(
        shared_strings.contains("tab\tnew line\n"),
        "{shared_strings}"
    );

    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let sheet = book.sheet(0).unwrap();
    for (i, value) in values.iter().enumerate() {
        assert_eq!(sheet.value((1, i as u32 + 1)), *value);
    }
}

// Excel writes a control character in a string as _xHHHH_.
#[test]
fn shared_string_escapes_are_read_as_their_characters() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value_string("PLACEHOLDER");
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let bytes = replace_in_part(
        &bytes,
        "xl/sharedStrings.xml",
        "PLACEHOLDER",
        "line_x000B_break _x005F_x0041_",
    );

    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    assert_eq!(book.sheet(0).unwrap().value("A1"), "line\u{b}break _x0041_");
}

fn replace_in_part(bytes: &[u8], part: &str, from: &str, to: &str) -> Vec<u8> {
    use std::io::Write;
    let mut source = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..source.len() {
        let mut file = source.by_index(index).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        let name = file.name().unwrap().to_string();
        if name == part {
            content = String::from_utf8(content)
                .unwrap()
                .replace(from, to)
                .into_bytes();
        }
        output
            .start_file(name, zip::write::SimpleFileOptions::default())
            .unwrap();
        output.write_all(&content).unwrap();
    }
    output.finish().unwrap().into_inner()
}
