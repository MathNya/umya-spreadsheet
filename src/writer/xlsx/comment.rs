use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::worksheet::Worksheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl";

pub(crate) fn write(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
    dir: &TempDir
) -> Result<(), XlsxError> {
    if worksheet.get_comments().len() == 0 {
        return Ok(());
    }

    let file_name = format!("comments{}.xml", p_worksheet_id);

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // comments
    write_start_tag(&mut writer, "comments", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
    ], false);

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
        let coordinate = comment.get_coordinate().get_coordinate();

        // comment
        let author_id = get_author_id(&authors, comment.get_author());
        write_start_tag(&mut writer, "comment", vec![
            ("ref", &coordinate),
            ("authorId", author_id.as_str()),
        ], false);

        // text
        write_start_tag(&mut writer, "text", vec![], false);
        for element in comment.get_text().get_rich_text_elements() {
            write_start_tag(&mut writer, "r", vec![], false);
            match element.get_font() {
                Some(font) => {
                    write_start_tag(&mut writer, "rPr", vec![], false);
                    // italic
                    if font.get_italic() == &true {
                    write_start_tag(&mut writer, "i", vec![], true);
                    }

                    // strike
                    if font.get_strikethrough() == &true {
                        write_start_tag(&mut writer, "strike", vec![], true);
                    }

                    // sz
                    write_start_tag(&mut writer, "sz", vec![
                        ("val", font.get_size().to_string().as_str()),
                    ], true);

                    // color
                    write_color(&mut writer, &font.get_color(), "color");

                    // name
                    write_start_tag(&mut writer, "name", vec![
                        ("val", font.get_name()),
                    ], true);

                    // family
                    write_start_tag(&mut writer, "family", vec![
                        ("val", font.get_family().to_string().as_str()),
                    ], true);

                    // charset
                    let zero:usize = 0;
                    if font.get_charset() > &zero {
                        write_start_tag(&mut writer, "charset", vec![
                            ("val", font.get_charset().to_string().as_str()),
                        ], true);
                    }

                    // scheme
                    if font.get_scheme() != "" {
                        write_start_tag(&mut writer, "scheme", vec![
                            ("val",  font.get_scheme()),
                        ], true);
                    }
                    write_end_tag(&mut writer, "rPr");
                },
                None => {}
            }
            write_start_tag(&mut writer, "t", vec![], false);
            write_text_node(&mut writer, element.get_text());
            write_end_tag(&mut writer, "t");
            write_end_tag(&mut writer, "r");
        }

        write_end_tag(&mut writer, "text");
        write_end_tag(&mut writer, "comment");
    }
    write_end_tag(&mut writer, "commentList");
    write_end_tag(&mut writer, "comments");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
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
        if is_match == false {
            authors.push(comment.get_author().to_string());
        }
    }
    authors
}

fn get_author_id(authors:&Vec<String>, author:&str) -> String {
    let mut i = 0;
    for value in authors {
        if author == value {
            return i.to_string();
        }
        i += 1;
    }
    "".to_string()
}
