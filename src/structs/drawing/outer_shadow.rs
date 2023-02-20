// a:outerShdw
use super::PresetColor;
use super::RgbColorModelHex;
use super::SchemeColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OuterShadow {
    blur_radius: Option<String>,
    alignment: Option<String>,
    horizontal_ratio: Option<String>,
    vertical_ratio: Option<String>,
    direction: Option<String>,
    distance: Option<String>,
    rotate_with_shape: Option<String>,
    preset_color: Option<PresetColor>,
    scheme_color: Option<SchemeColor>,
    rgb_color_model_hex: Option<RgbColorModelHex>,
}
impl OuterShadow {
    pub fn get_blur_radius(&self) -> &Option<String> {
        &self.blur_radius
    }

    pub fn set_blur_radius<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.blur_radius = Some(value.into());
        self
    }

    pub fn get_horizontal_ratio(&self) -> &Option<String> {
        &self.horizontal_ratio
    }

    pub fn set_horizontal_ratio<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.horizontal_ratio = Some(value.into());
        self
    }

    pub fn get_vertical_ratio(&self) -> &Option<String> {
        &self.vertical_ratio
    }

    pub fn set_vertical_ratio<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.vertical_ratio = Some(value.into());
        self
    }

    pub fn get_alignment(&self) -> &Option<String> {
        &self.alignment
    }

    pub fn set_alignment<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.alignment = Some(value.into());
        self
    }

    pub fn get_direction(&self) -> &Option<String> {
        &self.direction
    }

    pub fn set_direction<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.direction = Some(value.into());
        self
    }

    pub fn get_distance(&self) -> &Option<String> {
        &self.distance
    }

    pub fn set_distance<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.distance = Some(value.into());
        self
    }

    pub fn get_rotate_with_shape(&self) -> &Option<String> {
        &self.rotate_with_shape
    }

    pub fn set_rotate_with_shape<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.rotate_with_shape = Some(value.into());
        self
    }

    pub fn get_preset_color(&self) -> &Option<PresetColor> {
        &self.preset_color
    }

    pub fn get_preset_color_mut(&mut self) -> &mut Option<PresetColor> {
        &mut self.preset_color
    }

    pub fn set_preset_color(&mut self, value: PresetColor) -> &mut Self {
        self.preset_color = Some(value);
        self
    }

    pub fn get_scheme_color(&self) -> &Option<SchemeColor> {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut Option<SchemeColor> {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut Self {
        self.scheme_color = Some(value);
        self
    }

    pub fn get_rgb_color_model_hex(&self) -> &Option<RgbColorModelHex> {
        &self.rgb_color_model_hex
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> &mut Option<RgbColorModelHex> {
        &mut self.rgb_color_model_hex
    }

    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) -> &mut Self {
        self.rgb_color_model_hex = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"blurRad") {
            Some(v) => {
                self.set_blur_radius(v);
            }
            None => {}
        }
        match get_attribute(e, b"dist") {
            Some(v) => {
                self.set_distance(v);
            }
            None => {}
        }
        match get_attribute(e, b"dir") {
            Some(v) => {
                self.set_direction(v);
            }
            None => {}
        }
        match get_attribute(e, b"sx") {
            Some(v) => {
                self.set_horizontal_ratio(v);
            }
            None => {}
        }
        match get_attribute(e, b"sy") {
            Some(v) => {
                self.set_vertical_ratio(v);
            }
            None => {}
        }
        match get_attribute(e, b"algn") {
            Some(v) => {
                self.set_alignment(v);
            }
            None => {}
        }
        match get_attribute(e, b"rotWithShape") {
            Some(v) => {
                self.set_rotate_with_shape(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, true);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, true);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:prstClr" => {
                        let mut obj = PresetColor::default();
                        obj.set_attributes(reader, e);
                        self.set_preset_color(obj);
                    }
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, false);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, false);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:outerShdw" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:outerShdw"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:outerShdw
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.blur_radius {
            Some(v) => {
                attributes.push(("blurRad", v));
            }
            None => {}
        }
        match &self.distance {
            Some(v) => {
                attributes.push(("dist", v));
            }
            None => {}
        }
        match &self.direction {
            Some(v) => {
                attributes.push(("dir", v));
            }
            None => {}
        }
        match &self.horizontal_ratio {
            Some(v) => {
                attributes.push(("sx", v));
            }
            None => {}
        }
        match &self.vertical_ratio {
            Some(v) => {
                attributes.push(("sy", v));
            }
            None => {}
        }
        match &self.alignment {
            Some(v) => {
                attributes.push(("algn", v));
            }
            None => {}
        }
        match &self.rotate_with_shape {
            Some(v) => {
                attributes.push(("rotWithShape", v));
            }
            None => {}
        }
        write_start_tag(writer, "a:outerShdw", attributes, false);

        // a:prstClr
        match &self.preset_color {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:schemeClr
        match &self.scheme_color {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:srgbClr
        match &self.rgb_color_model_hex {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "a:outerShdw");
    }
}
