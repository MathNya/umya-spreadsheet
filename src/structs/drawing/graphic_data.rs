// *:graphicData
use super::charts::ChartSpace;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use reader::xlsx::chart;
use std::io::Cursor;
use structs::raw::RawRelationships;
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
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:chart" => {
                        let chart_id = get_attribute(e, b"r:id").unwrap();
                        let relationship = drawing_relationships.unwrap().get_relationship_by_rid(&chart_id);
                        let _ = chart::read(relationship.get_raw_file(), &mut self.chart_space);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:graphicData" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:graphicData"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // a:graphicData
        write_start_tag(
            writer,
            "a:graphicData",
            vec![(
                "uri",
                "http://schemas.openxmlformats.org/drawingml/2006/chart",
            )],
            false,
        );

        // c:chart
        write_start_tag(
            writer,
            "c:chart",
            vec![
                (
                    "xmlns:c",
                    "http://schemas.openxmlformats.org/drawingml/2006/chart",
                ),
                (
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                ),
                ("r:id", format!("rId{}", r_id).as_str()),
            ],
            true,
        );

        write_end_tag(writer, "a:graphicData");
    }
}
