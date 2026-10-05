//! Files whose references point at nothing should still be readable: Excel
//! repairs each of these by dropping the dangling reference. They used to
//! panic in the reader.

use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    Comment,
    Hyperlink,
    reader,
    writer,
};
use zip::{
    ZipArchive,
    ZipWriter,
    write::SimpleFileOptions,
};

fn write(book: &umya_spreadsheet::Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

/// Rewrite every entry whose name satisfies `pick` with `edit`.
fn patch(bytes: &[u8], pick: impl Fn(&str) -> bool, edit: impl Fn(String) -> String) -> Vec<u8> {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut out = ZipWriter::new(Cursor::new(Vec::new()));
    let mut patched = false;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.name().to_string();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        if pick(&name) {
            let before = String::from_utf8(data).unwrap();
            let after = edit(before.clone());
            assert_ne!(before, after, "the patch changed nothing in {name}");
            data = after.into_bytes();
            patched = true;
        }
        out.start_file(name, SimpleFileOptions::default()).unwrap();
        out.write_all(&data).unwrap();
    }
    assert!(patched, "no entry matched");
    out.finish().unwrap().into_inner()
}

fn read(bytes: Vec<u8>) -> umya_spreadsheet::Workbook {
    reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap()
}

#[test]
fn defined_name_scoped_to_a_missing_sheet_is_dropped() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .add_defined_name("Local", "Sheet1!$A$1")
        .unwrap();
    let bytes = patch(
        &write(&book),
        |name| name == "xl/workbook.xml",
        |xml| {
            xml.replace(
                r#"<definedName name="Local">"#,
                r#"<definedName name="Local" localSheetId="5">"#,
            )
        },
    );

    let book = read(bytes);
    assert!(book.sheet(0).unwrap().defined_names().is_empty());
}

#[test]
fn hyperlink_with_a_dangling_relationship_id_has_no_url() {
    let mut book = umya_spreadsheet::new_file();
    let mut hyperlink = Hyperlink::default();
    hyperlink.set_url("https://example.com/");
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_hyperlink(hyperlink);
    let bytes = patch(
        &write(&book),
        |name| name == "xl/worksheets/sheet1.xml",
        |xml| xml.replace(r#"r:id=""#, r#"r:id="missing"#),
    );

    let book = read(bytes);
    let hyperlink = book
        .sheet(0)
        .unwrap()
        .cell("A1")
        .unwrap()
        .hyperlink()
        .unwrap();
    assert_eq!(hyperlink.url(), "");
}

#[test]
fn conditional_format_with_a_dangling_dxf_id_is_read() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value_number(1);
    let bytes = patch(
        &write(&book),
        |name| name == "xl/worksheets/sheet1.xml",
        |xml| {
            xml.replace(
                "</sheetData>",
                r#"</sheetData><conditionalFormatting sqref="A1"><cfRule type="expression" dxfId="7" priority="1"><formula>TRUE</formula></cfRule></conditionalFormatting>"#,
            )
        },
    );

    let book = read(bytes);
    assert_eq!(
        book.sheet(0)
            .unwrap()
            .conditional_formatting_collection()
            .len(),
        1
    );
}

#[test]
fn comment_with_a_dangling_author_id_has_no_author() {
    let mut book = umya_spreadsheet::new_file();
    let mut comment = Comment::default();
    comment.new_comment("B2");
    comment.set_author("Someone");
    comment.set_text_string("note");
    book.sheet_mut(0).unwrap().add_comments(comment);
    let bytes = patch(
        &write(&book),
        |name| name.starts_with("xl/comments"),
        |xml| xml.replace(r#"authorId="0""#, r#"authorId="5""#),
    );

    let book = read(bytes);
    let comments = book.sheet(0).unwrap().comments();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].author(), "");
}
