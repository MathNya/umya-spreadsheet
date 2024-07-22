// a:themeElements
use super::ColorScheme;
use super::FontScheme;
use super::FormatScheme;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ThemeElements {
    color_scheme: ColorScheme,
    font_scheme: FontScheme,
    format_scheme: FormatScheme,
}

impl ThemeElements {
    pub fn set_color_scheme(&mut self, value: ColorScheme) {
        self.color_scheme = value;
    }

    pub fn get_color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }

    pub fn get_color_scheme_mut(&mut self) -> &mut ColorScheme {
        &mut self.color_scheme
    }

    pub fn set_font_scheme(&mut self, value: FontScheme) {
        self.font_scheme = value;
    }

    pub fn get_font_scheme(&self) -> &FontScheme {
        &self.font_scheme
    }

    pub fn get_font_scheme_mut(&mut self) -> &mut FontScheme {
        &mut self.font_scheme
    }

    pub fn set_format_scheme(&mut self, value: FormatScheme) {
        self.format_scheme = value;
    }

    pub fn get_format_scheme(&self) -> &FormatScheme {
        &self.format_scheme
    }

    pub fn get_format_scheme_mut(&mut self) -> &mut FormatScheme {
        &mut self.format_scheme
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
                    b"a:clrScheme" => {
                        self.color_scheme.set_attributes(reader, e);
                    }
                    b"a:fontScheme" => {
                        self.font_scheme.set_attributes(reader, e);
                    }
                    b"a:fmtScheme" => {
                        self.format_scheme.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:themeElements" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:themeElements")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:themeElements
        write_start_tag(writer, "a:themeElements", vec![], false);

        // a:clrScheme
        self.color_scheme.write_to(writer);

        // a:fontScheme
        self.font_scheme.write_to(writer);

        // a:fmtScheme
        self.format_scheme.write_to(writer);

        write_end_tag(writer, "a:themeElements");
    }
}
