// a:bodyPr
use super::super::EnumValue;
use super::super::Int32Value;
use super::ShapeAutoFit;
use super::TextWrappingValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BodyProperties {
    vert_overflow: Option<String>,
    horz_overflow: Option<String>,
    rtl_col: Option<String>,
    anchor: Option<String>,
    wrap: EnumValue<TextWrappingValues>,
    left_inset: Int32Value,
    top_inset: Int32Value,
    right_inset: Int32Value,
    bottom_inset: Int32Value,
    shape_auto_fit: Option<ShapeAutoFit>,
}
impl BodyProperties {
    pub fn get_vert_overflow(&self) -> &Option<String> {
        &self.vert_overflow
    }

    pub fn set_vert_overflow<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.vert_overflow = Some(value.into());
        self
    }

    pub fn get_horz_overflow(&self) -> &Option<String> {
        &self.horz_overflow
    }

    pub fn set_horz_overflow<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.horz_overflow = Some(value.into());
        self
    }

    pub fn get_rtl_col(&self) -> &Option<String> {
        &self.rtl_col
    }

    pub fn set_rtl_col<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.rtl_col = Some(value.into());
        self
    }

    pub fn get_anchor(&self) -> &Option<String> {
        &self.anchor
    }

    pub fn set_anchor<S: Into<String>>(&mut self, value: S) -> &mut BodyProperties {
        self.anchor = Some(value.into());
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

    pub fn get_shape_auto_fit(&self) -> &Option<ShapeAutoFit> {
        &self.shape_auto_fit
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
        for a in e.attributes().with_checks(false) {
            match a {
                Ok(ref attr) if attr.key.into_inner() == b"vertOverflow" => {
                    self.set_vert_overflow(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"horzOverflow" => {
                    self.set_horz_overflow(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"rtlCol" => {
                    self.set_rtl_col(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"anchor" => {
                    self.set_anchor(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"wrap" => {
                    self.wrap
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"lIns" => {
                    self.left_inset
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"tIns" => {
                    self.top_inset
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"rIns" => {
                    self.right_inset
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.into_inner() == b"bIns" => {
                    self.bottom_inset
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:spAutoFit" => {
                        let mut obj = ShapeAutoFit::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_auto_fit(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:bodyPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:bodyPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = &self.shape_auto_fit.is_none();

        // a:bodyPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.vert_overflow {
            Some(v) => attributes.push(("vertOverflow", v)),
            None => {}
        }
        match &self.horz_overflow {
            Some(v) => attributes.push(("horzOverflow", v)),
            None => {}
        }
        match &self.rtl_col {
            Some(v) => attributes.push(("rtlCol", v)),
            None => {}
        }
        match &self.anchor {
            Some(v) => attributes.push(("anchor", v)),
            None => {}
        }
        if &self.wrap.has_value() == &true {
            attributes.push(("wrap", self.wrap.get_value_string()));
        }
        let l_ins = self.left_inset.get_value_string();
        if &self.left_inset.has_value() == &true {
            attributes.push(("lIns", &l_ins));
        }
        let t_ins = self.top_inset.get_value_string();
        if &self.top_inset.has_value() == &true {
            attributes.push(("tIns", &t_ins));
        }
        let r_ins = self.right_inset.get_value_string();
        if &self.right_inset.has_value() == &true {
            attributes.push(("rIns", &r_ins));
        }
        let b_ins = self.bottom_inset.get_value_string();
        if &self.bottom_inset.has_value() == &true {
            attributes.push(("bIns", &b_ins));
        }

        write_start_tag(writer, "a:bodyPr", attributes, *empty_flag);

        if empty_flag == &false {
            match &self.shape_auto_fit {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "a:bodyPr");
        }
    }
}
