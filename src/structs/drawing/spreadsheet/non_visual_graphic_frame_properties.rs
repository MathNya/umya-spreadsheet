// xdr:nvGraphicFramePr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    NonVisualDrawingProperties,
    NonVisualGraphicFrameDrawingProperties,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct NonVisualGraphicFrameProperties {
    non_visual_drawing_properties:               NonVisualDrawingProperties,
    non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}

impl NonVisualGraphicFrameProperties {
    #[must_use]
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

    #[must_use]
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
                        NonVisualGraphicFrameDrawingProperties::set_attributes(reader, e);
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
        self.non_visual_drawing_properties.write_to(writer, 0);

        // xdr:cNvGraphicFramePr
        NonVisualGraphicFrameDrawingProperties::write_to(writer);

        write_end_tag(writer, "xdr:nvGraphicFramePr");
    }
}
