use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    reader,
    writer,
};
use zip::ZipArchive;

fn workbook_xml(bytes: &[u8]) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/workbook.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn write(book: &umya_spreadsheet::Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

#[test]
fn date1904_defaults_to_false_and_is_not_written() {
    let book = umya_spreadsheet::new_file();
    assert!(!book.date1904());
    assert!(!workbook_xml(&write(&book)).contains("date1904"));
}

#[test]
fn date1904_survives_write_and_read() {
    let mut book = umya_spreadsheet::new_file();
    book.set_date1904(true);
    let bytes = write(&book);
    assert!(workbook_xml(&bytes).contains(r#"date1904="1""#));

    let reread = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    assert!(reread.date1904());

    // and a second save keeps it: a 1904 workbook must not silently become a
    // 1900 one, which would move every date back by 1,462 days
    assert!(workbook_xml(&write(&reread)).contains(r#"date1904="1""#));
}

#[test]
fn date1904_true_spelling_is_read() {
    let mut book = umya_spreadsheet::new_file();
    book.set_date1904(true);
    let bytes = write(&book);

    // rewrite the attribute as xsd:boolean "true", which is also valid
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.name().to_string();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        if name == "xl/workbook.xml" {
            data = String::from_utf8(data)
                .unwrap()
                .replace(r#"date1904="1""#, r#"date1904="true""#)
                .into_bytes();
        }
        out.start_file(name, zip::write::SimpleFileOptions::default())
            .unwrap();
        std::io::Write::write_all(&mut out, &data).unwrap();
    }
    let bytes = out.finish().unwrap().into_inner();

    let reread = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    assert!(reread.date1904());
}
