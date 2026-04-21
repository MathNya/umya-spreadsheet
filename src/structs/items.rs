// items
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
    structs::Item,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Items {
    list: Vec<Item>,
}
impl Items {
    #[inline]
    #[must_use]
    pub fn list(&self) -> &[Item] {
        &self.list
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use list()")]
    pub fn get_list(&self) -> &[Item] {
        self.list()
    }

    #[inline]
    pub fn list_mut(&mut self) -> &mut Vec<Item> {
        &mut self.list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use list_mut()")]
    pub fn get_list_mut(&mut self) -> &mut Vec<Item> {
        self.list_mut()
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: Item) -> &mut Self {
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
                if e.name().into_inner() == b"item" {
                    let mut obj = Item::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"items" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "items")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // items
        if !self.list.is_empty() {
            write_start_tag(
                writer,
                "items",
                vec![("count", self.list.len().to_string()).into()],
                false,
            );

            // i
            for i in &self.list {
                i.write_to(writer);
            }

            write_end_tag(writer, "items");
        }
    }
}
