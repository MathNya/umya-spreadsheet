// a:bodyPr
use super::super::EnumValue;
use super::super::Int32Value;
use super::ShapeAutoFit;
use super::TextWrappingValues;
use crate::StringValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BodyProperties {
    vert_overflow: StringValue,
    horz_overflow: StringValue,
    rtl_col: StringValue,
    anchor: StringValue,
    wrap: EnumValue<TextWrappingValues>,
    left_inset: Int32Value,
    top_inset: Int32Value,
    right_inset: Int32Value,
    bottom_inset: Int32Value,
    shape_auto_fit: Option<ShapeAutoFit>,
}

impl BodyProperties {
    pub fn get_vert_overflow(&self) -> Option<&str> {
        self.vert_overflow.get_value()
    }

    pub fn set_vert_overflow<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.vert_overflow.set_value(value);
        self
    }

    pub fn get_horz_overflow(&self) -> Option<&str> {
        self.horz_overflow.get_value()
    }

    pub fn set_horz_overflow<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.horz_overflow.set_value(value);
        self
    }

    pub fn get_rtl_col(&self) -> Option<&str> {
        self.rtl_col.get_value()
    }

    pub fn set_rtl_col<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.rtl_col.set_value(value);
        self
    }

    pub fn get_anchor(&self) -> Option<&str> {
        self.anchor.get_value()
    }

    pub fn set_anchor<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.anchor.set_value(value);
        self
    }

    pub fn get_wrap(&self) -> &TextWrappingValues {
        self.wrap.get_value()
    }

    pub fn set_wrap(&mut self, value: TextWrappingValues) -> &mut BodyProperties {
        self.wrap.set_value(value);
        self
    }

    pub fn get_left_inset(&self) -> &i32 {
        self.left_inset.get_value()
    }

    pub fn set_left_inset(&mut self, value: i32) {
        self.left_inset.set_value(value);
    }

    pub fn get_top_inset(&self) -> &i32 {
        self.top_inset.get_value()
    }

    pub fn set_top_inset(&mut self, value: i32) {
        self.top_inset.set_value(value);
    }

    pub fn get_right_inset(&self) -> &i32 {
        self.right_inset.get_value()
    }

    pub fn set_right_inset(&mut self, value: i32) {
        self.right_inset.set_value(value);
    }

    pub fn get_bottom_inset(&self) -> &i32 {
        self.bottom_inset.get_value()
    }

    pub fn set_bottom_inset(&mut self, value: i32) {
        self.bottom_inset.set_value(value);
    }

    pub fn get_shape_auto_fit(&self) -> Option<&ShapeAutoFit> {
        self.shape_auto_fit.as_ref()
    }

    pub fn set_shape_auto_fit(&mut self, value: ShapeAutoFit) -> &mut BodyProperties {
        self.shape_auto_fit = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        for attr in e.attributes().with_checks(false) {
            if let Ok(attr) = attr {
                let key = attr.key.into_inner();
                let value = get_attribute_value(&attr).unwrap();
                match key {
                    b"vertOverflow" => {
                        self.set_vert_overflow(value);
                    }
                    b"horzOverflow" => {
                        self.set_horz_overflow(value);
                    }
                    b"rtlCol" => {
                        self.set_rtl_col(value);
                    }
                    b"anchor" => {
                        self.set_anchor(value);
                    }
                    b"wrap" => {
                        self.wrap.set_value_string(value);
                    }
                    b"lIns" => {
                        self.left_inset.set_value_string(value);
                    }
                    b"tIns" => {
                        self.top_inset.set_value_string(value);
                    }
                    b"rIns" => {
                        self.right_inset.set_value_string(value);
                    }
                    b"bIns" => {
                        self.bottom_inset.set_value_string(value);
                    }
                    _ => {}
                }
            }
        }

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:spAutoFit" {
                    let mut obj = ShapeAutoFit::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_auto_fit(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:bodyPr" {
                     return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:bodyPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = &self.shape_auto_fit.is_none();

        // a:bodyPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = self.vert_overflow.get_value() {
            attributes.push(("vertOverflow", v));
        }
        if let Some(v) = self.horz_overflow.get_value() {
            attributes.push(("horzOverflow", v));
        }
        if let Some(v) = self.rtl_col.get_value() {
            attributes.push(("rtlCol", v));
        }
        if let Some(v) = self.anchor.get_value() {
            attributes.push(("anchor", v));
        }
        if self.wrap.has_value() {
            attributes.push(("wrap", self.wrap.get_value_string()));
        }
        let l_ins = self.left_inset.get_value_string();
        if self.left_inset.has_value() {
            attributes.push(("lIns", &l_ins));
        }
        let t_ins = self.top_inset.get_value_string();
        if self.top_inset.has_value() {
            attributes.push(("tIns", &t_ins));
        }
        let r_ins = self.right_inset.get_value_string();
        if self.right_inset.has_value() {
            attributes.push(("rIns", &r_ins));
        }
        let b_ins = self.bottom_inset.get_value_string();
        if self.bottom_inset.has_value() {
            attributes.push(("bIns", &b_ins));
        }

        write_start_tag(writer, "a:bodyPr", attributes, *empty_flag);

        if !*empty_flag {
            if let Some(v) = &self.shape_auto_fit {
                v.write_to(writer);
            }

            write_end_tag(writer, "a:bodyPr");
        }
    }
}
