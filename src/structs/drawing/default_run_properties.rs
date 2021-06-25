// a:defRPr
use super::SolidFill;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct DefaultRunProperties {
    text: String,
    kumimoji: Option<String>,
    lang: Option<String>,
    alt_lang: Option<String>,
    sz: Option<String>,
    solid_fill: Option<SolidFill>,
}
impl DefaultRunProperties {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value:S) -> &mut DefaultRunProperties {
        self.text = value.into();
        self
    }

    pub fn get_kumimoji(&self) -> &Option<String> {
        &self.kumimoji
    }

    pub fn set_kumimoji<S: Into<String>>(&mut self, value:S) -> &mut DefaultRunProperties {
        self.kumimoji = Some(value.into());
        self
    }

    pub fn get_lang(&self) -> &Option<String> {
        &self.lang
    }

    pub fn set_lang<S: Into<String>>(&mut self, value:S) -> &mut DefaultRunProperties {
        self.lang = Some(value.into());
        self
    }

    pub fn get_alt_lang(&self) -> &Option<String> {
        &self.alt_lang
    }

    pub fn set_alt_lang<S: Into<String>>(&mut self, value:S) -> &mut DefaultRunProperties {
        self.alt_lang = Some(value.into());
        self
    }

    pub fn get_sz(&self) -> &Option<String> {
        &self.sz
    }

    pub fn set_sz<S: Into<String>>(&mut self, value:S) -> &mut DefaultRunProperties {
        self.sz = Some(value.into());
        self
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }
    
    pub fn set_solid_fill(&mut self, value:SolidFill) -> &mut DefaultRunProperties {
        self.solid_fill = Some(value);
        self
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
        match get_attribute(e, b"sz") {
            Some(v) => {&mut self.set_sz(v);},
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
                            let mut solid_fill = SolidFill::default();
                            solid_fill.set_attributes(reader, e);
                            &mut self.set_solid_fill(solid_fill);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:defRPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:defRPr"),
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
        match &self.sz {
            Some(v) => attributes.push(("sz", v)),
            None => {}
        }
        match &self.solid_fill {
            Some(v) => {
                write_start_tag(writer, "a:defRPr", attributes, false);
                v.write_to(writer);
                write_end_tag(writer, "a:defRPr");
            },
            None => {
                write_start_tag(writer, "a:defRPr", attributes, true);
            }
        }
    }
}