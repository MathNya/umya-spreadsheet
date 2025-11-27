// vertAlign
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    EnumValue,
    VerticalAlignmentRunValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VerticalTextAlignment {
    pub(crate) val: EnumValue<VerticalAlignmentRunValues>,
}

impl VerticalTextAlignment {
    #[inline]
    #[must_use]
    pub fn val(&self) -> &VerticalAlignmentRunValues {
        self.val.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> &VerticalAlignmentRunValues {
        self.val()
    }

    #[inline]
    pub fn set_val(&mut self, value: VerticalAlignmentRunValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, val, "val");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // vertAlign
        if self.val.has_value() {
            write_start_tag(
                writer,
                "vertAlign",
                vec![("val", self.val.value_string()).into()],
                true,
            );
        }
    }
}
