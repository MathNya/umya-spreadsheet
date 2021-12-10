// *:graphicData
use super::charts::ChartSpace;
use reader::xlsx::drawing_rels;
use reader::xlsx::chart;
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
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

    pub fn set_chart_space(&mut self, value:ChartSpace)-> &GraphicData {
        self.chart_space = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        target: &str,
    ) {
        let mut buf = Vec::new();
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:chart" => {
                            let chart_id = get_attribute(e, b"r:id").unwrap();
                            let drawing_rel = drawing_rels::read(arv, target).unwrap();
                            for (drawing_id, _, drawing_target) in &drawing_rel {
                                if &chart_id == drawing_id {
                                    let _ = chart::read(arv, &drawing_target, &mut self.chart_space);
                                }
                            }
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:graphicData" => return,
                        _ => (),
                    }
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
        write_start_tag(writer, "a:graphicData", vec![
            ("uri", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
        ], false);

        // c:chart
        write_start_tag(writer, "c:chart", vec![
            ("xmlns:c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
            ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
            ("r:id", format!("rId{}", r_id).as_str()),
        ], true);

        write_end_tag(writer, "a:graphicData");
    }
}
