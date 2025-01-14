// a:pattFill
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use super::{BackgroundColor, ForegroundColor};
use crate::{
    reader::driver::{get_attribute, xml_read_loop},
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Debug)]
pub struct PatternFill {
    preset: Box<str>,
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
}

impl Default for PatternFill {
    #[inline]
    fn default() -> Self {
        Self {
            preset: "pct5".into(),
            foreground_color: ForegroundColor::default(),
            background_color: BackgroundColor::default(),
        }
    }
}

impl PatternFill {
    #[inline]
    #[must_use]
    pub fn get_preset(&self) -> &str {
        &self.preset
    }

    #[inline]
    pub fn set_preset(&mut self, value: String) -> &mut PatternFill {
        self.preset = value.into_boxed_str();
        self
    }

    #[inline]
    #[must_use]
    pub fn get_foreground_color(&self) -> &ForegroundColor {
        &self.foreground_color
    }

    #[inline]
    pub fn get_foreground_color_mut(&mut self) -> &mut ForegroundColor {
        &mut self.foreground_color
    }

    #[inline]
    pub fn set_foreground_color(&mut self, value: ForegroundColor) -> &mut PatternFill {
        self.foreground_color = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_background_color(&self) -> &BackgroundColor {
        &self.background_color
    }

    #[inline]
    pub fn get_background_color_mut(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }

    #[inline]
    pub fn set_background_color(&mut self, value: BackgroundColor) -> &mut PatternFill {
        self.background_color = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"prst") {
            self.set_preset(v);
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:fgClr" => {
                        self.foreground_color.set_attributes(reader, e);
                    },
                    b"a:bgClr" => {
                        self.background_color.set_attributes(reader, e);
                    },
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:pattFill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:pattFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:pattFill
        write_start_tag(
            writer,
            "a:pattFill",
            vec![("prst", &self.preset).into()],
            false,
        );

        // a:fgClr
        self.foreground_color.write_to(writer);

        // a:bgClr
        self.background_color.write_to(writer);

        write_end_tag(writer, "a:pattFill");
    }
}
