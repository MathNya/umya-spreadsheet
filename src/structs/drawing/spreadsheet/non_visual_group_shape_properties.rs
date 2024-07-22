// xdr:nvGrpSpPr
use super::NonVisualDrawingProperties;
use super::NonVisualGroupShapeDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualGroupShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_group_shape_drawing_properties: NonVisualGroupShapeDrawingProperties,
}

impl NonVisualGroupShapeProperties {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(
        &mut self,
        value: NonVisualDrawingProperties,
    ) -> &mut Self {
        self.non_visual_drawing_properties = value;
        self
    }

    pub fn get_non_visual_group_shape_drawing_properties(
        &self,
    ) -> &NonVisualGroupShapeDrawingProperties {
        &self.non_visual_group_shape_drawing_properties
    }

    pub fn get_non_visual_group_shape_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualGroupShapeDrawingProperties {
        &mut self.non_visual_group_shape_drawing_properties
    }

    pub fn set_non_visual_group_shape_drawing_properties(
        &mut self,
        value: NonVisualGroupShapeDrawingProperties,
    ) -> &mut Self {
        self.non_visual_group_shape_drawing_properties = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner(){
                    b"xdr:cNvPr" =>{
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, false);
                    }
                    b"a:cNvGrpSpPr"=> {
                        self.non_visual_group_shape_drawing_properties
                            .set_attributes(reader, e, false);
                    }
                    _=>()
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"xdr:cNvPr" =>{
                        self.non_visual_drawing_properties
                        .set_attributes(reader, e, true);
                    }
                    b"a:cNvGrpSpPr" =>{
                        self.non_visual_group_shape_drawing_properties
                        .set_attributes(reader, e, true);
                    }
                    _ => ()
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:nvGrpSpPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:nvGrpSpPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvGrpSpPr
        write_start_tag(writer, "xdr:nvGrpSpPr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvGrpSpPr
        self.non_visual_group_shape_drawing_properties
            .write_to(writer);

        write_end_tag(writer, "xdr:nvGrpSpPr");
    }
}
