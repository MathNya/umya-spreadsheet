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
use structs::Int32Value;
use structs::StringValue;
use structs::TrueFalseValue;
use traits::AdjustmentCoordinate;
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
    optional_number: Int32Value,
    coordinate_size: StringValue,
}

impl Shape {
    pub fn get_style(&self) -> &str {
        self.style.get_value_str()
    }

    pub fn set_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.style.set_value(value);
        self
    }

    pub fn get_type(&self) -> &str {
        self.r_type.get_value_str()
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
        self.fill_color.get_value_str()
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
        self.stroke_color.get_value_str()
    }

    pub fn set_stroke_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.stroke_color.set_value(value);
        self
    }

    pub fn get_stroke_weight(&self) -> &str {
        self.stroke_weight.get_value_str()
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

    pub fn get_fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    pub fn get_fill_mut(&mut self) -> Option<&mut Fill> {
        self.fill.as_mut()
    }

    pub fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub fn get_image_data(&self) -> Option<&ImageData> {
        self.image_data.as_ref()
    }

    pub fn get_image_data_mut(&mut self) -> Option<&mut ImageData> {
        self.image_data.as_mut()
    }

    pub fn set_image_data(&mut self, value: ImageData) -> &mut Self {
        self.image_data = Some(value);
        self
    }

    pub fn get_stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }

    pub fn get_stroke_mut(&mut self) -> Option<&mut Stroke> {
        self.stroke.as_mut()
    }

    pub fn set_stroke(&mut self, value: Stroke) -> &mut Self {
        self.stroke = Some(value);
        self
    }

    pub fn get_shadow(&self) -> Option<&Shadow> {
        self.shadow.as_ref()
    }

    pub fn get_shadow_mut(&mut self) -> Option<&mut Shadow> {
        self.shadow.as_mut()
    }

    pub fn set_shadow(&mut self, value: Shadow) -> &mut Self {
        self.shadow = Some(value);
        self
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_ref()
    }

    pub fn get_path_mut(&mut self) -> Option<&mut Path> {
        self.path.as_mut()
    }

    pub fn set_path(&mut self, value: Path) -> &mut Self {
        self.path = Some(value);
        self
    }

    pub fn get_text_box(&self) -> Option<&TextBox> {
        self.text_box.as_ref()
    }

    pub fn get_text_box_mut(&mut self) -> Option<&mut TextBox> {
        self.text_box.as_mut()
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

    pub fn get_optional_number(&self) -> &i32 {
        self.optional_number.get_value()
    }

    pub fn set_optional_number(&mut self, value: i32) -> &mut Self {
        self.optional_number.set_value(value);
        self
    }

    pub fn get_coordinate_size(&self) -> &str {
        self.coordinate_size.get_value_str()
    }

    pub fn set_coordinate_size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.coordinate_size.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, r_type, "type");
        set_string_from_xml!(self, e, style, "style");
        set_string_from_xml!(self, e, filled, "filled");
        set_string_from_xml!(self, e, fill_color, "fillcolor");
        set_string_from_xml!(self, e, stroked, "stoked");
        set_string_from_xml!(self, e, stroke_color, "stokecolor");
        set_string_from_xml!(self, e, stroke_weight, "stokeweight");
        set_string_from_xml!(self, e, inset_mode, "o:insetmode");
        set_string_from_xml!(self, e, optional_number, "o:spt");
        set_string_from_xml!(self, e, coordinate_size, "coordsize");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::End(ref e) => {
                if  e.name().into_inner() == b"v:shape" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "v:shape")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, id: &usize, r_id: &usize) {
        // v:shape
        let id_str = format!("_x0000_s{}", id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("id", &id_str));
        if self.r_type.has_value() {
            attributes.push(("type", self.r_type.get_value_str()));
        }
        if self.style.has_value() {
            attributes.push(("style", self.style.get_value_str()));
        }
        if self.filled.has_value() {
            attributes.push(("filled", self.filled.get_value_string()));
        }
        if self.fill_color.has_value() {
            attributes.push(("fillcolor", self.fill_color.get_value_str()));
        }
        if self.stroked.has_value() {
            attributes.push(("stroked", self.stroked.get_value_string()));
        }
        if self.stroke_color.has_value() {
            attributes.push(("strokecolor", self.stroke_color.get_value_str()));
        }
        if self.stroke_weight.has_value() {
            attributes.push(("strokeweight", self.stroke_weight.get_value_str()));
        }
        if self.inset_mode.has_value() {
            attributes.push(("o:insetmode", self.inset_mode.get_value_string()));
        }
        let optional_number_str = self.optional_number.get_value_string();
        if self.optional_number.has_value() {
            attributes.push(("o:spt", &optional_number_str));
        }
        if self.coordinate_size.has_value() {
            attributes.push(("coordsize", self.coordinate_size.get_value_str()));
        }
        write_start_tag(writer, "v:shape", attributes, false);

        // v:fill
        if let Some(v) = &self.fill {
            v.write_to(writer);
        }

        // v:shadow
        if let Some(v) = &self.shadow {
            v.write_to(writer);
        }

        // v:path
        if let Some(v) = &self.path {
            v.write_to(writer);
        }

        // v:textbox
        if let Some(v) = &self.text_box {
            v.write_to(writer);
        }

        // v:stroke
        if let Some(v) = &self.stroke {
            v.write_to(writer);
        }

        // v:imagedata
        if let Some(v) = &self.image_data {
            v.write_to(writer, r_id);
        }

        // x:ClientData
        self.client_data.write_to(writer);

        write_end_tag(writer, "v:shape");
    }
}
impl AdjustmentCoordinate for Shape {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.client_data.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.client_data.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
