// a:graphic
use super::GraphicData;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
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

    pub fn set_graphic_data(&mut self, value:GraphicData)-> &mut Graphic {
        self.graphic_data = value;
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
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:graphicData" => {
                            &mut self.graphic_data.set_attributes(reader, e, arv, target);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:graphic" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:graphic"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // a:graphic
        write_start_tag(writer, "a:graphic", vec![], false);

        // a:graphicData
        &self.graphic_data.write_to(writer, r_id);

        write_end_tag(writer, "a:graphic");
    }
}
