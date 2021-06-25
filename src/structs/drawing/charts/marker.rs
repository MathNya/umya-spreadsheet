// c:marker
use super::Symbol;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Marker {
    val: Option<String>,
    symbol: Option<Symbol>,
}
impl Marker {
    pub fn get_val(&self)-> &Option<String> {
        &self.val
    }

    pub fn set_val<S: Into<String>>(&mut self, value:S)-> &mut Marker {
        self.val = Some(value.into());
        self
    }

    pub fn get_symbol(&self)-> &Option<Symbol> {
        &self.symbol
    }

    pub fn get_symbol_mut(&mut self)-> &mut Option<Symbol> {
        &mut self.symbol
    }

    pub fn set_symbol(&mut self, value:Symbol)-> &mut Marker {
        self.symbol = Some(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        empty_flag:bool,
    ) {
        match get_attribute(e, b"val") {
            Some(v) => {self.set_val(v);},
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:symbol" => {
                            let mut obj = Symbol::default();
                            obj.set_attributes(reader, e);
                            self.set_symbol(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:marker" => return,
                        _ => (),
                    }
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.val {
            Some(v) => attributes.push(("val", v)),
            None => {}
        }
        if self.symbol.is_some() {
            write_start_tag(writer, "c:marker", attributes, false);

            // a:symbol
            match &self.symbol {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            write_end_tag(writer, "c:marker");
        } else {
            write_start_tag(writer, "c:marker", attributes, true);
        }
    }
}
