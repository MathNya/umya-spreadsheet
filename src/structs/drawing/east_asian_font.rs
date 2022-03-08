// a:ea
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct EastAsianFont {
    typeface: String,
    pitch_family: String,
    charset: String,
}
impl EastAsianFont {
    pub fn get_typeface(&self) -> &str {
        &self.typeface
    }

    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut EastAsianFont {
        self.typeface = value.into();
        self
    }

    pub fn get_pitch_family(&self) -> &str {
        &self.pitch_family
    }

    pub fn set_pitch_family<S: Into<String>>(&mut self, value: S) -> &mut EastAsianFont {
        self.pitch_family = value.into();
        self
    }

    pub fn get_charset(&self) -> &str {
        &self.charset
    }

    pub fn set_charset<S: Into<String>>(&mut self, value: S) -> &mut EastAsianFont {
        self.charset = value.into();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"typeface") {
            Some(v) => {
                self.set_typeface(v);
            }
            None => {}
        }
        match get_attribute(e, b"pitchFamily") {
            Some(v) => {
                self.set_pitch_family(v);
            }
            None => {}
        }
        match get_attribute(e, b"charset") {
            Some(v) => {
                self.set_charset(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ea
        write_start_tag(
            writer,
            "a:ea",
            vec![
                ("typeface", &self.typeface),
                ("pitchFamily", &self.pitch_family),
                ("charset", &self.charset),
            ],
            true,
        );
    }
}
