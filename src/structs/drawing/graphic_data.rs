// *:graphicData
use super::charts::ChartSpace;
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use reader::xlsx::chart;
use std::io::Cursor;
use structs::raw::RawRelationships;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct GraphicData {
    chart_space: ChartSpace,
}

impl GraphicData {
    pub fn get_chart_space(&self) -> &ChartSpace {
        &self.chart_space
    }

    pub fn get_chart_space_mut(&mut self) -> &mut ChartSpace {
        &mut self.chart_space
    }

    pub fn set_chart_space(&mut self, value: ChartSpace) -> &GraphicData {
        self.chart_space = value;
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
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"c:chart" {
                    let chart_id = get_attribute(e, b"r:id").unwrap();
                    let relationship = drawing_relationships
                        .unwrap()
                        .get_relationship_by_rid(&chart_id);
                    chart::read(relationship.get_raw_file(), &mut self.chart_space);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:graphicData" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:graphicData")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // a:graphicData
        write_start_tag(
            writer,
            "a:graphicData",
            vec![("uri", DRAWINGML_CHART_NS)],
            false,
        );

        // c:chart
        rel_list.push((String::from("CHART"), String::from("")));
        write_start_tag(
            writer,
            "c:chart",
            vec![
                ("xmlns:c", DRAWINGML_CHART_NS),
                ("xmlns:r", REL_OFC_NS),
                ("r:id", format!("rId{}", rel_list.len()).as_str()),
            ],
            true,
        );

        write_end_tag(writer, "a:graphicData");
    }
}
impl AdjustmentCoordinateWithSheet for GraphicData {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.chart_space.adjustment_insert_coordinate_with_sheet(
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
        self.chart_space.adjustment_remove_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
