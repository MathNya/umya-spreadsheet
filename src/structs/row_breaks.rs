// rowBreaks
use crate::reader::driver::xml_read_loop;
use crate::structs::Break;
use crate::writer::driver::{write_end_tag, write_start_tag};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct RowBreaks {
    break_list: Vec<Break>,
}

impl RowBreaks {
    #[inline]
    #[must_use]
    pub fn get_break_list(&self) -> &[Break] {
        &self.break_list
    }

    #[inline]
    pub fn get_break_list_mut(&mut self) -> &mut Vec<Break> {
        &mut self.break_list
    }

    #[inline]
    pub fn add_break_list(&mut self, value: Break) -> &mut Self {
        self.break_list.push(value);
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        !self.break_list.is_empty()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"brk" {
                    let mut obj = Break::default();
                    obj.set_attributes(reader, e);
                    self.add_break_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"rowBreaks" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "rowBreaks")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.has_param() {
            return;
        }

        // rowBreaks
        let mut count = 0;
        let mut manual_count = 0;
        for obj in self.get_break_list() {
            count += 1;
            if obj.get_manual_page_break() {
                manual_count += 1;
            }
        }
        write_start_tag(
            writer,
            "rowBreaks",
            vec![
                ("count", count.to_string().as_str()),
                ("manualBreakCount", manual_count.to_string().as_str()),
            ],
            false,
        );

        // brk
        for obj in self.get_break_list() {
            obj.write_to(writer);
        }

        write_end_tag(writer, "rowBreaks");
    }
}
