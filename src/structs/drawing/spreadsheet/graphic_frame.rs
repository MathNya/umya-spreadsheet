// xdr:graphicFrame
use super::super::super::StringValue;
use super::NonVisualGraphicFrameProperties;
use super::Transform;
use super::super::Graphic;

use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;
use tempdir::TempDir;

#[derive(Default, Debug)]
pub struct GraphicFrame {
    r#macro: StringValue,
    non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    transform: Transform,
    graphic: Graphic,
}
impl GraphicFrame {
    pub fn get_macro(&self) -> &str {
        &self.r#macro.get_value()
    }

    pub fn set_macro<S: Into<String>>(&mut self, value:S) -> &mut GraphicFrame {
        self.r#macro.set_value(value);
        self
    }

    pub fn get_non_visual_graphic_frame_properties(&self) -> &NonVisualGraphicFrameProperties {
        &self.non_visual_graphic_frame_properties
    }

    pub fn get_non_visual_graphic_frame_properties_mut(&mut self) -> &NonVisualGraphicFrameProperties {
        &mut self.non_visual_graphic_frame_properties
    }

    pub fn set_non_visual_graphic_frame_properties(&mut self, value:NonVisualGraphicFrameProperties)-> &mut GraphicFrame {
        self.non_visual_graphic_frame_properties = value;
        self
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, value:Transform)-> &mut GraphicFrame {
        self.transform = value;
        self
    }

    pub fn get_graphic(&self) -> &Graphic {
        &self.graphic
    }

    pub fn get_graphic_mut(&mut self) -> &mut Graphic {
        &mut self.graphic
    }

    pub fn set_graphic(&mut self, value:Graphic)-> &mut GraphicFrame {
        self.graphic = value;
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        dir: &TempDir,
        target: &str
    ) {
        match get_attribute(e, b"macro") {
            Some(v) => {&mut self.r#macro.set_value_string(v);},
            None => {}
        }

        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"xdr:nvGraphicFramePr" => {
                            &mut self.non_visual_graphic_frame_properties.set_attributes(reader, e);
                        },
                        b"a:xfrm" => {
                            &mut self.transform.set_attributes(reader, e);
                        },
                        b"a:graphic" => {
                            &mut self.graphic.set_attributes(reader, e, dir, target);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:graphicFrame" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:graphicFrame"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // xdr:graphicFrame
        write_start_tag(writer, "xdr:graphicFrame", vec![
            ("macro", &self.r#macro.get_value_string()),
        ], false);
        
        // xdr:nvGraphicFramePr
        &self.non_visual_graphic_frame_properties.write_to(writer);

        // xdr:xfrm
        &self.transform.write_to(writer);

        // a:graphic
        &self.graphic.write_to(writer, r_id);

        write_end_tag(writer, "xdr:graphicFrame");
    }
}
