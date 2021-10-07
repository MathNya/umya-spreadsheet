// mruColors
use super::Color;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub(crate) struct MruColors {
    color: Vec<Color>,
}
impl MruColors {
    pub(crate) fn get_color(&self)-> &Vec<Color> {
        &self.color
    }

    pub(crate) fn get_color_mut(&mut self)-> &mut Vec<Color> {
        &mut self.color
    }

    pub(crate) fn set_color(&mut self, value:Color)-> &mut Self {
        self.color.push(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"color" => {
                            let mut obj = Color::default();
                            obj.set_attributes(reader, e);
                            self.set_color(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"mruColors" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mruColors"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.color.len() > 0 {
            // mruColors
            write_start_tag(writer, "mruColors", vec![], false);

            // color
            for color in &self.color {
                color.write_to_color(writer);
            }

            write_end_tag(writer, "mruColors");
        }
    }
}
