// a:graphic
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::GraphicData;
use crate::{
    reader::driver::xml_read_loop,
    structs::raw::RawRelationships,
    traits::AdjustmentCoordinateWithSheet,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Graphic {
    graphic_data: GraphicData,
}

impl Graphic {
    #[inline]
    #[must_use]
    pub fn graphic_data(&self) -> &GraphicData {
        &self.graphic_data
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use graphic_data()")]
    pub fn get_graphic_data(&self) -> &GraphicData {
        self.graphic_data()
    }

    #[inline]
    pub fn graphic_data_mut(&mut self) -> &mut GraphicData {
        &mut self.graphic_data
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use graphic_data_mut()")]
    pub fn get_graphic_data_mut(&mut self) -> &mut GraphicData {
        self.graphic_data_mut()
    }

    #[inline]
    pub fn set_graphic_data(&mut self, value: GraphicData) -> &mut Self {
        self.graphic_data = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:graphicData" {
                    self.graphic_data
                        .set_attributes(reader, e, drawing_relationships);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:graphic" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:graphic")
        );
    }

    pub(crate) fn write_to(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // a:graphic
        write_start_tag(writer, "a:graphic", vec![], false);

        // a:graphicData
        GraphicData::write_to(writer, rel_list);

        write_end_tag(writer, "a:graphic");
    }
}
impl AdjustmentCoordinateWithSheet for Graphic {
    #[inline]
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.graphic_data.adjustment_insert_coordinate_with_sheet(
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
        self.graphic_data.adjustment_remove_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
