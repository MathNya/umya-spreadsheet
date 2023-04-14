use super::office::InsetMarginValues;
use super::spreadsheet::ClientData;
use super::Fill;
use super::ImageData;
use super::Path;
use super::Shadow;
use super::Stroke;
use super::TextBox;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::EnumValue;
use structs::StringValue;
use structs::TrueFalseValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Shape {
    style: StringValue,
    r_type: StringValue,
    filled: TrueFalseValue,
    fill_color: StringValue,
    stroked: TrueFalseValue,
    stroke_color: StringValue,
    stroke_weight: StringValue,
    inset_mode: EnumValue<InsetMarginValues>,
    fill: Option<Fill>,
    image_data: Option<ImageData>,
    stroke: Option<Stroke>,
    shadow: Option<Shadow>,
    path: Option<Path>,
    text_box: Option<TextBox>,
    client_data: ClientData,
}
impl Shape {
    pub fn get_style(&self) -> &str {
        self.style.get_value()
    }

    pub fn set_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.style.set_value(value);
        self
    }

    pub fn get_type(&self) -> &str {
        self.r_type.get_value()
    }

    pub fn set_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.r_type.set_value(value);
        self
    }

    pub fn get_filled(&self) -> &bool {
        self.filled.get_value()
    }

    pub fn set_filled(&mut self, value: bool) -> &mut Self {
        self.filled.set_value(value);
        self
    }

    pub fn get_fill_color(&self) -> &str {
        self.fill_color.get_value()
    }

    pub fn set_fill_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.fill_color.set_value(value);
        self
    }

    pub fn get_stroked(&self) -> &bool {
        self.stroked.get_value()
    }

    pub fn set_stroked(&mut self, value: bool) -> &mut Self {
        self.stroked.set_value(value);
        self
    }

    pub fn get_stroke_color(&self) -> &str {
        self.stroke_color.get_value()
    }

    pub fn set_stroke_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.stroke_color.set_value(value);
        self
    }

    pub fn get_stroke_weight(&self) -> &str {
        self.stroke_weight.get_value()
    }

    pub fn set_stroke_weight<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.stroke_weight.set_value(value);
        self
    }

    pub fn get_inset_mode(&self) -> &InsetMarginValues {
        self.inset_mode.get_value()
    }

    pub fn set_inset_mode(&mut self, value: InsetMarginValues) -> &mut Self {
        self.inset_mode.set_value(value);
        self
    }

    pub fn get_fill(&self) -> &Option<Fill> {
        &self.fill
    }

    pub fn get_fill_mut(&mut self) -> &mut Option<Fill> {
        &mut self.fill
    }

    pub fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub fn get_image_data(&self) -> &Option<ImageData> {
        &self.image_data
    }

    pub fn get_image_data_mut(&mut self) -> &mut Option<ImageData> {
        &mut self.image_data
    }

    pub fn set_image_data(&mut self, value: ImageData) -> &mut Self {
        self.image_data = Some(value);
        self
    }

    pub fn get_stroke(&self) -> &Option<Stroke> {
        &self.stroke
    }

    pub fn get_stroke_mut(&mut self) -> &mut Option<Stroke> {
        &mut self.stroke
    }

    pub fn set_stroke(&mut self, value: Stroke) -> &mut Self {
        self.stroke = Some(value);
        self
    }

    pub fn get_shadow(&self) -> &Option<Shadow> {
        &self.shadow
    }

    pub fn get_shadow_mut(&mut self) -> &mut Option<Shadow> {
        &mut self.shadow
    }

    pub fn set_shadow(&mut self, value: Shadow) -> &mut Self {
        self.shadow = Some(value);
        self
    }

    pub fn get_path(&self) -> &Option<Path> {
        &self.path
    }

    pub fn get_path_mut(&mut self) -> &mut Option<Path> {
        &mut self.path
    }

    pub fn set_path(&mut self, value: Path) -> &mut Self {
        self.path = Some(value);
        self
    }

    pub fn get_text_box(&self) -> &Option<TextBox> {
        &self.text_box
    }

    pub fn get_text_box_mut(&mut self) -> &mut Option<TextBox> {
        &mut self.text_box
    }

    pub fn set_text_box(&mut self, value: TextBox) -> &mut Self {
        self.text_box = Some(value);
        self
    }

    pub fn get_client_data(&self) -> &ClientData {
        &self.client_data
    }

    pub fn get_client_data_mut(&mut self) -> &mut ClientData {
        &mut self.client_data
    }

    pub fn set_client_data(&mut self, value: ClientData) -> &mut Self {
        self.client_data = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        match get_attribute(e, b"type") {
            Some(v) => {
                self.r_type.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"style") {
            Some(v) => {
                self.style.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"filled") {
            Some(v) => {
                self.filled.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"fillcolor") {
            Some(v) => {
                self.fill_color.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"stroked") {
            Some(v) => {
                self.stroked.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"strokecolor") {
            Some(v) => {
                self.stroke_color.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"strokeweight") {
            Some(v) => {
                self.stroke_weight.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"o:insetmode") {
            Some(v) => {
                self.inset_mode.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"v:fill" => {
                        let mut obj = Fill::default();
                        obj.set_attributes(reader, e);
                        self.set_fill(obj);
                    }
                    b"v:shadow" => {
                        let mut obj = Shadow::default();
                        obj.set_attributes(reader, e);
                        self.set_shadow(obj);
                    }
                    b"v:path" => {
                        let mut obj = Path::default();
                        obj.set_attributes(reader, e);
                        self.set_path(obj);
                    }
                    b"v:stroke" => {
                        let mut obj = Stroke::default();
                        obj.set_attributes(reader, e);
                        self.set_stroke(obj);
                    }
                    b"v:imagedata" => {
                        let mut obj = ImageData::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_image_data(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"v:textbox" => {
                        let mut obj = TextBox::default();
                        obj.set_attributes(reader, e);
                        self.set_text_box(obj);
                    }
                    b"x:ClientData" => {
                        let mut obj = ClientData::default();
                        obj.set_attributes(reader, e);
                        self.set_client_data(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"v:shape" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "v:shape"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, id: &usize, r_id: &usize) {
        // v:shape
        let id_str = format!("_x0000_s{}", id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("id", &id_str));
        if self.r_type.has_value() {
            attributes.push(("type", self.r_type.get_value_string()));
        }
        if self.style.has_value() {
            attributes.push(("style", self.style.get_value_string()));
        }
        if self.filled.has_value() {
            attributes.push(("filled", self.filled.get_value_string()));
        }
        if self.fill_color.has_value() {
            attributes.push(("fillcolor", self.fill_color.get_value_string()));
        }
        if self.stroked.has_value() {
            attributes.push(("stroked", self.stroked.get_value_string()));
        }
        if self.stroke_color.has_value() {
            attributes.push(("strokecolor", self.stroke_color.get_value_string()));
        }
        if self.stroke_weight.has_value() {
            attributes.push(("strokeweight", self.stroke_weight.get_value_string()));
        }
        if self.inset_mode.has_value() {
            attributes.push(("o:insetmode", self.inset_mode.get_value_string()));
        }
        write_start_tag(writer, "v:shape", attributes, false);

        // v:fill
        match &self.fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // v:shadow
        match &self.shadow {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // v:path
        match &self.path {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // v:textbox
        match &self.text_box {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // v:stroke
        match &self.stroke {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // v:imagedata
        match &self.image_data {
            Some(v) => {
                v.write_to(writer, r_id);
            }
            None => {}
        }

        // x:ClientData
        self.client_data.write_to(writer);

        write_end_tag(writer, "v:shape");
    }
}
