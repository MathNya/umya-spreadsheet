// cellStyle
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        StringValue,
        UInt32Value,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct CellStyle {
    name:       StringValue,
    builtin_id: UInt32Value,
    format_id:  UInt32Value,
}

impl CellStyle {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn builtin_id(&self) -> u32 {
        self.builtin_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use builtin_id()")]
    pub fn get_builtin_id(&self) -> u32 {
        self.builtin_id()
    }

    #[inline]
    pub fn set_builtin_id(&mut self, value: u32) -> &mut Self {
        self.builtin_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn format_id(&self) -> u32 {
        self.format_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use format_id()")]
    pub fn get_format_id(&self) -> u32 {
        self.format_id()
    }

    #[inline]
    pub fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, builtin_id, "builtinId");
        set_string_from_xml!(self, e, format_id, "xfId");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cellStyle
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("name", self.name.value_str()).into());
        let format_id = self.format_id.value_string();
        attributes.push(("xfId", &format_id).into());
        let builtin_id = self.builtin_id.value_string();
        attributes.push(("builtinId", &builtin_id).into());
        write_start_tag(writer, "cellStyle", attributes, true);
    }
}
