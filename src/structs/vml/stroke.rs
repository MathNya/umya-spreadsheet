use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Stroke {
    dash_style: StringValue,
}
impl Stroke {
    pub fn get_dash_style(&self) -> &str {
        self.dash_style.get_value()
    }

    pub fn set_dash_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.dash_style.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"dashstyle") {
            Some(v) => {
                self.dash_style.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:stroke
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.dash_style.has_value() {
            attributes.push(("dashstyle", self.dash_style.get_value_string()));
        }
        write_start_tag(writer, "v:stroke", attributes, true);
    }
}
