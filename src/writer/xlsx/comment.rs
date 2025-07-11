use std::{
    collections::HashSet,
    io,
};

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
        write_text_node,
    },
};
use crate::{
    helper::const_str::SHEET_MAIN_NS,
    structs::{
        Worksheet,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    if worksheet.comments().is_empty() {
        return Ok(String::new());
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // comments
    write_start_tag(
        &mut writer,
        "comments",
        vec![("xmlns", SHEET_MAIN_NS).into()],
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
    for comment in worksheet.comments() {
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
        .comments()
        .iter()
        .map(|comment| comment.author().to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
