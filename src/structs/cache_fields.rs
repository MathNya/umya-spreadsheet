// cacheFields
use structs::BooleanValue;
use structs::StringValue;
use structs::UInt32Value;
use structs::ByteValue;
use structs::CacheField;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CacheFields {
    list: Vec<CacheField>,
}
impl CacheFields {
    pub fn get_list(&self) -> &Vec<CacheField> {
        &self.list
    }

    pub fn get_list_mut(&mut self) -> &mut Vec<CacheField> {
        &mut self.list
    }

    pub fn add_list_mut(&mut self, value: CacheField) -> &mut Self {
        self.list.push(value);
        self
    }

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

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cacheFields
        write_start_tag(writer, "cacheFields", vec![
            ("count", self.list.len().to_string().as_str())
        ], false);

        // cacheField
        for sheet_view in &self.list {
            sheet_view.write_to(writer);
        }

        write_end_tag(writer, "cacheFields");
    }
}
