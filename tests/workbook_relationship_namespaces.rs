//! Workbook relationships must be resolved by namespace, not prefix spelling.
use std::io::{
    Cursor,
    Read,
    Write,
};

use rstest::rstest;
use umya_spreadsheet::{
    Border,
    reader,
    writer,
};
use zip::{
    ZipArchive,
    ZipWriter,
    write::SimpleFileOptions,
};

fn workbook_with_relationship_prefix(prefix: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Namespace regression");
    sheet.cell_mut("B2").set_value_number(42);
    sheet.style_mut("A1").font_mut().set_bold(true);
    sheet
        .style_mut("A1")
        .borders_mut()
        .bottom_mut()
        .set_border_style(Border::BORDER_THIN);

    let mut original = Vec::new();
    writer::xlsx::write_writer(&book, &mut original).unwrap();
    let mut source = ZipArchive::new(Cursor::new(original)).unwrap();
    let mut output = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..source.len() {
        let mut part = source.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/_rels/workbook.xml.rels" && !prefix.is_empty() {
            let xml = String::from_utf8(bytes).unwrap();
            bytes = xml
                .replace("<Relationship", &format!("<{prefix}:Relationship"))
                .replace("</Relationships>", &format!("</{prefix}:Relationships>"))
                .replace("xmlns=", &format!("xmlns:{prefix}="))
                .into_bytes();
        }
        output
            .start_file(part.name(), SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

#[rstest]
#[case("")]
#[case("ns0")]
#[case("pkg")]
fn workbook_relationship_prefix_preserves_cells_and_styles(#[case] prefix: &str) {
    let bytes = workbook_with_relationship_prefix(prefix);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    for book in [book.clone(), {
        let mut output = Vec::new();
        writer::xlsx::write_writer(&book, &mut output).unwrap();
        reader::xlsx::read_reader(Cursor::new(output), true).unwrap()
    }] {
        let sheet = book.sheet(0).unwrap();
        assert_eq!(sheet.value("A1"), "Namespace regression");
        assert_eq!(sheet.value("B2"), "42");
        assert!(sheet.style("A1").font().unwrap().bold());
        assert_eq!(
            sheet.style("A1").borders().unwrap().bottom().border_style(),
            Border::BORDER_THIN
        );
    }
}
