use crate::xml_read_loop;

// c:numRef
use super::Formula;
use super::NumberingCache;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NumberReference {
    formula: Formula,
    numbering_cache: NumberingCache,
}

impl NumberReference {
    pub fn get_formula(&self) -> &Formula {
        &self.formula
    }

    pub fn get_formula_mut(&mut self) -> &mut Formula {
        &mut self.formula
    }

    pub fn set_formula(&mut self, value: Formula) -> &mut NumberReference {
        self.formula = value;
        self
    }

    pub fn get_numbering_cache(&self) -> &NumberingCache {
        &self.numbering_cache
    }

    pub fn get_numbering_cache_mut(&mut self) -> &mut NumberingCache {
        &mut self.numbering_cache
    }

    pub fn set_numbering_cache(&mut self, value: NumberingCache) -> &mut NumberReference {
        self.numbering_cache = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().0 {
                b"c:f" => {
                    self.formula.set_attributes(reader, e);
                }
                b"c:numCache" => {
                    self.numbering_cache.set_attributes(reader, e);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:numRef" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:numRef"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:numRef
        write_start_tag(writer, "c:numRef", vec![], false);

        // c:f
        self.formula.write_to(writer);

        // c:numCache
        self.numbering_cache
            .write_to(writer, self.get_formula().get_address(), spreadsheet);

        write_end_tag(writer, "c:numRef");
    }
}
