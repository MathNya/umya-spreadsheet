//! The "fit to page" print scaling flag must survive an XLSX round trip.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    reader,
    writer,
};

fn sheet_xml(bytes: &[u8]) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn replace_sheet_xml(bytes: Vec<u8>, edit: impl Fn(&mut String)) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/worksheets/sheet1.xml" {
            let mut xml = String::from_utf8(bytes).unwrap();
            edit(&mut xml);
            bytes = xml.into_bytes();
        }
        output
            .start_file(part.name(), zip::write::SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

#[test]
fn fit_to_page_survives_round_trip() {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Wide report");
    sheet
        .page_setup_mut()
        .set_fit_to_page(true)
        .set_fit_to_width(1)
        .set_fit_to_height(0);
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    assert!(sheet_xml(&bytes).contains(r#"<sheetPr><pageSetUpPr fitToPage="1"/></sheetPr>"#));

    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let page_setup = reopened.sheet(0).unwrap().page_setup();
    assert!(page_setup.fit_to_page());
    assert_eq!(page_setup.fit_to_width(), 1);
    assert_eq!(page_setup.fit_to_height(), 0);
}

#[test]
fn fit_to_page_from_another_application_is_kept_on_save() {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Scaled report");
    sheet.tab_color_mut().set_argb_str("FFFF0000");
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let bytes = replace_sheet_xml(bytes, |xml| {
        *xml = xml.replace("</sheetPr>", r#"<pageSetUpPr fitToPage="true"/></sheetPr>"#);
        let at = xml.find("<pageMargins ").unwrap();
        let end = at + xml[at..].find("/>").unwrap() + 2;
        xml.insert_str(end, r#"<pageSetup fitToWidth="2" fitToHeight="3"/>"#);
    });

    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let page_setup = book.sheet(0).unwrap().page_setup();
    assert!(page_setup.fit_to_page());
    assert_eq!(page_setup.fit_to_width(), 2);
    assert_eq!(page_setup.fit_to_height(), 3);

    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let xml = sheet_xml(&bytes);
    assert!(xml.contains(r#"<pageSetUpPr fitToPage="1"/></sheetPr>"#));
    assert!(xml.contains(r#"<tabColor rgb="FFFF0000"/>"#));
}

#[test]
fn unset_fit_to_page_writes_no_sheet_properties() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value("Plain report");
    assert!(!book.sheet(0).unwrap().page_setup().fit_to_page());
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    assert!(!sheet_xml(&bytes).contains("sheetPr"));
}
