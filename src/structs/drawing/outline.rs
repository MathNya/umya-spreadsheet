// a:ln
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Bevel,
    GradientFill,
    Miter,
    NoFill,
    PenAlignmentValues,
    PresetDash,
    Round,
    SolidFill,
    TailEnd,
};
use crate::{
    StringValue,
    drawing::SystemColor,
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::{
        EnumValue,
        UInt32Value,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Outline {
    width:              UInt32Value,
    cap_type:           StringValue,
    compound_line_type: StringValue,
    solid_fill:         Option<Box<SolidFill>>,
    gradient_fill:      Option<Box<GradientFill>>,
    tail_end:           Option<Box<TailEnd>>,
    no_fill:            Option<NoFill>,
    bevel:              Option<Box<Bevel>>,
    preset_dash:        Option<PresetDash>,
    miter:              Option<Miter>,
    round:              Option<Round>,
    alignment:          EnumValue<PenAlignmentValues>,
    system_color:       Option<Box<SystemColor>>,
}

impl Outline {
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use width()")]
    pub fn get_width(&self) -> u32 {
        self.width()
    }

    #[inline]
    pub fn set_width(&mut self, value: u32) -> &mut Self {
        self.width.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn cap_type(&self) -> Option<&str> {
        self.cap_type.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cap_type()")]
    pub fn get_cap_type(&self) -> Option<&str> {
        self.cap_type()
    }

    #[inline]
    pub fn set_cap_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cap_type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn compound_line_type(&self) -> Option<&str> {
        self.compound_line_type.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use compound_line_type()")]
    pub fn get_compound_line_type(&self) -> Option<&str> {
        self.compound_line_type()
    }

    #[inline]
    pub fn set_compound_line_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.compound_line_type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn solid_fill(&self) -> Option<&SolidFill> {
        self.solid_fill.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use solid_fill()")]
    pub fn get_solid_fill(&self) -> Option<&SolidFill> {
        self.solid_fill()
    }

    #[inline]
    pub fn solid_fill_mut(&mut self) -> Option<&mut SolidFill> {
        self.solid_fill.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use solid_fill_mut()")]
    pub fn get_solid_fill_mut(&mut self) -> Option<&mut SolidFill> {
        self.solid_fill_mut()
    }

    #[inline]
    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn gradient_fill(&self) -> Option<&GradientFill> {
        self.gradient_fill.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use gradient_fill()")]
    pub fn get_gradient_fill(&self) -> Option<&GradientFill> {
        self.gradient_fill()
    }

    #[inline]
    pub fn gradient_fill_mut(&mut self) -> Option<&mut GradientFill> {
        self.gradient_fill.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use gradient_fill_mut()")]
    pub fn get_gradient_fill_mut(&mut self) -> Option<&mut GradientFill> {
        self.gradient_fill_mut()
    }

    #[inline]
    pub fn set_gradient_fill(&mut self, value: GradientFill) -> &mut Self {
        self.gradient_fill = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn tail_end(&self) -> Option<&TailEnd> {
        self.tail_end.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tail_end()")]
    pub fn get_tail_end(&self) -> Option<&TailEnd> {
        self.tail_end()
    }

    #[inline]
    pub fn tail_end_mut(&mut self) -> Option<&mut TailEnd> {
        self.tail_end.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use tail_end_mut()")]
    pub fn get_tail_end_mut(&mut self) -> Option<&mut TailEnd> {
        self.tail_end_mut()
    }

    #[inline]
    pub fn set_tail_end(&mut self, value: TailEnd) -> &mut Self {
        self.tail_end = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn no_fill(&self) -> Option<&NoFill> {
        self.no_fill.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use no_fill()")]
    pub fn get_no_fill(&self) -> Option<&NoFill> {
        self.no_fill()
    }

    #[inline]
    pub fn no_fill_mut(&mut self) -> Option<&mut NoFill> {
        self.no_fill.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use no_fill_mut()")]
    pub fn get_no_fill_mut(&mut self) -> Option<&mut NoFill> {
        self.no_fill_mut()
    }

    #[inline]
    pub fn set_no_fill(&mut self, value: NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn bevel(&self) -> Option<&Bevel> {
        self.bevel.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bevel()")]
    pub fn get_bevel(&self) -> Option<&Bevel> {
        self.bevel()
    }

    #[inline]
    pub fn bevel_mut(&mut self) -> Option<&mut Bevel> {
        self.bevel.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use bevel_mut()")]
    pub fn get_bevel_mut(&mut self) -> Option<&mut Bevel> {
        self.bevel_mut()
    }

    #[inline]
    pub fn set_bevel(&mut self, value: Bevel) -> &mut Self {
        self.bevel = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn preset_dash(&self) -> Option<&PresetDash> {
        self.preset_dash.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use preset_dash()")]
    pub fn get_preset_dash(&self) -> Option<&PresetDash> {
        self.preset_dash()
    }

    #[inline]
    pub fn preset_dash_mut(&mut self) -> Option<&mut PresetDash> {
        self.preset_dash.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use preset_dash_mut()")]
    pub fn get_preset_dash_mut(&mut self) -> Option<&mut PresetDash> {
        self.preset_dash_mut()
    }

    #[inline]
    pub fn set_preset_dash(&mut self, value: PresetDash) -> &mut Self {
        self.preset_dash = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn miter(&self) -> Option<&Miter> {
        self.miter.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use miter()")]
    pub fn get_miter(&self) -> Option<&Miter> {
        self.miter()
    }

    #[inline]
    pub fn miter_mut(&mut self) -> Option<&mut Miter> {
        self.miter.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use miter_mut()")]
    pub fn get_miter_mut(&mut self) -> Option<&mut Miter> {
        self.miter_mut()
    }

    #[inline]
    pub fn set_miter(&mut self, value: Miter) -> &mut Self {
        self.miter = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn round(&self) -> Option<&Round> {
        self.round.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use round()")]
    pub fn get_round(&self) -> Option<&Round> {
        self.round()
    }

    #[inline]
    pub fn round_mut(&mut self) -> Option<&mut Round> {
        self.round.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use round_mut()")]
    pub fn get_round_mut(&mut self) -> Option<&mut Round> {
        self.round_mut()
    }

    #[inline]
    pub fn set_round(&mut self, value: Round) -> &mut Self {
        self.round = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn alignment(&self) -> &PenAlignmentValues {
        self.alignment.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use alignment()")]
    pub fn get_alignment(&self) -> &PenAlignmentValues {
        self.alignment()
    }

    #[inline]
    pub fn set_alignment(&mut self, value: PenAlignmentValues) {
        self.alignment.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn system_color(&self) -> Option<&SystemColor> {
        self.system_color.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use system_color()")]
    pub fn get_system_color(&self) -> Option<&SystemColor> {
        self.system_color()
    }

    #[inline]
    pub fn system_color_mut(&mut self) -> Option<&mut SystemColor> {
        self.system_color.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use system_color_mut()")]
    pub fn get_system_color_mut(&mut self) -> Option<&mut SystemColor> {
        self.system_color_mut()
    }

    #[inline]
    pub fn set_system_color(&mut self, value: SystemColor) {
        self.system_color = Some(Box::new(value));
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
                        let obj = NoFill::default();
                        NoFill::set_attributes(reader, e);
                        self.set_no_fill(obj);
                    }
                    b"a:bevel" => {
                        let obj = Bevel::default();
                        Bevel::set_attributes(reader, e);
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
                        let obj = Round::default();
                        Round::set_attributes(reader, e);
                        self.set_round(obj);
                    }
                    b"a:sysClr" => {
                        let mut obj = SystemColor::default();
                        obj.set_attributes(reader, e);
                        self.set_system_color(obj);
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
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let width = self.width.value_string();
        if self.width.has_value() {
            attributes.push(("w", &width).into());
        }
        if let Some(v) = self.cap_type.value() {
            attributes.push(("cap", v).into());
        }
        if let Some(v) = self.compound_line_type.value() {
            attributes.push(("cmpd", v).into());
        }
        if self.alignment.has_value() {
            attributes.push(("algn", (self.alignment.value_string())).into());
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
        if self.round.is_some() {
            Round::write_to(writer);
        }

        // a:tailEnd
        if let Some(v) = &self.tail_end {
            v.write_to(writer);
        }

        // a:noFill
        if self.no_fill.is_some() {
            NoFill::write_to(writer);
        }

        // a:bevel
        if self.bevel.is_some() {
            Bevel::write_to(writer);
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
