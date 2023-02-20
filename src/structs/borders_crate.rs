// borders
use super::Borders;
use super::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct BordersCrate {
    borders: Vec<Borders>,
}
impl BordersCrate {
    pub(crate) fn get_borders(&self) -> &Vec<Borders> {
        &self.borders
    }

    pub(crate) fn get_borders_mut(&mut self) -> &mut Vec<Borders> {
        &mut self.borders
    }

    pub(crate) fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders.push(value);
        self
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        match style.get_borders() {
            Some(v) => {
                let hash_code = v.get_hash_code();
                let mut id = 0;
                for borders in &self.borders {
                    if borders.get_hash_code() == hash_code {
                        return id;
                    }
                    id += 1;
                }
                self.set_borders(v.clone());
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
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"border" => {
                        let obj = Borders::default();
                        self.set_borders(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"border" => {
                        let mut obj = Borders::default();
                        obj.set_attributes(reader, e);
                        self.set_borders(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"borders" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "borders"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.borders.is_empty() {
            // borders
            write_start_tag(
                writer,
                "borders",
                vec![("count", &self.borders.len().to_string())],
                false,
            );

            // border
            for border in &self.borders {
                border.write_to(writer);
            }

            write_end_tag(writer, "borders");
        }
    }
}
