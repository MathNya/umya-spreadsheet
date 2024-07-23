// c:marker
use super::Symbol;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Marker {
    symbol: Option<Symbol>,
}

impl Marker {
    pub fn get_symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }

    pub fn get_symbol_mut(&mut self) -> Option<&mut Symbol> {
        self.symbol.as_mut()
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

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().0 == b"c:symbol" {
                    let mut obj = Symbol::default();
                    obj.set_attributes(reader, e);
                    self.set_symbol(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:marker" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:marker")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:marker
        if self.symbol.is_some() {
            write_start_tag(writer, "c:marker", vec![], false);

            // a:symbol
            if let Some(v) = &self.symbol {
                v.write_to(writer);
            }

            write_end_tag(writer, "c:marker");
        } else {
            write_start_tag(writer, "c:marker", vec![], true);
        }
    }
}
