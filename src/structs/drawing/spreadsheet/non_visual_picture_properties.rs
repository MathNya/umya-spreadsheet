//xdr:nvPicPr
use super::NonVisualDrawingProperties;
use super::NonVisualPictureDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualPictureProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
}
impl NonVisualPictureProperties {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value: NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }

    pub fn get_non_visual_picture_drawing_properties(&self) -> &NonVisualPictureDrawingProperties {
        &self.non_visual_picture_drawing_properties
    }

    pub fn get_non_visual_picture_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualPictureDrawingProperties {
        &mut self.non_visual_picture_drawing_properties
    }

    pub fn set_non_visual_picture_drawing_properties(
        &mut self,
        value: NonVisualPictureDrawingProperties,
    ) {
        self.non_visual_picture_drawing_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"xdr:cNvPicPr" => {
                        self.non_visual_picture_drawing_properties
                            .set_attributes(reader, e, false);
                    }
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, false);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"xdr:cNvPicPr" => {
                        self.non_visual_picture_drawing_properties
                            .set_attributes(reader, e, true);
                    }
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, true);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:nvPicPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:nvPicPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvPicPr
        write_start_tag(writer, "xdr:nvPicPr", vec![], false);

        // xdr:cNvPr
        let _ = &self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvPicPr
        let _ = &self.non_visual_picture_drawing_properties.write_to(writer);

        write_end_tag(writer, "xdr:nvPicPr");
    }
}
