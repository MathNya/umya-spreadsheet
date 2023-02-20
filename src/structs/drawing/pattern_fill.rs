// a:pattFill
use super::BackgroundColor;
use super::ForegroundColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Debug)]
pub struct PatternFill {
    preset: String,
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
}
impl Default for PatternFill {
    fn default() -> Self {
        Self {
            preset: "pct5".into(),
            foreground_color: ForegroundColor::default(),
            background_color: BackgroundColor::default(),
        }
    }
}
impl PatternFill {
    pub fn get_preset(&self) -> &String {
        &self.preset
    }

    pub fn set_preset(&mut self, value: String) -> &mut PatternFill {
        self.preset = value;
        self
    }

    pub fn get_foreground_color(&self) -> &ForegroundColor {
        &self.foreground_color
    }

    pub fn get_foreground_color_mut(&mut self) -> &mut ForegroundColor {
        &mut self.foreground_color
    }

    pub fn set_foreground_color(&mut self, value: ForegroundColor) -> &mut PatternFill {
        self.foreground_color = value;
        self
    }

    pub fn get_background_color(&self) -> &BackgroundColor {
        &self.background_color
    }

    pub fn get_background_color_mut(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }

    pub fn set_background_color(&mut self, value: BackgroundColor) -> &mut PatternFill {
        self.background_color = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"prst") {
            Some(v) => {
                self.set_preset(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:fgClr" => {
                        self.foreground_color.set_attributes(reader, e);
                    }
                    b"a:bgClr" => {
                        self.background_color.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:pattFill" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:pattFill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:pattFill
        write_start_tag(writer, "a:pattFill", vec![("prst", &self.preset)], false);

        // a:fgClr
        let _ = &self.foreground_color.write_to(writer);

        // a:bgClr
        let _ = &self.background_color.write_to(writer);

        write_end_tag(writer, "a:pattFill");
    }
}
