use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    reader,
    writer,
};
use zip::ZipArchive;

// openpyxl writes the picture geometry as a self-closing <a:prstGeom prst="rect"/>.
#[test]
fn self_closing_preset_geometry_keeps_prst() {
    let book = reader::xlsx::read("./tests/test_files/openpyxl_image.xlsx").unwrap();
    let mut output = Vec::new();
    writer::xlsx::write_writer(&book, &mut output).unwrap();

    let mut archive = ZipArchive::new(Cursor::new(output)).unwrap();
    let mut drawing = String::new();
    archive
        .by_name("xl/drawings/drawing1.xml")
        .unwrap()
        .read_to_string(&mut drawing)
        .unwrap();

    assert!(drawing.contains(r#"<a:prstGeom prst="rect">"#), "{drawing}");
}
