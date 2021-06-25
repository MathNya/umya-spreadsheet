use super::SolidFill;
use super::LatinFont;
use super::EastAsianFont;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct RunProperties {
    text: String,
    kumimoji: Option<String>,
    lang: Option<String>,
    alt_lang: Option<String>,
    bold: Option<String>,
    sz: Option<String>,
    italic: Option<String>,
    solid_fill: Option<SolidFill>,
    latin_font: Option<LatinFont>,
    east_asian_font: Option<EastAsianFont>,
}
impl RunProperties {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value:S) {
        self.text = value.into();
    }

    pub fn get_kumimoji(&self) -> &Option<String> {
        &self.kumimoji
    }

    pub fn set_kumimoji<S: Into<String>>(&mut self, value:S) {
        self.kumimoji = Some(value.into());
    }

    pub fn get_lang(&self) -> &Option<String> {
        &self.lang
    }

    pub fn set_lang<S: Into<String>>(&mut self, value:S) {
        self.lang = Some(value.into());
    }

    pub fn get_alt_lang(&self) -> &Option<String> {
        &self.alt_lang
    }

    pub fn set_alt_lang<S: Into<String>>(&mut self, value:S) {
        self.alt_lang = Some(value.into());
    }

    pub fn get_bold(&self) -> &Option<String> {
        &self.bold
    }

    pub fn set_bold<S: Into<String>>(&mut self, value:S) {
        self.bold = Some(value.into());
    }

    pub fn get_sz(&self) -> &Option<String> {
        &self.sz
    }

    pub fn set_sz<S: Into<String>>(&mut self, value:S) {
        self.sz = Some(value.into());
    }

    pub fn get_italic(&self) -> &Option<String> {
        &self.italic
    }

    pub fn set_italic<S: Into<String>>(&mut self, value:S) {
        self.italic = Some(value.into());
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }
    
    pub fn set_solid_fill(&mut self, value:SolidFill) {
        self.solid_fill = Some(value);
    }

    pub fn get_latin_font(&self) -> &Option<LatinFont> {
        &self.latin_font
    }

    pub fn get_latin_font_mut(&mut self) -> &mut Option<LatinFont> {
        &mut self.latin_font
    }
    
    pub fn set_latin_font(&mut self, value:LatinFont) {
        self.latin_font = Some(value);
    }

    pub fn get_east_asian_font(&self) -> &Option<EastAsianFont> {
        &self.east_asian_font
    }

    pub fn get_east_asian_font_mut(&mut self) -> &mut Option<EastAsianFont> {
        &mut self.east_asian_font
    }
    
    pub fn set_east_asian_font(&mut self, value:EastAsianFont) {
        self.east_asian_font = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        empty_flag:bool,
    ) {
        match get_attribute(e, b"kumimoji") {
            Some(v) => {&mut self.set_kumimoji(v);},
            None => {}
        }
        match get_attribute(e, b"lang") {
            Some(v) => {&mut self.set_lang(v);},
            None => {}
        }
        match get_attribute(e, b"altLang") {
            Some(v) => {&mut self.set_alt_lang(v);},
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
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:rPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:rPr"),
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
        match &self.lang {
            Some(v) => attributes.push(("lang", v)),
            None => {}
        }
        match &self.alt_lang {
            Some(v) => attributes.push(("altLang", v)),
            None => {}
        }
        match &self.bold {
            Some(v) => attributes.push(("b", v)),
            None => {}
        }
        match &self.sz {
            Some(v) => attributes.push(("sz", v)),
            None => {}
        }
        match &self.italic {
            Some(v) => attributes.push(("i", v)),
            None => {}
        }
        if self.solid_fill.is_some() || self.latin_font.is_some() || self.solid_fill.is_some() {
            write_start_tag(writer, "a:rPr", attributes, false);

            // a:solidFill
            match &self.solid_fill {
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

            write_end_tag(writer, "a:rPr");
        } else {
            write_start_tag(writer, "a:rPr", attributes, true);
        }
    }
}