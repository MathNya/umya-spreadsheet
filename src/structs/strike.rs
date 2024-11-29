// strike
use super::BooleanValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Strike {
    pub(crate) val: BooleanValue,
}

impl Strike {
    #[inline]
    pub fn get_val(&self) -> &bool {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: bool) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value(true);
        set_string_from_xml!(self, e, val, "val");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // strike
        if !self.val.has_value() {
            return;
        }

        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if !*self.val.get_value() {
            attributes.push(("val", self.val.get_value_string()));
        }
        write_start_tag(writer, "strike", attributes, true);
    }
}
