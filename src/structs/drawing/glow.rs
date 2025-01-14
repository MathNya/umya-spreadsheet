// a:glow
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use super::{super::super::Int64Value, SchemeColor};
use crate::{
    reader::driver::{get_attribute, xml_read_loop},
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct Glow {
    radius: Int64Value,
    scheme_color: Option<Box<SchemeColor>>,
}

impl Glow {
    #[inline]
    #[must_use]
    pub fn get_radius(&self) -> i64 {
        self.radius.get_value()
    }

    #[inline]
    pub fn set_radius(&mut self, value: i64) -> &mut Glow {
        self.radius.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.radius
            .set_value_string(get_attribute(e, b"rad").unwrap());

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    let mut obj = SchemeColor::default();
                    obj.set_attributes(reader, e, false);
                    self.set_scheme_color(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:glow" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:glow")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:glow
        write_start_tag(
            writer,
            "a:glow",
            vec![("rad", &self.radius.get_value_string()).into()],
            false,
        );

        // a:schemeClr
        if let Some(v) = &self.scheme_color {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:glow");
    }
}
