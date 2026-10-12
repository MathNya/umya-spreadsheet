use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    Comment,
    Workbook,
    reader,
    writer,
};
use zip::ZipArchive;

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn vml_xml(bytes: &[u8]) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/drawings/vmlDrawing1.vml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

#[test]
fn comment_shape_stroke_survives_roundtrip() {
    let mut book = umya_spreadsheet::new_file();
    let mut comment = Comment::default();
    comment.new_comment("B2");
    comment.set_text_string("note");
    comment
        .shape_mut()
        .set_stroked(false)
        .set_stroke_color("#ff0000")
        .set_stroke_weight("2pt");
    book.sheet_mut(0).unwrap().add_comments(comment);

    let bytes = write(&book);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let shape = book.sheet(0).unwrap().comments()[0].shape();
    assert!(!shape.stroked());
    assert_eq!(shape.stroke_color(), "#ff0000");
    assert_eq!(shape.stroke_weight(), "2pt");

    // stroked="f" has to be written again, VML strokes shapes by default
    let xml = vml_xml(&write(&book));
    assert!(xml.contains(r#"stroked="f""#), "{xml}");
    assert!(xml.contains(r##"strokecolor="#ff0000""##), "{xml}");
    assert!(xml.contains(r#"strokeweight="2pt""#), "{xml}");
}
