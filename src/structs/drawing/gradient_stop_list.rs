// a:gsLst
use super::GradientStop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct GradientStopList {
    gradient_stop: Vec<GradientStop>,
}
impl GradientStopList {
    pub fn get_gradient_stop(&self) -> &Vec<GradientStop> {
        &self.gradient_stop
    }

    pub fn get_gradient_stop_mut(&mut self) -> &mut Vec<GradientStop> {
        &mut self.gradient_stop
    }

    pub fn set_gradient_stop(&mut self, value: Vec<GradientStop>) -> &mut GradientStopList {
        self.gradient_stop = value;
        self
    }

    pub fn add_gradient_stop(&mut self, value: GradientStop) -> &mut GradientStopList {
        self.gradient_stop.push(value);
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
                    b"a:gs" => {
                        let mut obj = GradientStop::default();
                        obj.set_attributes(reader, e);
                        self.add_gradient_stop(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:gsLst" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:gsLst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:gsLst
        write_start_tag(writer, "a:gsLst", vec![], false);

        // a:gs
        for v in &self.gradient_stop {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:gsLst");
    }
}
