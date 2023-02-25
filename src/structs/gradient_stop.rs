// stop
use super::Color;
use super::DoubleValue;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GradientStop {
    position: DoubleValue,
    color: Color,
}
impl GradientStop {
    pub fn get_position(&self) -> &f64 {
        self.position.get_value()
    }

    pub fn set_position(&mut self, value: f64) -> &mut Self {
        self.position.set_value(value);
        self
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    pub fn set_color(&mut self, value: Color) -> &mut Self {
        self.color = value;
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                &self.position.get_value_string(),
                &self.color.get_hash_code(),
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"position") {
            Some(v) => {
                self.position.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"color" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, true);
                        self.set_color(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"stop" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "stop"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // stop
        write_start_tag(
            writer,
            "stop",
            vec![("position", &self.position.get_value_string())],
            false,
        );

        // color
        let _ = &self.color.write_to_color(writer);

        write_end_tag(writer, "stop");
    }
}
