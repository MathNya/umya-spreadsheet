// i
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
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::{
        EnumValue,
        ItemValues,
        MemberPropertyIndex,
        UInt32Value,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct RowItem {
    index:                  UInt32Value,
    item_type:              EnumValue<ItemValues>,
    repeated_item_count:    UInt32Value,
    member_property_indices: Vec<MemberPropertyIndex>,
}
impl RowItem {
    #[inline]
    #[must_use]
    pub fn index(&self) -> u32 {
        self.index.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use index()")]
    pub fn get_index(&self) -> u32 {
        self.index()
    }

    #[inline]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn item_type(&self) -> &ItemValues {
        self.item_type.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use item_type()")]
    pub fn get_item_type(&self) -> &ItemValues {
        self.item_type()
    }

    #[inline]
    pub fn set_item_type(&mut self, value: ItemValues) -> &mut Self {
        self.item_type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn repeated_item_count(&self) -> u32 {
        self.repeated_item_count.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use repeated_item_count()")]
    pub fn get_repeated_item_count(&self) -> u32 {
        self.repeated_item_count()
    }

    #[inline]
    pub fn set_repeated_item_count(&mut self, value: u32) -> &mut Self {
        self.repeated_item_count.set_value(value);
        self
    }

    /// Returns the first `<x/>` child (backward-compatible convenience method).
    /// Use [`member_property_indices()`] to access all row-field children.
    #[inline]
    #[must_use]
    pub fn member_property_index(&self) -> Option<&MemberPropertyIndex> {
        self.member_property_indices.first()
    }

    /// Returns a mutable reference to the first `<x/>` child.
    #[inline]
    pub fn member_property_index_mut(&mut self) -> Option<&mut MemberPropertyIndex> {
        self.member_property_indices.first_mut()
    }

    /// Sets a single `<x/>` child (replaces any existing entries).
    /// For multiple row fields, use [`add_member_property_index()`].
    #[inline]
    pub fn set_member_property_index_color(&mut self, value: MemberPropertyIndex) -> &mut Self {
        self.member_property_indices = vec![value];
        self
    }

    /// Returns all `<x/>` children — one per row field.
    #[inline]
    #[must_use]
    pub fn member_property_indices(&self) -> &[MemberPropertyIndex] {
        &self.member_property_indices
    }

    /// Returns a mutable reference to all `<x/>` children.
    #[inline]
    pub fn member_property_indices_mut(&mut self) -> &mut Vec<MemberPropertyIndex> {
        &mut self.member_property_indices
    }

    /// Appends an `<x/>` child. Call once per row field.
    #[inline]
    pub fn add_member_property_index(&mut self, value: MemberPropertyIndex) -> &mut Self {
        self.member_property_indices.push(value);
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
            // Support multiple <x/> children — one per row field.
            // A pivot table with N row fields produces N <x/> elements per <i> row item.
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"x" {
                    let mut obj = MemberPropertyIndex::default();
                    obj.set_attributes(reader, e);
                    self.member_property_indices.push(obj);
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
        let empty_flg = self.member_property_indices.is_empty();
        // i
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let index_str = self.index.value_string();
        if self.index.has_value() {
            attributes.push(("i", &index_str).into());
        }
        let item_type_str = self.item_type.value_string();
        if self.item_type.has_value() {
            attributes.push(("t", item_type_str).into());
        }
        let repeated_item_count_str = self.repeated_item_count.value_string();
        if self.repeated_item_count.has_value() {
            attributes.push(("r", &repeated_item_count_str).into());
        }
        write_start_tag(writer, "i", attributes, empty_flg);
        if !empty_flg {
            // Write all <x/> children — one per row field.
            for v in &self.member_property_indices {
                v.write_to(writer);
            }
            write_end_tag(writer, "i");
        }
    }
}
