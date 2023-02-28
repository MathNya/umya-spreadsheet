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
    cap_type: Option<String>,
    compound_line_type: Option<String>,
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

    pub fn get_cap_type(&self) -> &Option<String> {
        &self.cap_type
    }

    pub fn set_cap_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cap_type = Some(value.into());
        self
    }

    pub fn get_compound_line_type(&self) -> &Option<String> {
        &self.compound_line_type
    }

    pub fn set_compound_line_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.compound_line_type = Some(value.into());
        self
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_gradient_fill(&self) -> &Option<GradientFill> {
        &self.gradient_fill
    }

    pub fn get_gradient_fill_mut(&mut self) -> &mut Option<GradientFill> {
        &mut self.gradient_fill
    }

    pub fn set_gradient_fill(&mut self, value: GradientFill) -> &mut Self {
        self.gradient_fill = Some(value);
        self
    }

    pub fn get_tail_end(&self) -> &Option<TailEnd> {
        &self.tail_end
    }

    pub fn get_tail_end_mut(&mut self) -> &mut Option<TailEnd> {
        &mut self.tail_end
    }

    pub fn set_tail_end(&mut self, value: TailEnd) -> &mut Self {
        self.tail_end = Some(value);
        self
    }

    pub fn get_no_fill(&self) -> &Option<NoFill> {
        &self.no_fill
    }

    pub fn get_no_fill_mut(&mut self) -> &mut Option<NoFill> {
        &mut self.no_fill
    }

    pub fn set_no_fill(&mut self, value: NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    pub fn get_bevel(&self) -> &Option<Bevel> {
        &self.bevel
    }

    pub fn get_bevel_mut(&mut self) -> &mut Option<Bevel> {
        &mut self.bevel
    }

    pub fn set_bevel(&mut self, value: Bevel) -> &mut Self {
        self.bevel = Some(value);
        self
    }

    pub fn get_preset_dash(&self) -> &Option<PresetDash> {
        &self.preset_dash
    }

    pub fn get_preset_dash_mut(&mut self) -> &mut Option<PresetDash> {
        &mut self.preset_dash
    }

    pub fn set_preset_dash(&mut self, value: PresetDash) -> &mut Self {
        self.preset_dash = Some(value);
        self
    }

    pub fn get_miter(&self) -> &Option<Miter> {
        &self.miter
    }

    pub fn get_miter_mut(&mut self) -> &mut Option<Miter> {
        &mut self.miter
    }

    pub fn set_miter(&mut self, value: Miter) -> &mut Self {
        self.miter = Some(value);
        self
    }

    pub fn get_round(&self) -> &Option<Round> {
        &self.round
    }

    pub fn get_round_mut(&mut self) -> &mut Option<Round> {
        &mut self.round
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
        let mut buf = Vec::new();

        match get_attribute(e, b"w") {
            Some(v) => {
                self.set_width(v.parse::<u32>().unwrap());
            }
            None => {}
        }

        match get_attribute(e, b"cap") {
            Some(v) => {
                self.set_cap_type(v);
            }
            None => {}
        }

        match get_attribute(e, b"cmpd") {
            Some(v) => {
                self.set_compound_line_type(v);
            }
            None => {}
        }

        match get_attribute(e, b"algn") {
            Some(v) => {
                self.alignment.set_value_string(v);
            }
            None => {}
        }

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:ln" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:ln"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ln
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width = self.width.get_value_string();
        if self.width.has_value() {
            attributes.push(("w", &width));
        }
        match &self.cap_type {
            Some(v) => {
                attributes.push(("cap", v));
            }
            None => {}
        }
        match &self.compound_line_type {
            Some(v) => {
                attributes.push(("cmpd", v));
            }
            None => {}
        }
        if self.alignment.has_value() {
            attributes.push(("algn", (self.alignment.get_value_string())));
        }
        write_start_tag(writer, "a:ln", attributes, false);

        // a:solidFill
        match &self.solid_fill {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:gradFill
        match &self.gradient_fill {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:round
        match &self.round {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:tailEnd
        match &self.tail_end {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:noFill
        match &self.no_fill {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:bevel
        match &self.bevel {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:miter
        match &self.miter {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:prstDash
        match &self.preset_dash {
            Some(v) => v.write_to(writer),
            None => {}
        }

        write_end_tag(writer, "a:ln");
    }
}
