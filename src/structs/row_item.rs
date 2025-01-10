// i
use crate::reader::driver::*;
use crate::structs::EnumValue;
use crate::structs::ItemValues;
use crate::structs::MemberPropertyIndex;
use crate::structs::UInt32Value;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct RowItem {
    index: UInt32Value,
    item_type: EnumValue<ItemValues>,
    repeated_item_count: UInt32Value,
    member_property_index: Option<MemberPropertyIndex>,
}
impl RowItem {
    #[inline]
    pub fn get_index(&self) -> u32 {
        self.index.get_value()
    }

    #[inline]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index.set_value(value);
        self
    }

    #[inline]
    pub fn get_item_type(&self) -> &ItemValues {
        self.item_type.get_value()
    }

    #[inline]
    pub fn set_item_type(&mut self, value: ItemValues) -> &mut Self {
        self.item_type.set_value(value);
        self
    }

    #[inline]
    pub fn get_repeated_item_count(&self) -> u32 {
        self.repeated_item_count.get_value()
    }

    #[inline]
    pub fn set_repeated_item_count(&mut self, value: u32) -> &mut Self {
        self.repeated_item_count.set_value(value);
        self
    }

    #[inline]
    pub fn get_member_property_index(&self) -> Option<&MemberPropertyIndex> {
        self.member_property_index.as_ref()
    }

    #[inline]
    pub fn get_member_property_index_mut(&mut self) -> Option<&mut MemberPropertyIndex> {
        self.member_property_index.as_mut()
    }

    #[inline]
    pub fn set_member_property_index_color(&mut self, value: MemberPropertyIndex) -> &mut Self {
        self.member_property_index = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, index, "i");
        set_string_from_xml!(self, e, item_type, "t");
        set_string_from_xml!(self, e, repeated_item_count, "r");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"x" {
                    let mut obj = MemberPropertyIndex::default();
                    obj.set_attributes(reader, e);
                    self.set_member_property_index_color(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"i" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "i")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flg = self.member_property_index.is_some();
        // i
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let index_str = self.index.get_value_string();
        if self.index.has_value() {
            attributes.push(("i", index_str.as_str()));
        }
        if self.item_type.has_value() {
            attributes.push(("t", self.item_type.get_value_string()));
        }
        let repeated_item_count_str = self.repeated_item_count.get_value_string();
        if self.repeated_item_count.has_value() {
            attributes.push(("r", repeated_item_count_str.as_str()));
        }
        write_start_tag(writer, "i", attributes, empty_flg);
        if !empty_flg {
            if let Some(v) = &self.member_property_index {
                v.write_to(writer);
            }
            write_end_tag(writer, "i");
        }
    }
}
