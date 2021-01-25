use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;
use std::collections::HashMap;
use super::super::structs::comment::Comment;
use super::super::structs::color::Color;
use super::super::structs::anchor::Anchor;
use super::super::helper::coordinate::*;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
) -> result::Result<HashMap<String, Comment>, XlsxError> {
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result:HashMap<String, Comment> = HashMap::new();
    let mut string_value: String = String::from("");
    let mut row:usize = 0;
    let mut col_str:String = String::from("");
    let mut comment = Comment::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"v:shape" => {
                        let style =  get_attribute(e, b"style").unwrap();
                        let fillcolor =  get_attribute(e, b"fillcolor").unwrap();
                        set_style(&mut comment, &style);
                        let mut color = Color::default();
                        let _ = color.set_argb(fillcolor.replace("#", ""));
                        comment.set_fill_color(color);
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"x:Anchor" => {
                        let split_str:Vec<&str> = string_value.split(", ").collect();
                        let mut anchor = Anchor::default();
                        anchor.set_left_column(split_str.get(0).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_left_offset(split_str.get(1).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_top_row(split_str.get(2).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_top_offset(split_str.get(3).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_right_column(split_str.get(4).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_right_offset(split_str.get(5).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_bottom_row(split_str.get(6).unwrap().to_string().parse::<usize>().unwrap());
                        anchor.set_bottom_offset(split_str.get(7).unwrap().to_string().parse::<usize>().unwrap());
                        comment.set_anchor(anchor);
                    }
                    b"x:Row" => {
                        row = string_value.parse::<usize>().unwrap() + 1;
                        comment.get_coordinate_mut().set_row_num(&row);
                    },
                    b"x:Column" => {
                        let col = string_value.parse::<usize>().unwrap() + 1;
                        col_str = string_from_column_index(&col);
                        comment.get_coordinate_mut().set_col_num(&col);
                    },
                    b"v:shape" => {
                        let coordinate = format!("{}{}", col_str, row);
                        result.insert(coordinate, comment);
                        comment = Comment::default();
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(result)
}

fn set_style(comment:&mut Comment, style_string:&str) {
    let styles: Vec<&str> = style_string.split(';').collect();
    for style in &styles {
        let params: Vec<&str> = style.split(':').collect();
        if params.len() == 2 {
            let key_string = params[0].replace(" ", "").replace("\r\n", "");
            let key = key_string.as_str();
            let value = params[1].replace(" ", "").replace("\r\n", "");
            match key {
                "margin-left" => comment.set_margin_left(value),
                "margin-top" => comment.set_margin_top(value),
                "width" => comment.set_width(value),
                "height" => comment.set_height(value),
                "visibility" => comment.set_visible(if value == "visible" { true } else { false }),
                _ => {}
            }
        }
    }
}
