use super::ClipboardFormatValues;
use crate::reader::driver::*;
use crate::structs::EnumValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ClipboardFormat {
    value: EnumValue<ClipboardFormatValues>,
}

impl ClipboardFormat {
    #[inline]
    pub fn get_value(&self) -> &ClipboardFormatValues {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: ClipboardFormatValues) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.value.set_value_string(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:CF" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:CF")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:CF
        write_start_tag(writer, "x:CF", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:CF");
    }
}
