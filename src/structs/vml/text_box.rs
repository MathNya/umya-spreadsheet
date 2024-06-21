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
        self.style.get_value_str()
    }

    pub fn set_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.style.set_value(value);
        self
    }

    pub fn get_innder(&self) -> &str {
        self.innder.get_value_str()
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
        let mut inner_text = String::from("");
        reader.config_mut().check_end_names = false;
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    let mut tag = std::str::from_utf8(e.name().into_inner())
                        .unwrap()
                        .to_string();
                    let mut attrs = vec![];
                    e.attributes().for_each(|a| {
                        if let Ok(attribute) = a {
                            if let (Ok(key), Ok(value)) = (
                                std::str::from_utf8(attribute.key.into_inner()),
                                std::str::from_utf8(attribute.value.as_ref()),
                            ) {
                                attrs.push((key.to_owned(), value.to_owned()));
                            }
                        }
                    });
                    for (key, value) in &attrs {
                        tag = format!("{} {}={}", tag, key, value);
                    }
                    inner_text = format!("{}<{}/>", inner_text, tag);
                }
                Ok(Event::Start(ref e)) => {
                    let mut tag = std::str::from_utf8(e.name().into_inner())
                        .unwrap()
                        .to_string();
                    let mut attrs = vec![];
                    e.attributes().for_each(|a| {
                        if let Ok(attribute) = a {
                            if let (Ok(key), Ok(value)) = (
                                std::str::from_utf8(attribute.key.into_inner()),
                                std::str::from_utf8(attribute.value.as_ref()),
                            ) {
                                attrs.push((key.to_owned(), value.to_owned()));
                            }
                        }
                    });
                    for (key, value) in &attrs {
                        tag = format!("{} {}='{}'", tag, key, value);
                    }
                    inner_text = format!("{}<{}>", inner_text, tag);
                }
                Ok(Event::Text(ref e)) => {
                    let s = e.unescape().unwrap().to_string();
                    inner_text = format!("{}{}", inner_text, s);
                }
                Ok(Event::End(ref e)) => {
                    if e.name().into_inner() == b"v:textbox" {
                        break;
                    }
                    let s = std::str::from_utf8(e.name().into_inner()).unwrap();
                    inner_text = format!("{}</{}>", inner_text, s);
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
        //reader.check_end_names(true);
        self.set_innder(inner_text);
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
