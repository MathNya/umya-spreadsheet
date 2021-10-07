// xdr:nvGraphicFramePr
use super::NonVisualDrawingProperties;
use super::NonVisualGraphicFrameDrawingProperties;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NonVisualGraphicFrameProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}
impl NonVisualGraphicFrameProperties {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value:NonVisualDrawingProperties) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_drawing_properties = value;
        self
    }

    pub fn get_non_visual_graphic_frame_drawing_properties(&self) -> &NonVisualGraphicFrameDrawingProperties {
        &self.non_visual_graphic_frame_drawing_properties
    }

    pub fn get_non_visual_graphic_frame_drawing_properties_mut(&mut self) -> &mut NonVisualGraphicFrameDrawingProperties {
        &mut self.non_visual_graphic_frame_drawing_properties
    }

    pub fn set_non_visual_graphic_frame_drawing_properties(&mut self, value:NonVisualGraphicFrameDrawingProperties) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_graphic_frame_drawing_properties = value;
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"xdr:cNvPr" => {
                            &mut self.non_visual_drawing_properties.set_attributes(reader, e);
                        },
                        b"xdr:cNvGraphicFramePr" => {
                            &mut self.non_visual_graphic_frame_drawing_properties.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:nvGraphicFramePr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:nvGraphicFramePr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvGraphicFramePr
        write_start_tag(writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        &self.non_visual_drawing_properties.write_to(writer);
        
        // xdr:cNvGraphicFramePr
        &self.non_visual_graphic_frame_drawing_properties.write_to(writer);

        write_end_tag(writer, "xdr:nvGraphicFramePr");
    }
}
