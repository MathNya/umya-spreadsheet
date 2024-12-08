// pivotTableStyleInfo
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::structs::BooleanValue;
use crate::structs::ByteValue;
use crate::structs::Location;
use crate::structs::StringValue;
use crate::structs::UInt32Value;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PivotTableStyle {
    name: StringValue,
    show_row_headers: BooleanValue,
    show_column_headers: BooleanValue,
    show_row_stripes: BooleanValue,
    show_column_stripes: BooleanValue,
    show_last_column: BooleanValue,
}
impl PivotTableStyle {
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name.get_value_str()
    }

    #[inline]
    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    pub fn get_show_row_headers(&self) -> &bool {
        self.show_row_headers.get_value()
    }

    #[inline]
    pub fn set_show_row_headers(&mut self, value: bool) -> &mut Self {
        self.show_row_headers.set_value(value);
        self
    }

    #[inline]
    pub fn get_show_column_headers(&self) -> &bool {
        self.show_column_headers.get_value()
    }

    #[inline]
    pub fn set_show_column_headers(&mut self, value: bool) -> &mut Self {
        self.show_column_headers.set_value(value);
        self
    }

    #[inline]
    pub fn get_show_row_stripes(&self) -> &bool {
        self.show_row_stripes.get_value()
    }

    #[inline]
    pub fn set_show_row_stripes(&mut self, value: bool) -> &mut Self {
        self.show_row_stripes.set_value(value);
        self
    }

    #[inline]
    pub fn get_show_column_stripes(&self) -> &bool {
        self.show_column_stripes.get_value()
    }

    #[inline]
    pub fn set_show_column_stripes(&mut self, value: bool) -> &mut Self {
        self.show_column_stripes.set_value(value);
        self
    }

    #[inline]
    pub fn get_show_last_column(&self) -> &bool {
        self.show_last_column.get_value()
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
                ("dataField", self.name.get_value_str()),
                ("showAll", self.show_row_headers.get_value_string()),
                ("showAll", self.show_column_headers.get_value_string()),
                ("showAll", self.show_row_stripes.get_value_string()),
                ("showAll", self.show_column_stripes.get_value_string()),
                ("showAll", self.show_last_column.get_value_string()),
            ],
            true,
        );
    }
}
