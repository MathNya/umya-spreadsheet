use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use crate::reader::driver::xml_read_loop;
use crate::structs::TrueFalseBlankValue;
use crate::writer::driver::{write_end_tag, write_start_tag, write_text_node};

#[derive(Clone, Default, Debug)]
pub struct MoveWithCells {
    value: TrueFalseBlankValue,
}

impl MoveWithCells {
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> Option<bool> {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.value.set_value_string(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:MoveWithCells" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:MoveWithCells")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:MoveWithCells
        if self.value.has_value() {
            write_start_tag(writer, "x:MoveWithCells", vec![], false);
            write_text_node(writer, self.value.get_value_string2());
            write_end_tag(writer, "x:MoveWithCells");
        } else {
            write_start_tag(writer, "x:MoveWithCells", vec![], true);
        }
    }
}
