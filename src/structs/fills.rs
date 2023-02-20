// fills
use super::Fill;
use super::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct Fills {
    fill: Vec<Fill>,
}
impl Fills {
    pub(crate) fn get_fill(&self) -> &Vec<Fill> {
        &self.fill
    }

    pub(crate) fn get_fill_mut(&mut self) -> &mut Vec<Fill> {
        &mut self.fill
    }

    pub(crate) fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill.push(value);
        self
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        match style.get_fill() {
            Some(v) => {
                let hash_code = v.get_hash_code();
                let mut id = 0;
                for fill in &self.fill {
                    if fill.get_hash_code() == hash_code {
                        return id;
                    }
                    id += 1;
                }
                self.set_fill(v.clone());
                id
            }
            None => 0,
        }
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
                    b"fill" => {
                        let mut obj = Fill::default();
                        obj.set_attributes(reader, e);
                        self.set_fill(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"fills" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "fills"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.fill.is_empty() {
            // fills
            write_start_tag(
                writer,
                "fills",
                vec![("count", &self.fill.len().to_string())],
                false,
            );

            // fill
            for fill in &self.fill {
                fill.write_to(writer);
            }

            write_end_tag(writer, "fills");
        }
    }
}
