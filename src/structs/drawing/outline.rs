// a:ln
use super::Bevel;
use super::GradientFill;
use super::Miter;
use super::NoFill;
use super::PenAlignmentValues;
use super::PresetDash;
use super::Round;
use super::SolidFill;
use super::TailEnd;
use crate::StringValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::EnumValue;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Outline {
    width: UInt32Value,
    cap_type: StringValue,
    compound_line_type: StringValue,
    solid_fill: Option<SolidFill>,
    gradient_fill: Option<GradientFill>,
    tail_end: Option<TailEnd>,
    no_fill: Option<NoFill>,
    bevel: Option<Bevel>,
    preset_dash: Option<PresetDash>,
    miter: Option<Miter>,
    round: Option<Round>,
    alignment: EnumValue<PenAlignmentValues>,
}

impl Outline {
    pub fn get_width(&self) -> &u32 {
        self.width.get_value()
    }

    pub fn set_width(&mut self, value: u32) -> &mut Self {
        self.width.set_value(value);
        self
    }

    pub fn get_cap_type(&self) -> Option<&str> {
        self.cap_type.get_value()
    }

    pub fn set_cap_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cap_type.set_value(value);
        self
    }

    pub fn get_compound_line_type(&self) -> Option<&str> {
        self.compound_line_type.get_value()
    }

    pub fn set_compound_line_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.compound_line_type.set_value(value);
        self
    }

    pub fn get_solid_fill(&self) -> Option<&SolidFill> {
        self.solid_fill.as_ref()
    }

    pub fn get_solid_fill_mut(&mut self) -> Option<&mut SolidFill> {
        self.solid_fill.as_mut()
    }

    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_gradient_fill(&self) -> Option<&GradientFill> {
        self.gradient_fill.as_ref()
    }

    pub fn get_gradient_fill_mut(&mut self) -> Option<&mut GradientFill> {
        self.gradient_fill.as_mut()
    }

    pub fn set_gradient_fill(&mut self, value: GradientFill) -> &mut Self {
        self.gradient_fill = Some(value);
        self
    }

    pub fn get_tail_end(&self) -> Option<&TailEnd> {
        self.tail_end.as_ref()
    }

    pub fn get_tail_end_mut(&mut self) -> Option<&mut TailEnd> {
        self.tail_end.as_mut()
    }

    pub fn set_tail_end(&mut self, value: TailEnd) -> &mut Self {
        self.tail_end = Some(value);
        self
    }

    pub fn get_no_fill(&self) -> Option<&NoFill> {
        self.no_fill.as_ref()
    }

    pub fn get_no_fill_mut(&mut self) -> Option<&mut NoFill> {
        self.no_fill.as_mut()
    }

    pub fn set_no_fill(&mut self, value: NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    pub fn get_bevel(&self) -> Option<&Bevel> {
        self.bevel.as_ref()
    }

    pub fn get_bevel_mut(&mut self) -> Option<&mut Bevel> {
        self.bevel.as_mut()
    }

    pub fn set_bevel(&mut self, value: Bevel) -> &mut Self {
        self.bevel = Some(value);
        self
    }

    pub fn get_preset_dash(&self) -> Option<&PresetDash> {
        self.preset_dash.as_ref()
    }

    pub fn get_preset_dash_mut(&mut self) -> Option<&mut PresetDash> {
        self.preset_dash.as_mut()
    }

    pub fn set_preset_dash(&mut self, value: PresetDash) -> &mut Self {
        self.preset_dash = Some(value);
        self
    }

    pub fn get_miter(&self) -> Option<&Miter> {
        self.miter.as_ref()
    }

    pub fn get_miter_mut(&mut self) -> Option<&mut Miter> {
        self.miter.as_mut()
    }

    pub fn set_miter(&mut self, value: Miter) -> &mut Self {
        self.miter = Some(value);
        self
    }

    pub fn get_round(&self) -> Option<&Round> {
        self.round.as_ref()
    }

    pub fn get_round_mut(&mut self) -> Option<&mut Round> {
        self.round.as_mut()
    }

    pub fn set_round(&mut self, value: Round) -> &mut Self {
        self.round = Some(value);
        self
    }

    pub fn get_alignment(&self) -> &PenAlignmentValues {
        self.alignment.get_value()
    }

    pub fn set_alignment(&mut self, value: PenAlignmentValues) {
        self.alignment.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"w") {
            self.set_width(v.parse::<u32>().unwrap());
        }

        if let Some(v) = get_attribute(e, b"cap") {
            self.set_cap_type(v);
        }

        if let Some(v) = get_attribute(e, b"cmpd") {
            self.set_compound_line_type(v);
        }

        set_string_from_xml!(self, e, alignment, "algn");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:solidFill" => {
                        let mut solid_fill = SolidFill::default();
                        solid_fill.set_attributes(reader, e);
                        self.set_solid_fill(solid_fill);
                    }
                    b"a:gradFill" => {
                        let mut obj = GradientFill::default();
                        obj.set_attributes(reader, e);
                        self.set_gradient_fill(obj);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:tailEnd" => {
                        let mut obj = TailEnd::default();
                        obj.set_attributes(reader, e);
                        self.set_tail_end(obj);
                    }
                    b"a:noFill" => {
                        let mut obj = NoFill::default();
                        obj.set_attributes(reader, e);
                        self.set_no_fill(obj);
                    }
                    b"a:bevel" => {
                        let mut obj = Bevel::default();
                        obj.set_attributes(reader, e);
                        self.set_bevel(obj);
                    }
                    b"a:miter" => {
                        let mut obj = Miter::default();
                        obj.set_attributes(reader, e);
                        self.set_miter(obj);
                    }
                    b"a:prstDash" => {
                        let mut obj = PresetDash::default();
                        obj.set_attributes(reader, e);
                        self.set_preset_dash(obj);
                    }
                    b"a:round" => {
                        let mut obj = Round::default();
                        obj.set_attributes(reader, e);
                        self.set_round(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if  e.name().into_inner() == b"a:ln" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:ln")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ln
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width = self.width.get_value_string();
        if self.width.has_value() {
            attributes.push(("w", &width));
        }
        if let Some(v) = self.cap_type.get_value() {
            attributes.push(("cap", v));
        }
        if let Some(v) = self.compound_line_type.get_value() {
            attributes.push(("cmpd", v));
        }
        if self.alignment.has_value() {
            attributes.push(("algn", (self.alignment.get_value_string())));
        }
        write_start_tag(writer, "a:ln", attributes, false);

        // a:solidFill
        if let Some(v) = &self.solid_fill {
            v.write_to(writer);
        }

        // a:gradFill
        if let Some(v) = &self.gradient_fill {
            v.write_to(writer);
        }

        // a:round
        if let Some(v) = &self.round {
            v.write_to(writer);
        }

        // a:tailEnd
        if let Some(v) = &self.tail_end {
            v.write_to(writer);
        }

        // a:noFill
        if let Some(v) = &self.no_fill {
            v.write_to(writer);
        }

        // a:bevel
        if let Some(v) = &self.bevel {
            v.write_to(writer);
        }

        // a:prstDash
        if let Some(v) = &self.preset_dash {
            v.write_to(writer);
        }

        // a:miter
        if let Some(v) = &self.miter {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:ln");
    }
}
