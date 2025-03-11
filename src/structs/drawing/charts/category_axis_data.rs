// c:cat
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    StringLiteral,
    StringReference,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::Workbook,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct CategoryAxisData {
    string_reference: Option<StringReference>,
    string_literal:   Option<StringLiteral>,
}

impl CategoryAxisData {
    #[must_use]
    pub fn string_reference(&self) -> Option<&StringReference> {
        self.string_reference.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use string_reference()")]
    pub fn get_string_reference(&self) -> Option<&StringReference> {
        self.string_reference()
    }

    pub fn string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use get_string_reference_mut()")]
    pub fn get_string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference_mut()
    }

    pub fn set_string_reference(&mut self, value: StringReference) -> &mut Self {
        self.string_reference = Some(value);
        self
    }

    pub fn remove_string_reference(&mut self) -> &mut Self {
        self.string_reference = None;
        self
    }

    #[must_use]
    pub fn string_literal(&self) -> Option<&StringLiteral> {
        self.string_literal.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use string_literal()")]
    pub fn get_string_literal(&self) -> Option<&StringLiteral> {
        self.string_literal()
    }

    pub fn string_literal_mut(&mut self) -> Option<&mut StringLiteral> {
        self.string_literal.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use string_literal_mut()")]
    pub fn get_string_literal_mut(&mut self) -> Option<&mut StringLiteral> {
        self.string_literal_mut()
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
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:cat" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:cat")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:cat
        write_start_tag(writer, "c:cat", vec![], false);

        // c:strRef
        if let Some(v) = &self.string_reference {
            v.write_to(writer, wb);
        }

        // c:strLit
        if let Some(v) = &self.string_literal {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:cat");
    }
}
