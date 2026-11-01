//! The calculation properties of a workbook (`calcPr`) must survive reading
//! and saving it.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    CalculateModeValues,
    ReferenceModeValues,
    Workbook,
    reader,
    writer,
};

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn workbook_xml(bytes: &[u8]) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/workbook.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn replace_workbook_xml(bytes: Vec<u8>, edit: impl Fn(&mut String)) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name().unwrap() == "xl/workbook.xml" {
            let mut xml = String::from_utf8(bytes).unwrap();
            edit(&mut xml);
            bytes = xml.into_bytes();
        }
        output
            .start_file(
                part.name().unwrap(),
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

fn calc_pr_xml(xml: &str) -> String {
    let start = xml
        .find("<calcPr")
        .unwrap_or_else(|| panic!("calcPr not written: {xml}"));
    let end = start + xml[start..].find("/>").unwrap() + 2;
    xml[start..end].to_string()
}

fn has_attribute(element: &str, name: &str, value: &str) -> bool {
    element.contains(&format!(r#" {name}="{value}""#))
}

/// Saves a new workbook and replaces its `calcPr` with `calc_pr`.
fn book_with_calc_pr(calc_pr: &str) -> Vec<u8> {
    let book = umya_spreadsheet::new_file();
    replace_workbook_xml(write(&book), |xml| {
        let old = calc_pr_xml(xml);
        *xml = xml.replace(&old, calc_pr);
    })
}

const CALC_PR: &str = r#"<calcPr calcId="191029" calcMode="manual" fullCalcOnLoad="1" refMode="R1C1" iterate="1" iterateCount="50" iterateDelta="0.0001" fullPrecision="0" calcCompleted="0" calcOnSave="0" concurrentCalc="0" concurrentManualCount="2" forceFullCalc="1"/>"#;

#[test]
fn calculation_properties_are_read() {
    let bytes = book_with_calc_pr(CALC_PR);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let calc_pr = book.calculation_properties();
    assert_eq!(calc_pr.calc_mode(), &CalculateModeValues::Manual);
    assert!(calc_pr.full_calc_on_load());
    assert_eq!(calc_pr.ref_mode(), &ReferenceModeValues::R1C1);
    assert!(calc_pr.iterate());
    assert_eq!(calc_pr.iterate_count(), 50);
    assert!((calc_pr.iterate_delta() - 0.0001).abs() < f64::EPSILON);
    assert!(!calc_pr.full_precision());
    assert!(!calc_pr.calc_completed());
    assert!(!calc_pr.calc_on_save());
    assert!(!calc_pr.concurrent_calc());
    assert_eq!(calc_pr.concurrent_manual_count(), Some(2));
    assert!(calc_pr.force_full_calc());
}

#[test]
fn missing_attributes_use_defaults() {
    let bytes = book_with_calc_pr(r#"<calcPr calcId="191029"/>"#);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let calc_pr = book.calculation_properties();
    assert_eq!(calc_pr.calc_mode(), &CalculateModeValues::Auto);
    assert!(!calc_pr.full_calc_on_load());
    assert_eq!(calc_pr.ref_mode(), &ReferenceModeValues::A1);
    assert!(!calc_pr.iterate());
    assert_eq!(calc_pr.iterate_count(), 100);
    assert!((calc_pr.iterate_delta() - 0.001).abs() < f64::EPSILON);
    assert!(calc_pr.full_precision());
    assert!(calc_pr.calc_completed());
    assert!(calc_pr.calc_on_save());
    assert!(calc_pr.concurrent_calc());
    assert_eq!(calc_pr.concurrent_manual_count(), None);
    assert!(!calc_pr.force_full_calc());

    let calc_pr = calc_pr_xml(&workbook_xml(&write(&book)));
    assert_eq!(calc_pr, r#"<calcPr calcId="122211"/>"#);
}

#[test]
fn calculation_properties_are_written_back() {
    let bytes = book_with_calc_pr(CALC_PR);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let calc_pr = calc_pr_xml(&workbook_xml(&write(&book)));
    for (name, value) in [
        ("calcMode", "manual"),
        ("fullCalcOnLoad", "1"),
        ("refMode", "R1C1"),
        ("iterate", "1"),
        ("iterateCount", "50"),
        ("iterateDelta", "0.0001"),
        ("fullPrecision", "0"),
        ("calcCompleted", "0"),
        ("calcOnSave", "0"),
        ("concurrentCalc", "0"),
        ("concurrentManualCount", "2"),
        ("forceFullCalc", "1"),
    ] {
        assert!(has_attribute(&calc_pr, name, value), "{name}: {calc_pr}");
    }
    // The old calcId is kept so that Excel recalculates the formulas.
    assert!(has_attribute(&calc_pr, "calcId", "122211"), "{calc_pr}");

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    assert_eq!(
        book.calculation_properties().calc_mode(),
        &CalculateModeValues::Manual
    );
}

#[test]
fn calculation_properties_can_be_set() {
    let mut book = umya_spreadsheet::new_file();
    book.calculation_properties_mut()
        .set_calc_mode(CalculateModeValues::Manual)
        .set_full_calc_on_load(true);
    let calc_pr = calc_pr_xml(&workbook_xml(&write(&book)));
    assert_eq!(
        calc_pr,
        r#"<calcPr calcId="122211" calcMode="manual" fullCalcOnLoad="1"/>"#
    );
}

#[test]
fn new_workbook_calc_pr_is_unchanged() {
    let calc_pr = calc_pr_xml(&workbook_xml(&write(&umya_spreadsheet::new_file())));
    assert_eq!(calc_pr, r#"<calcPr calcId="122211"/>"#);
}
