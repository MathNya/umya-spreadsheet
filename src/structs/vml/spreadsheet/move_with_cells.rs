use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::TrueFalseBlankValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct MoveWithCells {
    value: TrueFalseBlankValue,
}

impl MoveWithCells {
    pub fn get_value(&self) -> Option<&bool> {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

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
