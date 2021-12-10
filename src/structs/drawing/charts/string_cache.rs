// c:strCache
use super::PointCount;
use super::StringPoint;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct StringCache {
    point_count: PointCount,
    string_point: Vec<StringPoint>,

}
impl StringCache {
    pub fn get_point_count(&self)-> &PointCount {
        &self.point_count
    }

    pub fn get_point_count_mut(&mut self)-> &mut PointCount {
        &mut self.point_count
    }

    pub fn set_point_count(&mut self, value:PointCount)-> &mut StringCache {
        self.point_count = value;
        self
    }

    pub fn get_string_point(&self)-> &Vec<StringPoint> {
        &self.string_point
    }

    pub fn get_string_point_mut(&mut self)-> &mut Vec<StringPoint> {
        &mut self.string_point
    }

    pub fn set_string_point(&mut self, value:Vec<StringPoint>)-> &mut StringCache {
        self.string_point = value;
        self
    }

    pub fn add_string_point(&mut self, value:StringPoint)-> &mut StringCache {
        self.string_point.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:pt" => {
                            let mut obj = StringPoint::default();
                            obj.set_attributes(reader, e);
                            self.add_string_point(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:ptCount" => {
                            self.point_count.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:strCache" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:strCache"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:strCache
        write_start_tag(writer, "c:strCache", vec![], false);

        // c:ptCount
        &self.point_count.write_to(writer);

        // c:pt
        for v in &self.string_point {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:strCache");
    }
}
