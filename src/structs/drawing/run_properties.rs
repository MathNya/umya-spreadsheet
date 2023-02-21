use super::super::EnumValue;
use super::super::Int32Value;
use super::EffectList;
use super::GradientFill;
use super::NoFill;
use super::Outline;
use super::SolidFill;
use super::TextCapsValues;
use super::TextFontType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct RunProperties {
    text: String,
    kumimoji: StringValue,
    language: StringValue,
    alternative_language: StringValue,
    bold: StringValue,
    sz: StringValue,
    italic: StringValue,
    capital: EnumValue<TextCapsValues>,
    spacing: Int32Value,
    strike: StringValue,
    outline: Option<Outline>,
    solid_fill: Option<SolidFill>,
    latin_font: Option<TextFontType>,
    east_asian_font: Option<TextFontType>,
    gradient_fill: Option<GradientFill>,
    no_fill: Option<NoFill>,
    effect_list: Option<EffectList>,
}
impl RunProperties {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text = value.into();
        self
    }

    pub fn get_kumimoji(&self) -> &str {
        self.kumimoji.get_value_string()
    }

    pub fn set_kumimoji<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.kumimoji.set_value_string(value.into());
        self
    }

    pub fn get_language(&self) -> &str {
        self.language.get_value_string()
    }

    pub fn set_language<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.language.set_value_string(value.into());
        self
    }

    pub fn get_alternative_language(&self) -> &str {
        self.alternative_language.get_value_string()
    }

    pub fn set_alternative_language<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.alternative_language.set_value_string(value.into());
        self
    }

    pub fn get_bold(&self) -> &str {
        self.bold.get_value_string()
    }

    pub fn set_bold<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.bold.set_value_string(value.into());
        self
    }

    pub fn get_sz(&self) -> &str {
        self.sz.get_value_string()
    }

    pub fn set_sz<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.sz.set_value_string(value.into());
        self
    }

    pub fn get_italic(&self) -> &str {
        self.italic.get_value_string()
    }

    pub fn set_italic<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.italic.set_value_string(value.into());
        self
    }

    pub fn get_capital(&self) -> &TextCapsValues {
        self.capital.get_value()
    }

    pub fn set_capital(&mut self, value: TextCapsValues) -> &mut Self {
        self.capital.set_value(value);
        self
    }

    pub fn get_spacing(&self) -> &i32 {
        self.spacing.get_value()
    }

    pub fn set_spacing(&mut self, value: i32) -> &mut Self {
        self.spacing.set_value(value);
        self
    }

    pub fn get_strike(&self) -> &str {
        self.strike.get_value_string()
    }

    pub fn set_strike<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.strike.set_value_string(value.into());
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

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value: Outline) -> &mut Self {
        self.outline = Some(value);
        self
    }

    pub fn get_latin_font(&self) -> &Option<TextFontType> {
        &self.latin_font
    }

    pub fn get_latin_font_mut(&mut self) -> &mut Option<TextFontType> {
        &mut self.latin_font
    }

    pub fn set_latin_font(&mut self, value: TextFontType) -> &mut Self {
        self.latin_font = Some(value);
        self
    }

    pub fn get_east_asian_font(&self) -> &Option<TextFontType> {
        &self.east_asian_font
    }

    pub fn get_east_asian_font_mut(&mut self) -> &mut Option<TextFontType> {
        &mut self.east_asian_font
    }

    pub fn set_east_asian_font(&mut self, value: TextFontType) -> &mut Self {
        self.east_asian_font = Some(value);
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

    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value: EffectList) -> &mut Self {
        self.effect_list = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"kumimoji") {
            Some(v) => {
                self.set_kumimoji(v);
            }
            None => {}
        }
        match get_attribute(e, b"lang") {
            Some(v) => {
                self.set_language(v);
            }
            None => {}
        }
        match get_attribute(e, b"altLang") {
            Some(v) => {
                self.set_alternative_language(v);
            }
            None => {}
        }
        match get_attribute(e, b"b") {
            Some(v) => {
                self.set_bold(v);
            }
            None => {}
        }
        match get_attribute(e, b"sz") {
            Some(v) => {
                self.set_sz(v);
            }
            None => {}
        }
        match get_attribute(e, b"strike") {
            Some(v) => {
                self.set_strike(v);
            }
            None => {}
        }
        match get_attribute(e, b"i") {
            Some(v) => {
                self.set_italic(v);
            }
            None => {}
        }
        match get_attribute(e, b"cap") {
            Some(v) => {
                self.capital.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"spc") {
            Some(v) => {
                self.spacing.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:solidFill" => {
                        let mut obj = SolidFill::default();
                        obj.set_attributes(reader, e);
                        self.set_solid_fill(obj);
                    }
                    b"a:ln" => {
                        let mut obj = Outline::default();
                        obj.set_attributes(reader, e);
                        self.set_outline(obj);
                    }
                    b"a:gradFill" => {
                        let mut obj = GradientFill::default();
                        obj.set_attributes(reader, e);
                        self.set_gradient_fill(obj);
                    }
                    b"a:effectLst" => {
                        let mut effect_list = EffectList::default();
                        effect_list.set_attributes(reader, e, false);
                        self.set_effect_list(effect_list);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:latin" => {
                        let mut obj = TextFontType::default();
                        obj.set_attributes(reader, e);
                        self.set_latin_font(obj);
                    }
                    b"a:ea" => {
                        let mut obj = TextFontType::default();
                        obj.set_attributes(reader, e);
                        self.set_east_asian_font(obj);
                    }
                    b"a:noFill" => {
                        let mut obj = NoFill::default();
                        obj.set_attributes(reader, e);
                        self.set_no_fill(obj);
                    }
                    b"a:effectLst" => {
                        let mut obj = EffectList::default();
                        obj.set_attributes(reader, e, true);
                        self.set_effect_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:rPr" => return,
                    b"a:endParaRPr" => return,
                    b"a:defRPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!(
                    "Error not find {} end element",
                    "a:rPr, a:endParaRPr, a:defRPr"
                ),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to_rpr(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:rPr")
    }

    pub(crate) fn write_to_end_para_rpr(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:endParaRPr")
    }

    pub(crate) fn write_to_def_rpr(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:defRPr")
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.kumimoji.has_value() {
            attributes.push(("kumimoji", self.kumimoji.get_value_string()))
        }
        if self.language.has_value() {
            attributes.push(("lang", self.language.get_value_string()))
        }
        if self.alternative_language.has_value() {
            attributes.push(("altLang", self.alternative_language.get_value_string()))
        }
        if self.sz.has_value() {
            attributes.push(("sz", self.sz.get_value_string()))
        }
        if self.bold.has_value() {
            attributes.push(("b", self.bold.get_value_string()))
        }
        if self.italic.has_value() {
            attributes.push(("i", self.italic.get_value_string()))
        }
        if self.capital.has_value() {
            attributes.push(("cap", self.capital.get_value_string()));
        }
        let spc = self.spacing.get_value_string();
        if self.spacing.has_value() {
            attributes.push(("spc", &spc));
        }
        if self.strike.has_value() {
            attributes.push(("strike", self.strike.get_value_string()));
        }
        if self.solid_fill.is_some()
            || self.outline.is_some()
            || self.latin_font.is_some()
            || self.east_asian_font.is_some()
            || self.gradient_fill.is_some()
            || self.effect_list.is_some()
        {
            write_start_tag(writer, tag_name, attributes, false);

            // a:solidFill
            match &self.solid_fill {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:ln
            match &self.outline {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:latin
            match &self.latin_font {
                Some(v) => {
                    v.write_to_latin(writer);
                }
                None => {}
            }

            // a:ea
            match &self.east_asian_font {
                Some(v) => {
                    v.write_to_ea(writer);
                }
                None => {}
            }

            // a:gradFill
            match &self.gradient_fill {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:noFill
            match &self.no_fill {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:effectLst
            match &self.effect_list {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, tag_name);
        } else {
            write_start_tag(writer, tag_name, attributes, true);
        }
    }
}
