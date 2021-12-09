// mc:Choice
use super::office2010::drawing::charts::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug)]
pub struct AlternateContentChoice {
    style: Style,
}
impl AlternateContentChoice {
    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut AlternateContentChoice {
        self.style = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"c14:style" => {
                        self.style.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"mc:Choice" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:Choice"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:Choice
        write_start_tag(
            writer,
            "mc:Choice",
            vec![
                ("Requires", "c14"),
                (
                    "xmlns:c14",
                    "http://schemas.microsoft.com/office/drawing/2007/8/2/chart",
                ),
            ],
            false,
        );

        // c14:style
        &self.style.write_to(writer);

        write_end_tag(writer, "mc:Choice");
    }
}
