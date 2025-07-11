use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Worksheet;
use crate::structs::WriterManager;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::collections::HashSet;
use std::io;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    if worksheet.get_comments().is_empty() {
        return Ok(String::new());
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // comments
    write_start_tag(
        &mut writer,
        "comments",
        vec![("xmlns", SHEET_MAIN_NS)],
        false,
    );

    // authors
    let authors = get_authors(worksheet);
    write_start_tag(&mut writer, "authors", vec![], false);
    for author in &authors {
        write_start_tag(&mut writer, "author", vec![], false);
        write_text_node(&mut writer, author);
        write_end_tag(&mut writer, "author");
    }
    write_end_tag(&mut writer, "authors");

    // commentList
    write_start_tag(&mut writer, "commentList", vec![], false);
    for comment in worksheet.get_comments() {
        // comment
        comment.write_to(&mut writer, &authors);
    }
    write_end_tag(&mut writer, "commentList");
    write_end_tag(&mut writer, "comments");

    let file_no = writer_mng.add_file_at_comment(writer)?;
    Ok(file_no.to_string())
}

fn get_authors(worksheet: &Worksheet) -> Vec<String> {
    worksheet
        .get_comments()
        .into_iter()
        .map(|comment| comment.get_author().to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
