// c:strRef
use super::Formula;
use super::StringCache;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct StringReference {
    formula: Formula,
    string_cache: StringCache,
}
impl StringReference {
    pub fn get_formula(&self)-> &Formula {
        &self.formula
    }

    pub fn get_formula_mut(&mut self)-> &mut Formula {
        &mut self.formula
    }

    pub fn set_formula(&mut self, value:Formula)-> &mut StringReference {
        self.formula = value;
        self
    }

    pub fn get_string_cache(&self)-> &StringCache {
        &self.string_cache
    }

    pub fn get_string_cache_mut(&mut self)-> &mut StringCache {
        &mut self.string_cache
    }

    pub fn set_string_cache(&mut self, value:StringCache)-> &mut StringReference {
        self.string_cache = value;
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:f" => {
                            self.formula.set_attributes(reader, e);
                        },
                        b"c:strCache" => {
                            self.string_cache.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:strRef" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:strRef"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:strRef
        write_start_tag(writer, "c:strRef", vec![], false);

        // c:f
        &self.formula.write_to(writer);

        // c:strCache
        &self.string_cache.write_to(writer);

        write_end_tag(writer, "c:strRef");
    }
}
