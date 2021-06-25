// a:pPr
use super::DefaultRunProperties;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ParagraphProperties {
    right_to_left: Option<String>,
    default_run_properties: Option<DefaultRunProperties>,
}
impl ParagraphProperties {
    pub fn get_right_to_left(&self) -> &Option<String> {
        &self.right_to_left
    }

    pub fn set_right_to_left<S: Into<String>>(&mut self, value:S) -> &mut ParagraphProperties {
        self.right_to_left = Some(value.into());
        self
    }

    pub fn get_default_run_properties(&self) -> &Option<DefaultRunProperties> {
        &self.default_run_properties
    }

    pub fn get_default_run_properties_mut(&mut self) -> &mut Option<DefaultRunProperties> {
        &mut self.default_run_properties
    }

    pub fn set_default_run_properties(&mut self, value:DefaultRunProperties) -> &mut ParagraphProperties {
        self.default_run_properties = Some(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        empty_flag:bool,
    ) {
        match get_attribute(e, b"rtl") {
            Some(v) => {&mut self.set_right_to_left(v);},
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
                        b"a:defRPr" => {
                            let mut obj = DefaultRunProperties::default();
                            obj.set_attributes(reader, e, false);
                            &mut self.set_default_run_properties(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:defRPr" => {
                            let mut obj = DefaultRunProperties::default();
                            obj.set_attributes(reader, e, true);
                            &mut self.set_default_run_properties(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:pPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:pPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:pPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.right_to_left {
            Some(v) => {attributes.push(("rtl", v));},
            None => {}
        }
        write_start_tag(writer, "a:pPr", attributes, false);

        // a:defRPr
        match &self.default_run_properties {
            Some(v) => v.write_to(writer),
            None => {}
        }

        write_end_tag(writer, "a:pPr");
    }
}