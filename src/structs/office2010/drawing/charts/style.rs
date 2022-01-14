// c14:style
use structs::StringValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Style {
    val: StringValue,
    val_fallback: StringValue,
    requires: StringValue,
}
impl Style {
    pub fn get_val(&self)-> &str {
        &self.val.get_value()
    }

    pub fn set_val<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.val.set_value(value);
        self
    }

    pub fn get_val_fallback(&self)-> &str {
        &self.val_fallback.get_value()
    }

    pub fn set_val_fallback<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.val_fallback.set_value(value);
        self
    }

    pub fn get_requires(&self) -> &str {
        &self.requires.get_value()
    }

    pub fn set_requires<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.requires.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut alternate_content = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"mc:Choice" => {
                        alternate_content = String::from("Choice");
                        match get_attribute(e, b"Requires") {
                            Some(v) => {
                                self.requires.set_value_string(v);
                            },
                            None => {}
                        }
                    }
                    b"mc:Fallback" => {
                        alternate_content = String::from("Fallback");
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"c14:style" => {
                        if alternate_content.as_str() == "Choice" {
                            self.val.set_value_string(get_attribute(e, b"val").unwrap());
                        }
                        if alternate_content.as_str() == "Fallback" {
                            self.val_fallback.set_value_string(get_attribute(e, b"val").unwrap());
                        }
                    }
                    b"c:style" => {
                        if alternate_content.as_str() == "Choice" {
                            self.val.set_value_string(get_attribute(e, b"val").unwrap());
                        }
                        if alternate_content.as_str() == "Fallback" {
                            self.val_fallback.set_value_string(get_attribute(e, b"val").unwrap());
                        }
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"mc:AlternateContent" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:AlternateContent"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:AlternateContent
        write_start_tag(writer, "mc:AlternateContent", vec![
            ("xmlns:mc","http://schemas.openxmlformats.org/markup-compatibility/2006"),
        ], false);

        // mc:Choice
        write_start_tag(writer, "mc:Choice", vec![
            ("Requires", &self.requires.get_value_string()),
            ("xmlns:c14","http://schemas.microsoft.com/office/drawing/2007/8/2/chart"),
        ], false);

        // c14:style
        write_start_tag(writer, "c14:style", vec![
            ("val", &self.val.get_value_string()),
        ], true);

        write_end_tag(writer, "mc:Choice");

        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // c14:style
        write_start_tag(writer, "c:style", vec![
            ("val", &self.val_fallback.get_value_string()),
        ], true);

        write_end_tag(writer, "mc:Fallback");

        write_end_tag(writer, "mc:AlternateContent");
    }
}
