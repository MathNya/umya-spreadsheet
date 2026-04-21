// pivotField
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{BytesStart, Event},
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        BooleanValue,
        Items,
    },
    writer::driver::{write_end_tag, write_start_tag}, xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct PivotField {
    data_field: BooleanValue,
    show_all:   BooleanValue,
    items: Items,
}
impl PivotField {
    #[inline]
    #[must_use]
    pub fn data_field(&self) -> bool {
        self.data_field.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_field()")]
    pub fn get_data_field(&self) -> bool {
        self.data_field()
    }

    #[inline]
    pub fn set_data_field(&mut self, value: bool) -> &mut Self {
        self.data_field.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn show_all(&self) -> bool {
        self.show_all.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use show_all()")]
    pub fn get_show_all(&self) -> bool {
        self.show_all()
    }

    #[inline]
    pub fn set_show_all(&mut self, value: bool) -> &mut Self {
        self.show_all.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn items(&self) -> &Items {
        &self.items
    }

    #[inline]
    #[must_use]
    pub fn items_mut(&mut self) -> &mut Items {
        &mut self.items
    }

    #[inline]
    pub fn set_items(&mut self, value: Items) -> &mut Self {
        self.items = value;
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, data_field, "dataField");
        set_string_from_xml!(self, e, show_all, "showAll");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"items" {
                    self.items.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"pivotField" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "pivotField")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotField
        let empty_flg = self.items.list().is_empty();
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.data_field.has_value() {
            attributes.push(("dataField", self.data_field.value_string()).into());
        }
        if self.show_all.has_value() {
            attributes.push(("showAll", self.show_all.value_string()).into());
        }
        write_start_tag(
            writer,
            "pivotField",
            attributes,
            empty_flg,
        );
        if !empty_flg {
            self.items.write_to(writer);
            write_end_tag(writer, "pivotField");
        }
    }
}
