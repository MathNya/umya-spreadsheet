// c:tx
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::RichText;
use crate::{
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct ChartText {
    rich_text: RichText,
}

impl ChartText {
    #[must_use]
    pub fn rich_text(&self) -> &RichText {
        &self.rich_text
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rich_text()")]
    pub fn get_rich_text(&self) -> &RichText {
        self.rich_text()
    }

    pub fn rich_text_mut(&mut self) -> &mut RichText {
        &mut self.rich_text
    }

    #[deprecated(since = "3.0.0", note = "Use rich_text_mut()")]
    pub fn get_rich_text_mut(&mut self) -> &mut RichText {
        self.rich_text_mut()
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut ChartText {
        self.rich_text = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"c:rich" {
                    self.rich_text.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:tx" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:tx"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:rich
        self.rich_text.write_to(writer);

        write_end_tag(writer, "c:tx");
    }
}
