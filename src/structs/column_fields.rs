// colFields
use structs::Field;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ColumnFields {
    list: Vec<Field>,
}
impl ColumnFields {
    pub fn get_list(&self) -> &Vec<Field> {
        &self.list
    }

    pub fn get_list_mut(&mut self) -> &mut Vec<Field> {
        &mut self.list
    }

    pub fn add_list_mut(&mut self, value: Field) -> &mut Self {
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
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"field" {
                    let mut obj = Field::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colFields")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // colFields
        write_start_tag(writer, "colFields", vec![
            ("count", self.list.len().to_string().as_str())
        ], false);

        // i
        for i in &self.list {
            i.write_to(writer);
        }

        write_end_tag(writer, "colFields");
    }
}
