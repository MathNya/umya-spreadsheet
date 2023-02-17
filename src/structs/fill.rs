use super::GradientFill;
use super::PatternFill;
use super::PatternValues;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Fill {
    pattern_fill: Option<PatternFill>,
    gradient_fill: Option<GradientFill>,
}
impl Fill {
    pub fn get_pattern_fill(&self) -> &Option<PatternFill> {
        &self.pattern_fill
    }

    pub fn get_pattern_fill_mut(&mut self) -> &mut PatternFill {
        match &self.pattern_fill {
            Some(_) => return self.pattern_fill.as_mut().unwrap(),
            None => {}
        }
        self.set_pattern_fill(PatternFill::default());
        self.pattern_fill.as_mut().unwrap()
    }

    pub fn set_pattern_fill(&mut self, value: PatternFill) -> &mut Self {
        self.pattern_fill = Some(value);
        self.gradient_fill = None;
        self
    }

    pub fn get_gradient_fill(&self) -> &Option<GradientFill> {
        &self.gradient_fill
    }

    pub fn get_gradient_fill_mut(&mut self) -> &mut GradientFill {
        match &self.gradient_fill {
            Some(_) => return self.gradient_fill.as_mut().unwrap(),
            None => {}
        }
        self.set_gradient_fill(GradientFill::default());
        self.gradient_fill.as_mut().unwrap()
    }

    pub fn set_gradient_fill(&mut self, value: GradientFill) -> &mut Self {
        self.pattern_fill = None;
        self.gradient_fill = Some(value);
        self
    }

    pub(crate) fn get_defalut_value() -> Self {
        let mut def = Self::default();
        let mut pfill = PatternFill::default();
        pfill.set_pattern_type(PatternValues::None);
        def.set_pattern_fill(pfill);
        def
    }

    pub(crate) fn get_defalut_value_2() -> Self {
        let mut def = Self::default();
        let mut pfill = PatternFill::default();
        pfill.set_pattern_type(PatternValues::Gray125);
        def.set_pattern_fill(pfill);
        def
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                match &self.pattern_fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "NONE".to_string()
                    }
                },
                match &self.gradient_fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "NONE".to_string()
                    }
                },
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"patternFill" => {
                        let mut obj = PatternFill::default();
                        obj.set_attributes(reader, e, true);
                        self.set_pattern_fill(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"patternFill" => {
                        let mut obj = PatternFill::default();
                        obj.set_attributes(reader, e, false);
                        self.set_pattern_fill(obj);
                    }
                    b"gradientFill" => {
                        let mut obj = GradientFill::default();
                        obj.set_attributes(reader, e);
                        self.set_gradient_fill(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"fill" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "fill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // fill
        write_start_tag(writer, "fill", vec![], false);

        // gradientFill
        match &self.pattern_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // patternFill
        match &self.gradient_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "fill");
    }
}
