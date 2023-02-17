// xdr:nvCxnSpPr
use super::NonVisualConnectorShapeDrawingProperties;
use super::NonVisualDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualConnectionShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
}
impl NonVisualConnectionShapeProperties {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(
        &mut self,
        value: NonVisualDrawingProperties,
    ) -> &mut NonVisualConnectionShapeProperties {
        self.non_visual_drawing_properties = value;
        self
    }

    pub fn get_non_visual_connector_shape_drawing_properties(
        &self,
    ) -> &NonVisualConnectorShapeDrawingProperties {
        &self.non_visual_connector_shape_drawing_properties
    }

    pub fn get_non_visual_connector_shape_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualConnectorShapeDrawingProperties {
        &mut self.non_visual_connector_shape_drawing_properties
    }

    pub fn set_non_visual_connector_shape_drawing_properties(
        &mut self,
        value: NonVisualConnectorShapeDrawingProperties,
    ) -> &mut NonVisualConnectionShapeProperties {
        self.non_visual_connector_shape_drawing_properties = value;
        self
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
                    b"xdr:cNvCxnSpPr" => {
                        self.non_visual_connector_shape_drawing_properties
                            .set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"xdr:cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, true);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:nvCxnSpPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:nvCxnSpPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvCxnSpPr
        write_start_tag(writer, "xdr:nvCxnSpPr", vec![], false);

        // xdr:cNvPr
        let _ = &self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvCxnSpPr
        let _ = &self
            .non_visual_connector_shape_drawing_properties
            .write_to(writer);

        write_end_tag(writer, "xdr:nvCxnSpPr");
    }
}
