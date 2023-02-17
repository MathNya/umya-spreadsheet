// colBreaks
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Break;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ColumnBreaks {
    break_list: Vec<Break>,
}
impl ColumnBreaks {
    pub fn get_break_list(&self) -> &Vec<Break> {
        &self.break_list
    }

    pub fn get_break_list_mut(&mut self) -> &mut Vec<Break> {
        &mut self.break_list
    }

    pub fn add_break_list(&mut self, value: Break) -> &mut Self {
        self.break_list.push(value);
        self
    }

    pub(crate) fn has_param(&self) -> bool {
        if !self.break_list.is_empty() {
            return true;
        }
        false
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
                    b"brk" => {
                        let mut obj = Break::default();
                        obj.set_attributes(reader, e);
                        self.add_break_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"colBreaks" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "colBreaks"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // colBreaks
            let mut count = 0;
            let mut manual_count = 0;
            for obj in self.get_break_list() {
                count += 1;
                if obj.get_manual_page_break() == &true {
                    manual_count += 1;
                }
            }
            write_start_tag(
                writer,
                "colBreaks",
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

            write_end_tag(writer, "colBreaks");
        }
    }
}
