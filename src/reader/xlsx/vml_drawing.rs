use std::io::Read;
use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use super::driver::*;
use structs::Worksheet;
use structs::vml::Shape;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    worksheet: &mut Worksheet,
) -> result::Result<(), XlsxError> {
    let data = {
        let path_str = normalize_path_to_str(&format!("xl/drawings/{}", target));
        let mut r = io::BufReader::new(arv.by_name(path_str.as_str())?);
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        std::io::Cursor::new(buf)
    };
    let mut reader = Reader::from_reader(data);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut ole_index = 0;
    let mut comment_index = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"v:shape" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(&mut reader, e, arv, target);
                        match obj.get_client_data().get_comment_column_target() {
                            Some(_) => {
                                worksheet.get_comments_mut()[comment_index].set_shape(obj);
                                comment_index += 1;
                            },
                            None => {
                                worksheet.get_ole_objects_mut().get_ole_object_mut()[ole_index].set_shape(obj);
                                ole_index += 1;
                            }
                        }
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

    Ok(())
}

//fn set_style(comment:&mut Comment, style_string:&str) {
//    let styles: Vec<&str> = style_string.split(';').collect();
//    for style in &styles {
//        let params: Vec<&str> = style.split(':').collect();
//        if params.len() == 2 {
//            let key_string = params[0].replace(" ", "").replace("\r\n", "");
//            let key = key_string.as_str();
//            let value = params[1].replace(" ", "").replace("\r\n", "");
//            match key {
//                "margin-left" => comment.set_margin_left(value),
//                "margin-top" => comment.set_margin_top(value),
//                "width" => comment.set_width(value),
//                "height" => comment.set_height(value),
//                "visibility" => comment.set_visible(if value == "visible" { true } else { false }),
//                _ => {}
//            }
//        }
//    }
//}
