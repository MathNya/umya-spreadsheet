// c:marker
use super::Symbol;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Marker {
    symbol: Option<Symbol>,
}
impl Marker {
    pub fn get_symbol(&self) -> &Option<Symbol> {
        &self.symbol
    }

    pub fn get_symbol_mut(&mut self) -> &mut Option<Symbol> {
        &mut self.symbol
    }

    pub fn set_symbol(&mut self, value: Symbol) -> &mut Marker {
        self.symbol = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"c:symbol" => {
                        let mut obj = Symbol::default();
                        obj.set_attributes(reader, e);
                        self.set_symbol(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:marker" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:marker"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:marker
        if self.symbol.is_some() {
            write_start_tag(writer, "c:marker", vec![], false);

            // a:symbol
            match &self.symbol {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "c:marker");
        } else {
            write_start_tag(writer, "c:marker", vec![], true);
        }
    }
}
