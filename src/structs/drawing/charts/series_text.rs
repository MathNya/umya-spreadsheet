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

use crate::{
    reader::driver::xml_read_loop,
    structs::StringValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SeriesText {
    value: StringValue,
}

impl SeriesText {
    #[must_use]
    pub fn get_value(&self) -> &str {
        self.value.value_str()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_value(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:tx" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:tx")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, self.value.value_str());
        write_end_tag(writer, "c:v");

        write_end_tag(writer, "c:tx");
    }
}
