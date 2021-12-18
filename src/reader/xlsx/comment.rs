use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use std::collections::HashMap; 
use super::driver::*;
use structs::Theme;
use structs::Worksheet;
use structs::Comment;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    worksheet: &mut Worksheet,
    comment_list: &mut HashMap<String, Comment>,
    _theme: &Theme
) -> result::Result<(), XlsxError> {
    let path_str = normalize_path_to_str(&format!("xl/worksheets/{}", target));
    let r = io::BufReader::new(arv.by_name(path_str.as_str())?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(false);
    let mut buf = Vec::new();

    let mut authors: Vec<String> = Vec::new();
    let mut comment: Comment = Comment::default();
    let mut value: String = String::from("");

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
                    b"text" => {
                        comment.get_text_mut().set_attributes_text(&mut reader, e);
                    },
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
