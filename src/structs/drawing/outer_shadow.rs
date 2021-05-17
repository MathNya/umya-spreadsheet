// a:outerShdw
use super::preset_color::PresetColor;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct OuterShadow {
    alignment: Option<String>,
    blur_radius: Option<i64>,
    direction: Option<i32>,
    distance: Option<i64>,
    rotate_with_shape: Option<bool>,
    preset_color: Option<PresetColor>,
}
impl OuterShadow {
    pub fn get_alignment(&self) -> &Option<String> {
        &self.alignment
    }

    pub fn set_alignment<S: Into<String>>(&mut self, value:S) {
        self.alignment = Some(value.into());
    }

    pub fn get_blur_radius(&self) -> &Option<i64> {
        &self.blur_radius
    }

    pub fn set_blur_radius(&mut self, value:i64) {
        self.blur_radius = Some(value);
    }

    pub fn get_direction(&self) -> &Option<i32> {
        &self.direction
    }

    pub fn set_direction(&mut self, value:i32) {
        self.direction = Some(value);
    }

    pub fn get_distance(&self) -> &Option<i64> {
        &self.distance
    }

    pub fn set_distance(&mut self, value:i64) {
        self.distance = Some(value);
    }

    pub fn get_rotate_with_shape(&self) -> &Option<bool> {
        &self.rotate_with_shape
    }

    pub fn set_rotate_with_shape(&mut self, value:bool) {
        self.rotate_with_shape = Some(value);
    }

    pub fn get_preset_color(&self) -> &Option<PresetColor> {
        &self.preset_color
    }

    pub fn set_preset_color(&mut self, value:PresetColor) {
        self.preset_color = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"algn") {
            Some(v) => {&mut self.set_alignment(v);},
            None => {}
        }
        match get_attribute(e, b"blurRad") {
            Some(v) => {&mut self.set_blur_radius(v.parse::<i64>().unwrap());},
            None => {}
        }
        match get_attribute(e, b"dir") {
            Some(v) => {&mut self.set_direction(v.parse::<i32>().unwrap());},
            None => {}
        }
        match get_attribute(e, b"dist") {
            Some(v) => {&mut self.set_distance(v.parse::<i64>().unwrap());},
            None => {}
        }
        match get_attribute(e, b"rotWithShape") {
            Some(v) => {
                match &*v {
                    "1" => {&mut self.set_rotate_with_shape(true);},
                    "0" => {&mut self.set_rotate_with_shape(false);},
                     _ => {}
                };
            },
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:prstClr" => {
                            let mut preset_color = PresetColor::default();
                            preset_color.set_attributes(reader, e);
                            &mut self.set_preset_color(preset_color);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:outerShdw" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:outerShdw"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ln
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let mut blur_rad_str = String::default();
        match &self.blur_radius {
            Some(v) => {
                blur_rad_str = v.to_string();
                attributes.push(("blurRad", &blur_rad_str));
            },
            None => {}
        }
        let mut distance_str = String::from("");
        match &self.distance {
            Some(v) => {
                distance_str = v.to_string();
                attributes.push(("dist", &distance_str));
            },
            None => {}
        }
        let mut direction_str = String::from("");
        match &self.direction {
            Some(v) => {
                direction_str = v.to_string();
                attributes.push(("dir", &direction_str));
            },
            None => {}
        }
        match &self.alignment {
            Some(v) => {attributes.push(("algn", &v));},
            None => {}
        }
        match &self.rotate_with_shape {
            Some(v) => {
                match v {
                    true => attributes.push(("rotWithShape", "1")),
                    false => attributes.push(("rotWithShape", "0")),
                }
            },
            None => {}
        }
        write_start_tag(writer, "a:outerShdw", attributes, false);

        // a:prstClr
        match &self.preset_color {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        write_end_tag(writer, "a:outerShdw");
    }
}
