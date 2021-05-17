// a:ln
use super::tail_end::TailEnd;
use super::solid_fill::SolidFill;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Outline {
    width: u32,
    solid_fill: Option<SolidFill>,
    tail_end: Option<TailEnd>,
}
impl Outline {
    pub fn get_width(&self) -> &u32 {
        &self.width
    }

    pub fn set_width(&mut self, value:u32) {
        self.width = value;
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value:SolidFill) {
        self.solid_fill = Some(value);
    }

    pub fn get_tail_end(&self) -> &Option<TailEnd> {
        &self.tail_end
    }

    pub fn get_tail_end_mut(&mut self) -> &mut Option<TailEnd> {
        &mut self.tail_end
    }

    pub fn set_tail_end(&mut self, value:TailEnd) {
        self.tail_end = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        let mut buf = Vec::new();
    
        match get_attribute(e, b"w") {
            Some(v) => {&mut self.set_width(v.parse::<u32>().unwrap());},
            None => {}
        }
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:solidFill" => {
                            let mut solid_fill = SolidFill::default();
                            solid_fill.set_attributes(reader, e);
                            &mut self.set_solid_fill(solid_fill);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:tailEnd" => {
                            let mut tail_end = TailEnd::default();
                            tail_end.set_attributes(reader, e);
                            &mut self.set_tail_end(tail_end);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:ln" => {
                            return;
                        },
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:ln"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ln
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width_str = &self.width.to_string();
        if &self.width > &0 {
            attributes.push(("w", width_str.as_str()))
        }
        write_start_tag(writer, "a:ln", attributes, false);

        // a:solidFill
        match &self.solid_fill {
            Some(v) => v.write_to(writer),
            None => {},
        }

        // a:tailEnd
        match &self.tail_end {
            Some(v) => v.write_to(writer),
            None => {},
        }

        write_end_tag(writer, "a:ln");
    }
}
