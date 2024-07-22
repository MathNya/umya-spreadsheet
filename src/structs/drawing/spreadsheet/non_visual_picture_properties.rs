//xdr:nvPicPr
use super::NonVisualDrawingProperties;
use super::NonVisualPictureDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
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
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"xdr:cNvPicPr" => {
                        self.non_visual_picture_drawing_properties
                            .set_attributes(reader, e, false);
                        }
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, false);
                        }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"xdr:cNvPicPr" => {
                        self.non_visual_picture_drawing_properties
                            .set_attributes(reader, e, true);
                        }
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, true);
                        }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:nvPicPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:nvPicPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvPicPr
        write_start_tag(writer, "xdr:nvPicPr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvPicPr
        self.non_visual_picture_drawing_properties.write_to(writer);

        write_end_tag(writer, "xdr:nvPicPr");
    }
}
