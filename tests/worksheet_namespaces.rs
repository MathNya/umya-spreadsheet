use std::io::{
    Cursor,
    Read,
    Write,
};

use quick_xml::{
    Reader,
    Writer,
    events::Event,
};
use umya_spreadsheet::{
    reader,
    writer,
};
use zip::{
    ZipArchive,
    ZipWriter,
    write::SimpleFileOptions,
};

fn workbook_with_worksheet_prefix(prefix: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Quarterly report");
    sheet.cell_mut("B2").set_value_number(240);
    sheet.cell_mut("A1").style_mut().font_mut().set_bold(true);
    sheet
        .header_footer_mut()
        .odd_header_mut()
        .set_value("&CReport heading");
    sheet
        .header_footer_mut()
        .odd_footer_mut()
        .set_value("&LInternal&R&P / &N");
    let mut original = Vec::new();
    writer::xlsx::write_writer(&book, &mut original).unwrap();
    let mut input = ZipArchive::new(Cursor::new(original)).unwrap();
    let mut output = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..input.len() {
        let mut part = input.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/worksheets/sheet1.xml" && !prefix.is_empty() {
            let mut xml = Reader::from_reader(bytes.as_slice());
            let mut serialized = Writer::new(Vec::new());
            loop {
                let event = xml.read_event().unwrap();
                let is_empty = matches!(event, Event::Empty(_));
                let event = match event {
                    Event::Start(element) | Event::Empty(element) => {
                        let mut renamed = element.into_owned();
                        let local = String::from_utf8(renamed.name().as_ref().to_vec()).unwrap();
                        renamed.set_name(format!("{prefix}:{local}").as_bytes());
                        if local == "worksheet" {
                            renamed.push_attribute((
                                format!("xmlns:{prefix}").as_str(),
                                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                            ));
                        }
                        if is_empty {
                            Event::Empty(renamed)
                        } else {
                            Event::Start(renamed)
                        }
                    }
                    Event::End(element) => Event::End(quick_xml::events::BytesEnd::new(format!(
                        "{prefix}:{}",
                        std::str::from_utf8(element.name().as_ref()).unwrap()
                    ))),
                    Event::Eof => break,
                    other => other.into_owned(),
                };
                serialized.write_event(event).unwrap();
            }
            bytes = serialized.into_inner();
        }
        output
            .start_file(part.name(), SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

fn assert_book_contents(book: &umya_spreadsheet::Workbook) {
    let sheet = book.sheet(0).unwrap();
    assert_eq!(
        sheet.cell("A1").unwrap().value(),
        "Quarterly report",
        "worksheet text must survive namespace handling"
    );
    assert_eq!(sheet.cell("B2").unwrap().value_number(), Some(240.0));
    assert!(sheet.cell("A1").unwrap().style().font().unwrap().bold());
    assert_eq!(
        sheet.header_footer().odd_header().value(),
        "&CReport heading"
    );
    assert_eq!(
        sheet.header_footer().odd_footer().value(),
        "&LInternal&R&P / &N"
    );
}

fn assert_worksheet_contents(prefix: &str) {
    let bytes = workbook_with_worksheet_prefix(prefix);
    for with_sheet_read in [false, true] {
        let mut book =
            reader::xlsx::read_reader(Cursor::new(bytes.clone()), with_sheet_read).unwrap();
        book.read_sheet(0);
        assert_book_contents(&book);
        let mut rewritten = Vec::new();
        writer::xlsx::write_writer(&book, &mut rewritten).unwrap();
        let reopened = reader::xlsx::read_reader(Cursor::new(rewritten), true).unwrap();
        assert_book_contents(&reopened);
    }
}

#[test]
fn default_worksheet_namespace_preserves_contents() {
    assert_worksheet_contents("");
}

#[test]
fn short_worksheet_namespace_prefix_preserves_contents() {
    assert_worksheet_contents("s");
}

#[test]
fn long_worksheet_namespace_prefix_preserves_contents() {
    assert_worksheet_contents("spreadsheet");
}

#[test]
fn prefixed_worksheet_lazy_file_and_cell_stream_preserve_values() {
    for prefix in ["s", "spreadsheet"] {
        let path = std::env::temp_dir().join(format!(
            "umya-worksheet-namespace-{}-{}-{prefix}.xlsx",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file.write_all(&workbook_with_worksheet_prefix(prefix))
            .unwrap();
        drop(file);
        let mut book = reader::xlsx::lazy_read(&path).unwrap();
        let cells = book.lazy_read_sheet_cells(0).unwrap();
        let mut lite_values: Vec<String> = cells
            .iter_collection()
            .map(|cell| cell.value().to_string())
            .collect();
        lite_values.sort();
        assert_eq!(lite_values, vec!["240", "Quarterly report"]);
        book.read_sheet(0);
        assert_book_contents(&book);
        let mut values = Vec::new();
        reader::xlsx::read_sheet_by_name_stream(&path, "Sheet1", |cell| {
            values.push(cell.value().to_string());
        })
        .unwrap();
        values.sort();
        assert_eq!(values, vec!["240", "Quarterly report"]);
        std::fs::remove_file(&path).unwrap();
    }
}
