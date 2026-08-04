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
    structs::StringValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node_no_escape,
    },
};

#[derive(Clone, Debug)]
pub struct TextBox {
    style:  StringValue,
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
    #[must_use]
    pub fn style(&self) -> &str {
        self.style.value_str()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_style(&self) -> &str {
        self.style()
    }

    pub fn set_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.style.set_value(value);
        self
    }

    #[must_use]
    pub fn innder(&self) -> &str {
        self.innder.value_str()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use innder()")]
    pub fn get_innder(&self) -> &str {
        self.innder()
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
        let mut buf = Vec::new();
        reader.config_mut().check_end_names = false;
        let text = crate::helper::utils::unescape_xml_text(
            &reader.read_text_into(e.name(), &mut buf).unwrap(),
        );
        self.set_innder(text);
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:textbox
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.style.has_value() {
            attributes.push(("style", self.style.value_str()).into());
        }
        write_start_tag(writer, "v:textbox", attributes, false);
        write_text_node_no_escape(writer, self.innder.value_str());
        write_end_tag(writer, "v:textbox");
    }
}
