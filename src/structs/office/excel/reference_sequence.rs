// xm:sqref
use std::{
    io::Cursor,
    vec,
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    structs::Range,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Default, Debug, Clone)]
pub struct ReferenceSequence {
    value: Vec<Range>,
}
impl ReferenceSequence {
    #[inline]
    #[must_use]
    pub fn value(&self) -> &[Range] {
        &self.value
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value(&self) -> &[Range] {
        self.value()
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut Vec<Range> {
        &mut self.value
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_mut()")]
    pub fn get_value_mut(&mut self) -> &mut Vec<Range> {
        self.value_mut()
    }

    #[inline]
    pub fn set_value(&mut self, value: impl Into<Vec<Range>>) -> &mut Self {
        self.value = value.into();
        self
    }

    #[inline]
    pub fn add_value(&mut self, value: Range) -> &mut Self {
        self.value.push(value);
        self
    }

    #[inline]
    pub fn remove_value(&mut self) -> &mut Self {
        self.value.clear();
        self
    }

    pub fn set_sqref<S: Into<String>>(&mut self, value: S) -> &mut Self {
        value.into().split(' ').for_each(|range_value| {
            let mut range = Range::default();
            range.set_range(range_value);
            self.value.push(range);
        });
        self
    }

    #[inline]
    #[must_use]
    pub fn sqref(&self) -> String {
        self.value
            .iter()
            .map(Range::range)
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sqref()")]
    pub fn get_sqref(&self) -> String {
        self.sqref()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    value = e.unescape().unwrap().to_string();
                }
                Ok(Event::End(ref e)) => {
                    if e.name().into_inner() == b"xm:sqref" {
                        self.set_sqref(value);
                        return;
                    }
                }
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "xm:sqref"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:sqref", vec![], false);
        write_text_node(writer, self.sqref());
        write_end_tag(writer, "xm:sqref");
    }
}
