// a:prstClr
use super::alpha::Alpha;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PresetColor {
    val: String,
    alpha: Option<Alpha>,
}
impl PresetColor {
    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) {
        self.val = value.into();
    }

    pub fn get_alpha(&self) -> &Option<Alpha> {
        &self.alpha
    }

    pub fn get_alpha_mut(&mut self) -> &mut Option<Alpha> {
        &mut self.alpha
    }

    pub fn set_alpha(&mut self, value: Alpha) {
        self.alpha = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(get_attribute(e, b"val").unwrap());

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:alpha" => {
                        let mut alpha = Alpha::default();
                        alpha.set_attributes(reader, e);
                        self.set_alpha(alpha);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:prstClr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:prstClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:prstClr
        write_start_tag(writer, "a:prstClr", vec![("val", &self.val)], false);

        // a:alpha
        match &self.alpha {
            Some(v) => v.write_to(writer),
            None => {}
        }

        write_end_tag(writer, "a:prstClr");
    }
}
