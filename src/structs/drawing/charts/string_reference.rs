use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

// c:strRef
use super::Formula;
use super::StringCache;
use crate::{
    structs::Workbook,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct StringReference {
    formula:      Formula,
    string_cache: StringCache,
}

impl StringReference {
    #[must_use]
    pub fn formula(&self) -> &Formula {
        &self.formula
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use formula()")]
    pub fn get_formula(&self) -> &Formula {
        self.formula()
    }

    pub fn formula_mut(&mut self) -> &mut Formula {
        &mut self.formula
    }

    #[deprecated(since = "3.0.0", note = "Use formula_mut()")]
    pub fn get_formula_mut(&mut self) -> &mut Formula {
        self.formula_mut()
    }

    pub fn set_formula(&mut self, value: Formula) -> &mut StringReference {
        self.formula = value;
        self
    }

    #[must_use]
    pub fn string_cache(&self) -> &StringCache {
        &self.string_cache
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use string_cache()")]
    pub fn get_string_cache(&self) -> &StringCache {
        self.string_cache()
    }

    pub fn string_cache_mut(&mut self) -> &mut StringCache {
        &mut self.string_cache
    }

    #[deprecated(since = "3.0.0", note = "Use string_cache_mut()")]
    pub fn get_string_cache_mut(&mut self) -> &mut StringCache {
        self.string_cache_mut()
    }

    pub fn set_string_cache(&mut self, value: StringCache) -> &mut StringReference {
        self.string_cache = value;
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
                b"c:strCache" => {
                    StringCache::set_attributes(reader, e);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:strRef" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:strRef"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:strRef
        write_start_tag(writer, "c:strRef", vec![], false);

        // c:f
        self.formula.write_to(writer);

        // c:strCache
        StringCache::write_to(writer, self.formula().address(), wb);

        write_end_tag(writer, "c:strRef");
    }
}
