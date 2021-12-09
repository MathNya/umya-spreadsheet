// a:endParaRPr
use super::TextCharacterPropertiesType;
use super::TextCapsValues;
use super::super::EnumValue;
use super::super::Int32Value;
use super::SolidFill;
use super::Outline;
use super::LatinFont;
use super::EastAsianFont;
use super::GradientFill;
use super::NoFill;
use super::EffectList;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct EndParagraphRunProperties {
    text: String,
    kumimoji: Option<String>,
    language: Option<String>,
    alternative_language: Option<String>,
    bold: Option<String>,
    sz: Option<String>,
    italic: Option<String>,
    capital: EnumValue<TextCapsValues>,
    spacing: Int32Value,
    outline: Option<Outline>,
    solid_fill: Option<SolidFill>,
    latin_font: Option<LatinFont>,
    east_asian_font: Option<EastAsianFont>,
    gradient_fill: Option<GradientFill>,
    no_fill: Option<NoFill>,
    effect_list: Option<EffectList>,
}
impl TextCharacterPropertiesType for EndParagraphRunProperties {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.text = value.into();
        self
    }

    fn get_kumimoji(&self) -> &Option<String> {
        &self.kumimoji
    }

    fn set_kumimoji<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.kumimoji = Some(value.into());
        self
    }

    fn get_language(&self) -> &Option<String> {
        &self.language
    }

    fn set_language<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.language = Some(value.into());
        self
    }

    fn get_alternative_language(&self) -> &Option<String> {
        &self.alternative_language
    }

    fn set_alternative_language<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.alternative_language = Some(value.into());
        self
    }

    fn get_bold(&self) -> &Option<String> {
        &self.bold
    }

    fn set_bold<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.bold = Some(value.into());
        self
    }

    fn get_sz(&self) -> &Option<String> {
        &self.sz
    }

    fn set_sz<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.sz = Some(value.into());
        self
    }

    fn get_italic(&self) -> &Option<String> {
        &self.italic
    }

    fn set_italic<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.italic = Some(value.into());
        self
    }

    fn get_capital(&self) -> &TextCapsValues {
        &self.capital.get_value()
    }

    fn set_capital(&mut self, value:TextCapsValues) -> &mut Self {
        self.capital.set_value(value);
        self
    }

    fn get_spacing(&self) -> &i32 {
        &self.spacing.get_value()
    }

    fn set_spacing(&mut self, value:i32) -> &mut Self {
        self.spacing.set_value(value);
        self
    }

    fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }
    
    fn set_solid_fill(&mut self, value:SolidFill) -> &mut Self {
        self.solid_fill = Some(value);
        self
    }

    fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    fn set_outline(&mut self, value:Outline) -> &mut Self {
        self.outline = Some(value);
        self
    }

    fn get_latin_font(&self) -> &Option<LatinFont> {
        &self.latin_font
    }

    fn get_latin_font_mut(&mut self) -> &mut Option<LatinFont> {
        &mut self.latin_font
    }
    
    fn set_latin_font(&mut self, value:LatinFont) -> &mut Self {
        self.latin_font = Some(value);
        self
    }

    fn get_east_asian_font(&self) -> &Option<EastAsianFont> {
        &self.east_asian_font
    }

    fn get_east_asian_font_mut(&mut self) -> &mut Option<EastAsianFont> {
        &mut self.east_asian_font
    }
    
    fn set_east_asian_font(&mut self, value:EastAsianFont) -> &mut Self {
        self.east_asian_font = Some(value);
        self
    }

    fn get_gradient_fill(&self) -> &Option<GradientFill> {
        &self.gradient_fill
    }

    fn get_gradient_fill_mut(&mut self) -> &mut Option<GradientFill> {
        &mut self.gradient_fill
    }
    
    fn set_gradient_fill(&mut self, value:GradientFill) -> &mut Self {
        self.gradient_fill = Some(value);
        self
    }

    fn get_no_fill(&self) -> &Option<NoFill> {
        &self.no_fill
    }

    fn get_no_fill_mut(&mut self) -> &mut Option<NoFill> {
        &mut self.no_fill
    }
    
    fn set_no_fill(&mut self, value:NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }
    
    fn set_effect_list(&mut self, value:EffectList) -> &mut Self {
        self.effect_list = Some(value);
        self
    }
}

impl EndParagraphRunProperties {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        e:&BytesStart,
        empty_flag:bool,
    ) {
        match get_attribute(e, b"kumimoji") {
            Some(v) => {&mut self.set_kumimoji(v);},
            None => {}
        }
        match get_attribute(e, b"lang") {
            Some(v) => {&mut self.set_language(v);},
            None => {}
        }
        match get_attribute(e, b"altLang") {
            Some(v) => {&mut self.set_alternative_language(v);},
            None => {}
        }
        match get_attribute(e, b"b") {
            Some(v) => {&mut self.set_bold(v);},
            None => {}
        }
        match get_attribute(e, b"sz") {
            Some(v) => {&mut self.set_sz(v);},
            None => {}
        }
        match get_attribute(e, b"i") {
            Some(v) => {&mut self.set_italic(v);},
            None => {}
        }
        match get_attribute(e, b"cap") {
            Some(v) => {&mut self.capital.set_value_string(v);},
            None => {}
        }
        match get_attribute(e, b"spc") {
            Some(v) => {&mut self.spacing.set_value_string(v);},
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:solidFill" => {
                            let mut obj = SolidFill::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_solid_fill(obj);
                        },
                        b"a:ln" => {
                            let mut obj = Outline::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_outline(obj);
                        },
                        b"a:gradFill" => {
                            let mut obj = GradientFill::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_gradient_fill(obj);
                        },
                        b"a:effectLst" => {
                            let mut effect_list = EffectList::default();
                            effect_list.set_attributes(reader, e, false);
                            &mut self.set_effect_list(effect_list);
                        }
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:latin" => {
                            let mut obj = LatinFont::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_latin_font(obj);
                        },
                        b"a:ea" => {
                            let mut obj = EastAsianFont::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_east_asian_font(obj);
                        },
                        b"a:noFill" => {
                            let mut obj = NoFill::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_no_fill(obj);
                        }
                        b"a:effectLst" => {
                            let mut obj = EffectList::default();
                            obj.set_attributes(reader, e, true);
                            &mut self.set_effect_list(obj);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:endParaRPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:endParaRPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.kumimoji {
            Some(v) => attributes.push(("kumimoji", v)),
            None => {}
        }
        match &self.language {
            Some(v) => attributes.push(("lang", v)),
            None => {}
        }
        match &self.alternative_language {
            Some(v) => attributes.push(("altLang", v)),
            None => {}
        }
        match &self.sz {
            Some(v) => attributes.push(("sz", v)),
            None => {}
        }
        match &self.bold {
            Some(v) => attributes.push(("b", v)),
            None => {}
        }
        match &self.italic {
            Some(v) => attributes.push(("i", v)),
            None => {}
        }
        if self.capital.has_value() == true {
            attributes.push(("cap", self.capital.get_value_string()));
        }
        if self.spacing.has_value() == true {
            attributes.push(("spc", self.spacing.get_value_string()));
        }
        if self.solid_fill.is_some() || self.outline.is_some() || self.latin_font.is_some() || self.east_asian_font.is_some() || self.gradient_fill.is_some() || self.effect_list.is_some() {
            write_start_tag(writer, "a:endParaRPr", attributes, false);

            // a:solidFill
            match &self.solid_fill {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            // a:ln
            match &self.outline {
                Some(v) => {v.write_to(writer);},
                None => {}
            }
    
            // a:latin
            match &self.latin_font {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            // a:ea
            match &self.east_asian_font {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            // a:gradFill
            match &self.gradient_fill {
                Some(v) => {v.write_to(writer);},
                None => {}
            }
            
            // a:noFill
            match &self.no_fill {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            // a:effectLst
            match &self.effect_list {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            write_end_tag(writer, "a:endParaRPr");
        } else {
            write_start_tag(writer, "a:endParaRPr", attributes, true);
        }
    }
}
