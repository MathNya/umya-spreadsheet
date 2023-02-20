// a:xfrm
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Transform2D {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    rot: Option<String>,
    flip_v: Option<String>,
    flip_h: Option<String>,
}
impl Transform2D {
    pub fn get_x(&self) -> &usize {
        &self.x
    }

    pub fn set_x(&mut self, value: usize) {
        self.x = value;
    }

    pub fn get_y(&self) -> &usize {
        &self.y
    }

    pub fn set_y(&mut self, value: usize) {
        self.y = value;
    }

    pub fn get_width(&self) -> &usize {
        &self.width
    }

    pub fn set_width(&mut self, value: usize) {
        self.width = value;
    }

    pub fn get_height(&self) -> &usize {
        &self.height
    }

    pub fn set_height(&mut self, value: usize) {
        self.height = value;
    }

    pub fn get_rot(&self) -> &Option<String> {
        &self.rot
    }

    pub fn set_rot<S: Into<String>>(&mut self, value: S) {
        self.rot = Some(value.into());
    }

    pub fn get_flip_v(&self) -> &Option<String> {
        &self.flip_v
    }

    pub fn set_flip_v<S: Into<String>>(&mut self, value: S) {
        self.flip_v = Some(value.into());
    }

    pub fn get_flip_h(&self) -> &Option<String> {
        &self.flip_h
    }

    pub fn set_flip_h<S: Into<String>>(&mut self, value: S) {
        self.flip_h = Some(value.into());
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();

        match get_attribute(e, b"rot") {
            Some(v) => {
                self.set_rot(v);
            }
            None => {}
        }

        match get_attribute(e, b"flipH") {
            Some(v) => {
                self.set_flip_h(v);
            }
            None => {}
        }

        match get_attribute(e, b"flipV") {
            Some(v) => {
                self.set_flip_v(v);
            }
            None => {}
        }

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:off" => {
                        self.set_x(get_attribute(e, b"x").unwrap().parse::<usize>().unwrap());
                        self.set_y(get_attribute(e, b"y").unwrap().parse::<usize>().unwrap());
                    }
                    b"a:ext" => {
                        self.set_width(get_attribute(e, b"cx").unwrap().parse::<usize>().unwrap());
                        self.set_height(get_attribute(e, b"cy").unwrap().parse::<usize>().unwrap());
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:xfrm" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:xfrm"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:xfrm
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.rot {
            Some(v) => attributes.push(("rot", v)),
            None => {}
        }
        match &self.flip_h {
            Some(v) => attributes.push(("flipH", v)),
            None => {}
        }
        match &self.flip_v {
            Some(v) => attributes.push(("flipV", v)),
            None => {}
        }
        write_start_tag(writer, "a:xfrm", attributes, false);

        // a:off
        write_start_tag(
            writer,
            "a:off",
            vec![("x", &self.x.to_string()), ("y", &self.y.to_string())],
            true,
        );

        // a:ext
        write_start_tag(
            writer,
            "a:ext",
            vec![
                ("cx", &self.width.to_string()),
                ("cy", &self.height.to_string()),
            ],
            true,
        );

        write_end_tag(writer, "a:xfrm");
    }
}
