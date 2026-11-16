//! An array formula and its cell metadata survive reading and writing: the
//! formula keeps `t="array"` and its `ref`, a dynamic array formula's cell keeps
//! its `cm` index, and the cell metadata part it points into (`xl/metadata.xml`)
//! is written back with its relationship and content type. Writing the marker
//! without the part made Excel repair the file (#206); now both are kept.
use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    reader,
    writer,
};

const SOURCE: &str = "./tests/test_files/issue_206.xlsx";

fn written(book: &umya_spreadsheet::Workbook) -> zip::ZipArchive<Cursor<Vec<u8>>> {
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(book, &mut bytes).unwrap();
    zip::ZipArchive::new(Cursor::new(bytes)).unwrap()
}

fn part(zip: &mut zip::ZipArchive<Cursor<Vec<u8>>>, name: &str) -> Option<Vec<u8>> {
    let mut file = zip.by_name(name).ok()?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Some(buf)
}

fn text(zip: &mut zip::ZipArchive<Cursor<Vec<u8>>>, name: &str) -> String {
    String::from_utf8(part(zip, name).unwrap_or_else(|| panic!("{name} written"))).unwrap()
}

#[test]
fn a_dynamic_array_formula_keeps_its_marker_its_range_and_its_metadata() {
    let source = std::fs::read(SOURCE).unwrap();
    let mut original = zip::ZipArchive::new(Cursor::new(source)).unwrap();
    let metadata = part(&mut original, "xl/metadata.xml").unwrap();

    let book = reader::xlsx::read(std::path::Path::new(SOURCE)).unwrap();
    let cell = book.sheet(0).unwrap().cell("J24").unwrap();
    assert_eq!(cell.cell_meta_index(), 1);
    assert_eq!(cell.formula_obj().unwrap().reference(), "J24");

    let mut out = written(&book);
    let sheet = text(&mut out, "xl/worksheets/sheet1.xml");
    let j24 = &sheet[sheet.find(r#"<c r="J24""#).unwrap()..];
    let j24 = &j24[..j24.find("</c>").unwrap()];
    assert!(j24.contains(r#"cm="1""#), "{j24}");
    assert!(j24.contains(r#"<f t="array" ref="J24">SUMPRODUCT($F$23:$I$23*F24:I24)</f>"#), "{j24}");

    assert_eq!(part(&mut out, "xl/metadata.xml").as_deref(), Some(metadata.as_slice()), "the part as read");
    assert!(text(&mut out, "xl/_rels/workbook.xml.rels").contains(
        r#"Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata" Target="metadata.xml""#
    ));
    assert!(text(&mut out, "[Content_Types].xml").contains(
        r#"<Override PartName="/xl/metadata.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"/>"#
    ));
}

#[test]
fn a_cell_metadata_index_is_written_only_with_its_part_and_its_formula() {
    // Without the part, `cm` would point at nothing; the array formula stays.
    let mut book = reader::xlsx::read(std::path::Path::new(SOURCE)).unwrap();
    book.remove_sheet_metadata();
    let mut out = written(&book);
    assert!(part(&mut out, "xl/metadata.xml").is_none());
    let sheet = text(&mut out, "xl/worksheets/sheet1.xml");
    assert!(!sheet.contains(" cm="), "no cm without its part");
    assert!(sheet.contains(r#"<f t="array" ref="J24">"#));

    // A value written over the formula leaves no `cm` behind.
    let mut book = reader::xlsx::read(std::path::Path::new(SOURCE)).unwrap();
    book.sheet_mut(0).unwrap().cell_mut("J24").set_value_number(7);
    let mut out = written(&book);
    let sheet = text(&mut out, "xl/worksheets/sheet1.xml");
    assert!(!sheet.contains(" cm="), "no cm once the formula is gone");
}
