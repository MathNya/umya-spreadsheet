use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    structs::TrueFalseBlankValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ResizeWithCells {
    value: TrueFalseBlankValue,
}

impl ResizeWithCells {
    #[inline]
    #[must_use]
    pub fn value(&self) -> Option<bool> {
        self.value.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value(&self) -> Option<bool> {
        self.value()
    }

    #[inline]
    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.value
            .set_value_string(crate::helper::utils::unescape_xml_text(&text));
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:SizeWithCells
        if self.value.has_value() {
            write_start_tag(writer, "x:SizeWithCells", vec![], false);
            write_text_node(writer, self.value.value_string2());
            write_end_tag(writer, "x:SizeWithCells");
        } else {
            write_start_tag(writer, "x:SizeWithCells", vec![], true);
        }
    }
}
