//! Tests for the streaming xlsx writer (`writer::streaming_writer`).
//!
//! The streaming writer flushes worksheets one at a time and only emits the
//! workbook-level metadata (styles, shared strings, workbook.xml, content
//! types, ...) in `finish`. These tests assert that the bytes it produces are:
//!   1. a valid OOXML package containing the parts our format requires,
//!   2. fully round-trippable through our own reader (sheet names, order and
//!      every cell value/formula survive), and
//!   3. the same package (same set of parts) and same content the normal
//!      in-memory writer emits for an identical workbook.

extern crate umya_spreadsheet;
extern crate zip;

use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    Workbook,
    Worksheet,
    new_file,
    reader,
    writer::{
        self,
        streaming_writer::StreamingWriter,
    },
};

/// Sheet names used by the tests, in workbook order. The third name is
/// intentionally non-ASCII to exercise UTF-8 handling in workbook.xml.
const SHEETS: [&str; 3] = ["Sheet1", "Sheet2", "データ"];

/// Build a deterministic, multi-sheet workbook covering the common cell value
/// kinds (string, number, bool, formula) plus a non-ASCII string value.
fn build_book() -> Workbook {
    let mut book = new_file(); // creates "Sheet1"
    book.new_sheet("Sheet2").unwrap();
    book.new_sheet("データ").unwrap();

    let s1 = book.sheet_by_name_mut("Sheet1").unwrap();
    s1.cell_mut("A1").set_value("hello");
    s1.cell_mut("A2").set_value_number(42);
    s1.cell_mut("A3").set_value_bool(true);
    s1.cell_mut("B1").set_formula("A2+1");

    let s2 = book.sheet_by_name_mut("Sheet2").unwrap();
    s2.cell_mut("A1").set_value("world");
    s2.cell_mut("C3").set_value_number(3.14);

    let s3 = book.sheet_by_name_mut("データ").unwrap();
    s3.cell_mut("A1").set_value("うみゃー");

    book
}

/// Stream `book` to an in-memory xlsx, flushing sheets in `order`.
fn stream_to_bytes(book: Workbook, order: &[&str]) -> Vec<u8> {
    let zip_writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let mut sw = StreamingWriter::new(zip_writer, book);
    for name in order {
        let sheet = sw
            .take_sheet(name)
            .unwrap_or_else(|| panic!("sheet `{name}` not available to take"));
        sw.flush_sheet(sheet).unwrap();
    }
    sw.finish().expect("streaming finish").into_inner()
}

/// Sorted list of the part (file) names inside an xlsx byte buffer. Also proves
/// the buffer is a readable zip archive.
fn zip_part_names(bytes: &[u8]) -> Vec<String> {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).expect("bytes are a valid zip");
    let mut names: Vec<String> = (0..zip.len())
        .map(|i| zip.by_index(i).unwrap().name().to_string())
        .collect();
    names.sort();
    names
}

/// Every cell value/formula written by `build_book`, read back and asserted.
fn assert_book_contents(book: &Workbook) {
    let names: Vec<&str> = book
        .sheet_collection()
        .iter()
        .map(Worksheet::name)
        .collect();
    assert_eq!(names, SHEETS, "sheet names/order must round-trip");

    let s1 = book.sheet_by_name("Sheet1").unwrap();
    assert_eq!(s1.value("A1"), "hello");
    assert_eq!(s1.value("A2"), "42");
    assert_eq!(s1.value("A3"), "TRUE");
    assert_eq!(s1.cell("B1").expect("B1 missing").formula(), "A2+1");

    let s2 = book.sheet_by_name("Sheet2").unwrap();
    assert_eq!(s2.value("A1"), "world");
    assert_eq!(s2.value("C3"), "3.14");

    let s3 = book.sheet_by_name("データ").unwrap();
    assert_eq!(s3.value("A1"), "うみゃー");
}

/// The streamed bytes must read back through our own reader with every sheet,
/// name, order, value and formula intact.
#[test]
fn streaming_writer_roundtrips_all_sheets() {
    let bytes = stream_to_bytes(build_book(), &SHEETS);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).expect("read streamed xlsx");
    assert_book_contents(&book);
}

/// The streamed bytes must be a valid OPC zip that contains the canonical parts
/// an xlsx package requires (content types, package + workbook rels, workbook,
/// styles, shared strings, theme, and one worksheet part per sheet).
#[test]
fn streaming_writer_emits_valid_ooxml_package() {
    let bytes = stream_to_bytes(build_book(), &SHEETS);
    let parts = zip_part_names(&bytes);

    let required = [
        "[Content_Types].xml",
        "_rels/.rels",
        "xl/_rels/workbook.xml.rels",
        "xl/workbook.xml",
        "xl/styles.xml",
        "xl/sharedStrings.xml",
        "xl/theme/theme1.xml",
        "xl/worksheets/sheet1.xml",
        "xl/worksheets/sheet2.xml",
        "xl/worksheets/sheet3.xml",
    ];
    for part in required {
        assert!(
            parts.iter().any(|p| p == part),
            "streamed package is missing required part `{part}`; got {parts:?}",
        );
    }

    // One worksheet part per sheet, no more.
    let sheet_parts = parts
        .iter()
        .filter(|p| p.starts_with("xl/worksheets/sheet") && p.ends_with(".xml"))
        .count();
    assert_eq!(sheet_parts, SHEETS.len(), "one worksheet part per sheet");
}

/// The streaming writer must produce the same package (same set of parts) and
/// the same logical content as the normal in-memory writer for an identical
/// workbook. This is the strongest "follows our format" guarantee: anything the
/// normal writer emits, the streaming writer emits too.
#[test]
fn streaming_writer_matches_normal_writer() {
    // Normal writer (consumes its own copy of the workbook).
    let mut normal_bytes = Vec::new();
    writer::xlsx::write_writer(&build_book(), &mut normal_bytes).expect("normal write");

    // Streaming writer, flushing in the same order the normal writer iterates.
    let streamed_bytes = stream_to_bytes(build_book(), &SHEETS);

    assert_eq!(
        zip_part_names(&streamed_bytes),
        zip_part_names(&normal_bytes),
        "streaming and normal writers must emit the same set of package parts",
    );

    // Both must read back to identical content.
    let from_stream =
        reader::xlsx::read_reader(Cursor::new(streamed_bytes), true).expect("read streamed");
    let from_normal =
        reader::xlsx::read_reader(Cursor::new(normal_bytes), true).expect("read normal");
    assert_book_contents(&from_stream);
    assert_book_contents(&from_normal);
}

/// Sheets may be flushed in an order different from the workbook's declared
/// order. Whatever order they are flushed in becomes the on-disk worksheet
/// order; the reader must reflect it.
#[test]
fn streaming_writer_respects_flush_order() {
    let order = ["データ", "Sheet1", "Sheet2"];
    let bytes = stream_to_bytes(build_book(), &order);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).expect("read streamed xlsx");

    let names: Vec<&str> = book
        .sheet_collection()
        .iter()
        .map(Worksheet::name)
        .collect();
    assert_eq!(names, order, "flush order must determine worksheet order");

    // Content still lands on the correct sheet regardless of flush order.
    assert_eq!(book.sheet_by_name("Sheet1").unwrap().value("A1"), "hello");
    assert_eq!(
        book.sheet_by_name("データ").unwrap().value("A1"),
        "うみゃー"
    );
}

/// Count `(total_charts, charts_that_still_have_a_legend)` inside an xlsx
/// buffer.
fn chart_legend_counts(bytes: &[u8]) -> (usize, usize) {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).unwrap();
    let mut charts = 0;
    let mut with_legend = 0;
    for i in 0..zip.len() {
        let mut e = zip.by_index(i).unwrap();
        let name = e.name().to_string();
        if name.starts_with("xl/charts/chart") && name.ends_with(".xml") {
            charts += 1;
            let mut s = String::new();
            e.read_to_string(&mut s).unwrap();
            if s.contains("<c:legend>") {
                with_legend += 1;
            }
        }
    }
    (charts, with_legend)
}

/// Streaming-writer counterpart of `set_legend_present_false_suppresses_legend`
/// in `tests/integration_test.rs`. Reads the real `aaa.xlsx` fixture (which has
/// charts), suppresses every legend via the streaming writer's intended
/// take -> mutate -> flush workflow, and asserts no `<c:legend>` survives.
///
/// This exercises the chart/drawing/relationship code path in `flush_sheet`,
/// which the synthetic round-trip tests above do not cover.
///
/// KNOWN FAILURE: the streaming writer cannot currently write a sheet that
/// has a chart referencing cells. `StreamingWriter::new` calls
/// `take_all_sheets()`, emptying `work_book`, but chart num/str caches rebuild
/// their `<c:pt>` values at write time via
/// `wb.cell_value_by_address_crate(..)`, which then panics with `NotFound`.
/// This test will pass once `flush_sheet` keeps the sheet reachable to the
/// workbook during `chart::write`.
#[test]
fn streaming_writer_suppresses_legend_like_normal_writer() {
    let src = std::path::Path::new("./tests/test_files/aaa.xlsx");

    let book = reader::xlsx::read(src).expect("read aaa.xlsx");
    // Sheet names must be captured before the workbook is moved into the writer.
    let sheet_names: Vec<String> = book
        .sheet_collection()
        .iter()
        .map(|ws| ws.name().to_string())
        .collect();

    let zip_writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let mut sw = StreamingWriter::new(zip_writer, book);
    for name in &sheet_names {
        let mut sheet = sw
            .take_sheet(name)
            .unwrap_or_else(|| panic!("sheet `{name}` not available to take"));
        // Mutate the taken sheet before flushing it.
        for chart in sheet.chart_collection_mut().iter_mut() {
            chart
                .chart_space_mut()
                .chart_mut()
                .set_legend_present(false);
        }
        sw.flush_sheet(sheet).unwrap();
    }
    let bytes = sw.finish().expect("streaming finish").into_inner();

    // Sanity: the package is still readable end-to-end after charts/drawings.
    reader::xlsx::read_reader(Cursor::new(bytes.clone()), true)
        .expect("streamed xlsx with charts must read back");

    let (charts, with_legend) = chart_legend_counts(&bytes);
    assert!(charts > 0, "aaa.xlsx should contain charts");
    assert_eq!(
        with_legend, 0,
        "streaming set_legend_present(false) should suppress all legends; {with_legend}/{charts} \
         remain"
    );
}
