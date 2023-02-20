use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    if worksheet.get_comments().is_empty() {
        return Ok(String::from(""));
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // comments
    write_start_tag(
        &mut writer,
        "comments",
        vec![(
            "xmlns",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        )],
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
        let coordinate = comment.get_coordinate().get_coordinate();
        let author_id = get_author_id(&authors, comment.get_author());
        write_start_tag(
            &mut writer,
            "comment",
            vec![("ref", &coordinate), ("authorId", author_id.as_str())],
            false,
        );

        // text
        comment.get_text().write_to_text(&mut writer);

        write_end_tag(&mut writer, "comment");
    }
    write_end_tag(&mut writer, "commentList");
    write_end_tag(&mut writer, "comments");

    let file_no = writer_mng.add_file_at_comment(writer)?;
    Ok(file_no.to_string())
}

fn get_authors(worksheet: &Worksheet) -> Vec<String> {
    let mut authors: Vec<String> = Vec::new();
    for comment in worksheet.get_comments() {
        let mut is_match = false;
        for author in &authors {
            if comment.get_author() == author {
                is_match = true;
                break;
            }
        }
        if !is_match {
            authors.push(comment.get_author().to_string());
        }
    }
    authors
}

fn get_author_id(authors: &Vec<String>, author: &str) -> String {
    for (i, value) in authors.iter().enumerate() {
        if author == value {
            return i.to_string();
        }
    }
    "".to_string()
}
