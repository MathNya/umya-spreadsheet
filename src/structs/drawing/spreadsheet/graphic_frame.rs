// xdr:graphicFrame
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use super::{
    super::{super::StringValue, Graphic},
    NonVisualGraphicFrameProperties, Transform,
};
use crate::{
    reader::driver::{get_attribute, set_string_from_xml, xml_read_loop},
    structs::raw::RawRelationships,
    traits::AdjustmentCoordinateWithSheet,
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct GraphicFrame {
    r#macro: StringValue,
    non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    transform: Transform,
    graphic: Graphic,
}

impl GraphicFrame {
    #[inline]
    #[must_use]
    pub fn get_macro(&self) -> &str {
        self.r#macro.value_str()
    }

    #[inline]
    pub fn set_macro<S: Into<String>>(&mut self, value: S) -> &mut GraphicFrame {
        self.r#macro.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_non_visual_graphic_frame_properties(&self) -> &NonVisualGraphicFrameProperties {
        &self.non_visual_graphic_frame_properties
    }

    #[inline]
    pub fn get_non_visual_graphic_frame_properties_mut(
        &mut self,
    ) -> &mut NonVisualGraphicFrameProperties {
        &mut self.non_visual_graphic_frame_properties
    }

    #[inline]
    pub fn set_non_visual_graphic_frame_properties(
        &mut self,
        value: NonVisualGraphicFrameProperties,
    ) -> &mut Self {
        self.non_visual_graphic_frame_properties = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    #[inline]
    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    #[inline]
    pub fn set_transform(&mut self, value: Transform) -> &mut Self {
        self.transform = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_graphic(&self) -> &Graphic {
        &self.graphic
    }

    #[inline]
    pub fn get_graphic_mut(&mut self) -> &mut Graphic {
        &mut self.graphic
    }

    #[inline]
    pub fn set_graphic(&mut self, value: Graphic) -> &mut Self {
        self.graphic = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, r#macro, "macro");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"xdr:nvGraphicFramePr" => {
                        self.non_visual_graphic_frame_properties
                            .set_attributes(reader, e);
                        }
                    b"xdr:xfrm" => {
                        self.transform.set_attributes(reader, e);
                    }
                    b"a:graphic" => {
                        self.graphic
                            .set_attributes(reader, e, drawing_relationships);
                        }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if  e.name().into_inner() == b"xdr:graphicFrame" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:graphicFrame")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:graphicFrame
        write_start_tag(
            writer,
            "xdr:graphicFrame",
            vec![("macro", self.r#macro.value_str()).into()],
            false,
        );

        // xdr:nvGraphicFramePr
        self.non_visual_graphic_frame_properties.write_to(writer);

        // xdr:xfrm
        self.transform.write_to(writer);

        // a:graphic
        Graphic::write_to(writer, rel_list);

        write_end_tag(writer, "xdr:graphicFrame");
    }
}
impl AdjustmentCoordinateWithSheet for GraphicFrame {
    #[inline]
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.graphic.adjustment_insert_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.graphic.adjustment_remove_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
