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

    pub fn get_rot(&self) -> Option<&String> {
        self.rot.as_ref()
    }

    pub fn set_rot<S: Into<String>>(&mut self, value: S) {
        self.rot = Some(value.into());
    }

    pub fn get_flip_v(&self) -> Option<&String> {
        self.flip_v.as_ref()
    }

    pub fn set_flip_v<S: Into<String>>(&mut self, value: S) {
        self.flip_v = Some(value.into());
    }

    pub fn get_flip_h(&self) -> Option<&String> {
        self.flip_h.as_ref()
    }

    pub fn set_flip_h<S: Into<String>>(&mut self, value: S) {
        self.flip_h = Some(value.into());
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"rot") {
            self.set_rot(v);
        }

        if let Some(v) = get_attribute(e, b"flipH") {
            self.set_flip_h(v);
        }

        if let Some(v) = get_attribute(e, b"flipV") {
            self.set_flip_v(v);
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:off" => {
                        self.set_x(get_attribute(e, b"x").unwrap().parse::<usize>().unwrap());
                        self.set_y(get_attribute(e, b"y").unwrap().parse::<usize>().unwrap());
                    }
                    b"a:ext" => {
                        self.set_width(get_attribute(e, b"cx").unwrap().parse::<usize>().unwrap());
                        self.set_height(get_attribute(e, b"cy").unwrap().parse::<usize>().unwrap());
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:xfrm" {
                    return;
                }
            },
            Event::Eof => panic!("Error not find {} end element", "a:xfrm")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:xfrm
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = &self.rot {
            attributes.push(("rot", v))
        }
        if let Some(v) = &self.flip_h {
            attributes.push(("flipH", v))
        }
        if let Some(v) = &self.flip_v {
            attributes.push(("flipV", v))
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
