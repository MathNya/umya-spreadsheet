// fronts
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Font;
use structs::Style;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct Fonts {
    font: Vec<Font>,
}
impl Fonts {
    pub(crate) fn get_font(&self) -> &Vec<Font> {
        &self.font
    }

    pub(crate) fn get_font_mut(&mut self) -> &mut Vec<Font> {
        &mut self.font
    }

    pub(crate) fn set_font(&mut self, value: Font) -> &mut Self {
        self.font.push(value);
        self
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        match style.get_font() {
            Some(v) => {
                let hash_code = v.get_hash_code();
                let mut id = 0;
                for font in &self.font {
                    if font.get_hash_code() == hash_code {
                        return id;
                    }
                    id += 1;
                }
                self.set_font(v.clone());
                id
            }
            None => 0,
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"font" => {
                        let obj = Font::default();
                        self.set_font(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"font" => {
                        let mut obj = Font::default();
                        obj.set_attributes(reader, e);
                        self.set_font(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"fonts" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "fonts"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.font.is_empty() {
            // fonts
            write_start_tag(
                writer,
                "fonts",
                vec![
                    ("count", &self.font.len().to_string()),
                    ("x14ac:knownFonts", "1"),
                ],
                false,
            );

            // font
            for font in &self.font {
                font.write_to_font(writer);
            }

            write_end_tag(writer, "fonts");
        }
    }
}
