use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Debug)]
pub struct TextBox {
    style: StringValue,
    innder: StringValue,
}

impl Default for TextBox {
    fn default() -> Self {
        let mut style = StringValue::default();
        style.set_value_string("mso-direction-alt:auto");
        let mut innder = StringValue::default();
        innder.set_value_string("<div style=\"text-align:left\"/>");
        Self { style, innder }
    }
}

impl TextBox {
    pub fn get_style(&self) -> &str {
        self.style.get_value()
    }

    pub fn set_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.style.set_value(value);
        self
    }

    pub fn get_innder(&self) -> &str {
        self.innder.get_value()
    }

    pub fn set_innder<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.innder.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, style, "style");

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"v:textbox" {
                    return
                }
            },
            Event::Eof => panic!("Error not find {} end element", "v:textbox")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:textbox
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.style.has_value() {
            attributes.push(("style", self.style.get_value_str()));
        }
        write_start_tag(writer, "v:textbox", attributes, false);
        write_text_node_no_escape(writer, self.innder.get_value_str());
        write_end_tag(writer, "v:textbox");
    }
}
