use super::XlsxError;
use crate::structs::raw::RawFile;
use crate::structs::raw::RawRelationships;
use crate::structs::vml::Shape;
use crate::structs::Worksheet;
use crate::xml_read_loop;
use quick_xml::events::Event;
use quick_xml::Reader;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
    drawing_relationships: Option<&RawRelationships>,
) -> Result<(), XlsxError> {
    let data = std::io::Cursor::new(drawing_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);

    let mut ole_index = 0;
    let mut comment_index = 0;

    xml_read_loop!(
        reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"v:shape" {
                    let mut obj = Shape::default();
                    obj.set_attributes(&mut reader, e, drawing_relationships);
                    if obj.get_client_data().get_comment_column_target().is_some() {
                        worksheet
                            .get_comments_mut()
                            .get_mut(comment_index)
                            .map(|comment| comment.set_shape(obj));
                        comment_index += 1;
                    } else {
                        worksheet
                            .get_ole_objects_mut()
                            .get_ole_object_mut()
                            .get_mut(ole_index)
                            .map(|ole_obj| ole_obj.set_shape(obj));
                        ole_index += 1;
                    }
                }
            },
            Event::Eof => break,
    );

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
