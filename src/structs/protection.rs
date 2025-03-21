// protection
use std::io::Cursor;

use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::BooleanValue;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Default, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Protection {
    locked: BooleanValue,
    hidden: BooleanValue,
}

impl Protection {
    #[inline]
    #[must_use]
    pub fn locked(&self) -> bool {
        self.locked.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use locked()")]
    pub fn get_locked(&self) -> bool {
        self.locked()
    }

    #[inline]
    pub fn set_locked(&mut self, value: bool) {
        self.locked.set_value(value);
    }

    #[inline]
    pub fn hidden(&mut self) -> bool {
        self.hidden.value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hidden()")]
    pub fn get_hidden(&mut self) -> bool {
        self.hidden()
    }

    #[inline]
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden.set_value(value);
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                &self.locked.hash_string(),
                &self.hidden.hash_string()
            ))
        )
    }

    #[inline]
    #[allow(dead_code)]
    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, locked, "locked");
        set_string_from_xml!(self, e, hidden, "hidden");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // protection
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.locked.has_value() {
            attributes.push(("locked", self.locked.value_string()).into());
        }
        if self.hidden.has_value() {
            attributes.push(("hidden", self.hidden.value_string()).into());
        }
        write_start_tag(writer, "protection", attributes, true);
    }
}
