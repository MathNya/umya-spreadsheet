// cellStyle
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CellStyle {
    name: StringValue,
    builtin_id: UInt32Value,
    format_id: UInt32Value,
}
impl CellStyle {
    pub fn get_name(&self) -> &str {
        self.name.get_value()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_builtin_id(&self) -> &u32 {
        self.builtin_id.get_value()
    }

    pub fn set_builtin_id(&mut self, value: u32) -> &mut Self {
        self.builtin_id.set_value(value);
        self
    }

    pub fn get_format_id(&self) -> &u32 {
        self.format_id.get_value()
    }

    pub fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"name") {
            Some(v) => {
                self.name.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"xfId") {
            Some(v) => {
                self.builtin_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"builtinId") {
            Some(v) => {
                self.format_id.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cellStyle
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("name", self.name.get_value_string()));
        let builtin_id = self.builtin_id.get_value_string();
        attributes.push(("xfId", &builtin_id));
        let format_id = self.format_id.get_value_string();
        attributes.push(("builtinId", &format_id));
        write_start_tag(writer, "cellStyle", attributes, true);
    }
}
