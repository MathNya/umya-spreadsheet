// a:prstClr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::alpha::Alpha;
use crate::{
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct PresetColor {
    val: Box<str>,
    alpha: Option<Alpha>,
}

impl PresetColor {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &str {
        &self.val
    }

    #[inline]
    pub fn set_val<S: Into<String>>(&mut self, value: S) {
        self.val = value.into().into_boxed_str();
    }

    #[inline]
    #[must_use]
    pub fn get_alpha(&self) -> Option<&Alpha> {
        self.alpha.as_ref()
    }

    #[inline]
    pub fn get_alpha_mut(&mut self) -> Option<&mut Alpha> {
        self.alpha.as_mut()
    }

    #[inline]
    pub fn set_alpha(&mut self, value: Alpha) {
        self.alpha = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(get_attribute(e, b"val").unwrap());

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:alpha" {
                    let mut alpha = Alpha::default();
                    alpha.set_attributes(reader, e);
                    self.set_alpha(alpha);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:prstClr" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:prstClr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:prstClr
        write_start_tag(writer, "a:prstClr", vec![("val", &self.val)], false);

        // a:alpha
        if let Some(v) = &self.alpha {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:prstClr");
    }
}
