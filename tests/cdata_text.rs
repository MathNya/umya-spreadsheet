use std::io::{
    Cursor,
    Read,
    Write,
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

fn workbook_with_header_text(content: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("CDATA regression");
    sheet
        .header_footer_mut()
        .odd_header_mut()
        .set_value("HEADER_PLACEHOLDER");
    sheet
        .header_footer_mut()
        .odd_footer_mut()
        .set_value("FOOTER_PLACEHOLDER");
    let mut original = Vec::new();
    writer::xlsx::write_writer(&book, &mut original).unwrap();
    let mut source = ZipArchive::new(Cursor::new(original)).unwrap();
    let mut output = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..source.len() {
        let mut part = source.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/worksheets/sheet1.xml" {
            bytes = String::from_utf8(bytes)
                .unwrap()
                .replace("HEADER_PLACEHOLDER", content)
                .replace("FOOTER_PLACEHOLDER", content)
                .into_bytes();
        }
        output
            .start_file(part.name(), SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

#[test]
fn header_footer_text_preserves_cdata_and_decodes_only_outside_entities() {
    for (xml, expected) in [
        (
            "&amp;CResearch &amp; Development",
            "&CResearch & Development",
        ),
        (
            "<![CDATA[&C&BResearch & Development&B]]>",
            "&C&BResearch & Development&B",
        ),
        (
            "&amp;LBefore <![CDATA[&amp; literal <tag>]]> &#65;<![CDATA[&Rafter]]>",
            "&LBefore &amp; literal <tag> A&Rafter",
        ),
        (
            "<![CDATA[]]>&amp;C<![CDATA[One]]><![CDATA[Two]]>",
            "&COneTwo",
        ),
        ("&lt;![CDATA[ordinary]]&gt;", "<![CDATA[ordinary]]>"),
    ] {
        let bytes = workbook_with_header_text(xml);
        let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
        let header_footer = book.sheet(0).unwrap().header_footer();
        assert_eq!(header_footer.odd_header().value(), expected, "{xml}");
        assert_eq!(header_footer.odd_footer().value(), expected, "{xml}");
    }
}
