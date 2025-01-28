// pivotTableStyleInfo
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
        BooleanValue,
        StringValue,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PivotTableStyle {
    name:                StringValue,
    show_row_headers:    BooleanValue,
    show_column_headers: BooleanValue,
    show_row_stripes:    BooleanValue,
    show_column_stripes: BooleanValue,
    show_last_column:    BooleanValue,
}
impl PivotTableStyle {
    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_row_headers(&self) -> bool {
        self.show_row_headers.value()
    }

    #[inline]
    pub fn set_show_row_headers(&mut self, value: bool) -> &mut Self {
        self.show_row_headers.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_column_headers(&self) -> bool {
        self.show_column_headers.value()
    }

    #[inline]
    pub fn set_show_column_headers(&mut self, value: bool) -> &mut Self {
        self.show_column_headers.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_row_stripes(&self) -> bool {
        self.show_row_stripes.value()
    }

    #[inline]
    pub fn set_show_row_stripes(&mut self, value: bool) -> &mut Self {
        self.show_row_stripes.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_column_stripes(&self) -> bool {
        self.show_column_stripes.value()
    }

    #[inline]
    pub fn set_show_column_stripes(&mut self, value: bool) -> &mut Self {
        self.show_column_stripes.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_last_column(&self) -> bool {
        self.show_last_column.value()
    }

    #[inline]
    pub fn set_show_last_column(&mut self, value: bool) -> &mut Self {
        self.show_last_column.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, show_row_headers, "showRowHeaders");
        set_string_from_xml!(self, e, show_column_headers, "showColHeaders");
        set_string_from_xml!(self, e, show_row_stripes, "showRowStripes");
        set_string_from_xml!(self, e, show_column_stripes, "showColStripes");
        set_string_from_xml!(self, e, show_last_column, "showLastColumn");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotTableStyleInfo
        write_start_tag(
            writer,
            "pivotTableStyleInfo",
            vec![
                ("dataField", self.name.value_str()).into(),
                ("showAll", self.show_row_headers.value_string()).into(),
                ("showAll", self.show_column_headers.value_string()).into(),
                ("showAll", self.show_row_stripes.value_string()).into(),
                ("showAll", self.show_column_stripes.value_string()).into(),
                ("showAll", self.show_last_column.value_string()).into(),
            ],
            true,
        );
    }
}
