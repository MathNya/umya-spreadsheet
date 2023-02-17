// c:cat
use super::StringLiteral;
use super::StringReference;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CategoryAxisData {
    string_reference: Option<StringReference>,
    string_literal: Option<StringLiteral>,
}
impl CategoryAxisData {
    pub fn get_string_reference(&self) -> &Option<StringReference> {
        &self.string_reference
    }

    pub fn get_string_reference_mut(&mut self) -> &mut Option<StringReference> {
        &mut self.string_reference
    }

    pub fn set_string_reference(&mut self, value: StringReference) -> &mut Self {
        self.string_reference = Some(value);
        self
    }

    pub fn remove_string_reference(&mut self) -> &mut Self {
        self.string_reference = None;
        self
    }

    pub fn get_string_literal(&self) -> &Option<StringLiteral> {
        &self.string_literal
    }

    pub fn get_string_literal_mut(&mut self) -> &mut Option<StringLiteral> {
        &mut self.string_literal
    }

    pub fn set_string_literal(&mut self, value: StringLiteral) -> &mut Self {
        self.string_literal = Some(value);
        self
    }

    pub fn remove_string_literal(&mut self) -> &mut Self {
        self.string_literal = None;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"c:strRef" => {
                        let mut obj = StringReference::default();
                        obj.set_attributes(reader, e);
                        self.set_string_reference(obj);
                    }
                    b"c:strLit" => {
                        let mut obj = StringLiteral::default();
                        obj.set_attributes(reader, e);
                        self.set_string_literal(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:cat" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:cat"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:cat
        write_start_tag(writer, "c:cat", vec![], false);

        // c:strRef
        match &self.string_reference {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:strLit
        match &self.string_literal {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:cat");
    }
}
