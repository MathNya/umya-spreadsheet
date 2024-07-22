// xdr:nvGraphicFramePr
use super::NonVisualDrawingProperties;
use super::NonVisualGraphicFrameDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
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

    pub fn set_non_visual_drawing_properties(
        &mut self,
        value: NonVisualDrawingProperties,
    ) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_drawing_properties = value;
        self
    }

    pub fn get_non_visual_graphic_frame_drawing_properties(
        &self,
    ) -> &NonVisualGraphicFrameDrawingProperties {
        &self.non_visual_graphic_frame_drawing_properties
    }

    pub fn get_non_visual_graphic_frame_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualGraphicFrameDrawingProperties {
        &mut self.non_visual_graphic_frame_drawing_properties
    }

    pub fn set_non_visual_graphic_frame_drawing_properties(
        &mut self,
        value: NonVisualGraphicFrameDrawingProperties,
    ) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_graphic_frame_drawing_properties = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, true);
                    },
                    b"xdr:cNvGraphicFramePr" => {
                        self.non_visual_graphic_frame_drawing_properties
                            .set_attributes(reader, e);
                    },
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"xdr:cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, false);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:nvGraphicFramePr" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:nvGraphicFramePr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvGraphicFramePr
        write_start_tag(writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvGraphicFramePr
        self.non_visual_graphic_frame_drawing_properties
            .write_to(writer);

        write_end_tag(writer, "xdr:nvGraphicFramePr");
    }
}
