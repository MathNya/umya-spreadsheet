// rowFields
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
    Field, reader::driver::xml_read_loop, writer::driver::{
        write_end_tag,
        write_start_tag,
    }
};

#[derive(Clone, Default, Debug)]
pub struct RowFields {
    list: Vec<Field>,
}
impl RowFields {
    #[inline]
    #[must_use]
    pub fn list(&self) -> &[Field] {
        &self.list
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use list()")]
    pub fn get_list(&self) -> &[Field] {
        self.list()
    }

    #[inline]
    pub fn list_mut(&mut self) -> &mut Vec<Field> {
        &mut self.list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use list_mut()")]
    pub fn get_list_mut(&mut self) -> &mut Vec<Field> {
        self.list_mut()
    }
    
    #[inline]
    pub fn add_list_mut(&mut self, value: Field) -> &mut Self {
        self.list.push(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
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
                if e.name().into_inner() == b"rowFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "rowFields")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.list.is_empty() {
            // rowFields
            write_start_tag(
                writer,
                "rowFields",
                vec![("count", self.list.len().to_string()).into()],
                false,
            );

            // field
            for obj in &self.list {
                obj.write_to(writer);
            }

            write_end_tag(writer, "rowFields");
        }
    }
}
