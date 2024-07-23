// a:graphic
use super::GraphicData;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Graphic {
    graphic_data: GraphicData,
}

impl Graphic {
    pub fn get_graphic_data(&self) -> &GraphicData {
        &self.graphic_data
    }

    pub fn get_graphic_data_mut(&mut self) -> &mut GraphicData {
        &mut self.graphic_data
    }

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
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // a:graphic
        write_start_tag(writer, "a:graphic", vec![], false);

        // a:graphicData
        self.graphic_data.write_to(writer, rel_list);

        write_end_tag(writer, "a:graphic");
    }
}
impl AdjustmentCoordinateWithSheet for Graphic {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.graphic_data.adjustment_insert_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
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
