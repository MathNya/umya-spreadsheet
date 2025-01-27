// cacheFields
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::xml_read_loop,
    structs::CacheField,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct CacheFields {
    list: Vec<CacheField>,
}
impl CacheFields {
    #[inline]
    #[must_use]
    pub fn list(&self) -> &Vec<CacheField> {
        &self.list
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use list()")]
    pub fn get_list(&self) -> &Vec<CacheField> {
        self.list()
    }

    #[inline]
    pub fn list_mut(&mut self) -> &mut Vec<CacheField> {
        &mut self.list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use list_mut()")]
    pub fn get_list_mut(&mut self) -> &mut Vec<CacheField> {
        self.list_mut()
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: CacheField) -> &mut Self {
        self.list.push(value);
        self
    }

    #[inline]
    #[allow(dead_code, unused_variables)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"cacheField" {
                    let mut obj = CacheField::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cacheFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cacheFields")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cacheFields
        write_start_tag(
            writer,
            "cacheFields",
            vec![("count", self.list.len().to_string()).into()],
            false,
        );

        // cacheField
        for sheet_view in &self.list {
            sheet_view.write_to(writer);
        }

        write_end_tag(writer, "cacheFields");
    }
}
