//! Public API regression coverage for lazy loaded-sheet access.
use std::{
    fs,
    path::PathBuf,
    process,
};

use umya_spreadsheet::{
    XlsxErrorKind,
    reader,
    writer,
};

fn fixture_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "umya-loaded-sheet-access-{}-{}.xlsx",
        process::id(),
        std::thread::current().name().unwrap_or("test")
    ))
}

#[test]
fn loaded_sheet_access_preserves_lazy_deserialization_and_errors() {
    let path = fixture_path();
    let mut source = umya_spreadsheet::new_file();
    source.new_sheet("Second").unwrap();
    source
        .sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value("first");
    source
        .sheet_by_name_mut("Second")
        .unwrap()
        .cell_mut("A1")
        .set_value("second");
    writer::xlsx::write(&source, &path).unwrap();

    let mut book = reader::xlsx::lazy_read(&path).unwrap();
    assert_eq!(
        book.sheet(0).unwrap_err().kind(),
        XlsxErrorKind::NotDeserialized
    );
    assert_eq!(
        book.sheet_by_name("Second").unwrap_err().kind(),
        XlsxErrorKind::NotDeserialized
    );
    assert_eq!(
        book.sheet_collection().unwrap_err().kind(),
        XlsxErrorKind::NotDeserialized
    );
    assert_eq!(
        book.sheet_mut(2).unwrap_err().kind(),
        XlsxErrorKind::NotFound
    );
    assert_eq!(
        book.read_sheet(2).unwrap_err().kind(),
        XlsxErrorKind::NotFound
    );
    assert_eq!(
        book.read_sheet_by_name("Missing").unwrap_err().kind(),
        XlsxErrorKind::NotFound
    );

    assert_eq!(book.sheet_mut(0).unwrap().value("A1"), "first");
    assert_eq!(book.sheet(0).unwrap().value("A1"), "first");
    assert_eq!(
        book.sheet(1).unwrap_err().kind(),
        XlsxErrorKind::NotDeserialized
    );

    book.read_sheet_by_name("Second").unwrap();
    assert_eq!(book.sheet_by_name("Second").unwrap().value("A1"), "second");

    book.sheet_collection_mut().unwrap();
    assert_eq!(book.sheet_collection().unwrap().len(), 2);
    fs::remove_file(path).unwrap();
}
