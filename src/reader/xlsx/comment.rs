use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use std::collections::HashMap; 
use super::driver::*;
use structs::TextElement;
use structs::Theme;
use structs::RichText;
use structs::Worksheet;
use structs::Comment;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
    worksheet: &mut Worksheet,
    comment_list: &mut HashMap<String, Comment>,
    theme: &Theme
) -> result::Result<(), XlsxError> {
    let path = dir.path().join(format!("xl/worksheets/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut authors: Vec<String> = Vec::new();
    let mut comment: Comment = Comment::default();
    let mut value: String = String::from("");

    let mut text_element_vec: Vec<TextElement> = Vec::new();
    let mut result: Vec<Comment> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"comment" => {
                        let coordinate = get_attribute(e, b"ref").unwrap();
                        comment = comment_list.get_mut(&coordinate).unwrap().clone();
                        let author_id = get_attribute(e, b"authorId").unwrap().parse::<usize>().unwrap();
                        let author = authors.get(author_id).unwrap();
                        comment.set_author(author);
                    },
                    b"r" => text_element_vec.push(get_text_element(&mut reader, theme)),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                value = e.unescape_and_decode(&reader).unwrap();
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"author" => {
                        authors.push(value.clone());
                    },
                    b"text" => {
                        let mut rich_text = RichText::default();
                        rich_text.set_rich_text_elements(text_element_vec);
                        comment.set_text(rich_text);
                        text_element_vec = Vec::new();
                    },
                    b"comment"=> {
                        result.push(comment);
                        comment = Comment::default();
                    }
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    worksheet.set_comments(result);
    Ok(())
}
