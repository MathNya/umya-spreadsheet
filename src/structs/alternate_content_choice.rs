// mc:Choice
use super::StringValue;
use super::office2010::drawing::charts::Style;
use super::OleObject;
//use super::drawing::spreadsheet::TwoCellAnchor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;
use reader::driver::*;

#[derive(Default, Debug)]
pub struct AlternateContentChoice {
    sub_path: Vec<(String,String)>,
    requires: StringValue,
    style: Option<Style>,
    ole_object: Option<OleObject>,
    //two_cell_anchor: Option<TwoCellAnchor>,
}
impl AlternateContentChoice {
    pub fn get_sub_path(&self) -> &Vec<(String,String)> {
        &self.sub_path
    }

    pub fn set_sub_path(&mut self, value: (String,String)) -> &mut Self {
        self.sub_path.push(value);
        self
    }

    pub fn get_requires(&self) -> &str {
        &self.requires.get_value()
    }

    pub fn set_requires<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.requires.set_value(value);
        self
    }

    pub fn get_style(&self) -> &Option<Style> {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut Option<Style> {
        &mut self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = Some(value);
        self
    }

    pub fn get_ole_object(&self) -> &Option<OleObject> {
        &self.ole_object
    }

    pub fn get_ole_object_mut(&mut self) -> &mut Option<OleObject> {
        &mut self.ole_object
    }

    pub fn set_ole_object(&mut self, value: OleObject) -> &mut Self {
        self.ole_object = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        sheet_name: Option<&str>,
    ) {
        match get_attribute(e, b"Requires") {
            Some(v) => {
                self.requires.set_value_string(v);
            },
            None => {}
        }
        match get_attribute(e, b"xmlns:c14") {
            Some(v) => {
                self.set_sub_path(("xmlns:c14".to_string(), v));
            },
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"c14:style" => {
                        let mut obj = Style::default();
                        obj.set_attributes(reader, e);
                        self.set_style(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name() {
                    b"oleObject" => {
                        let mut obj = OleObject::default();
                        obj.set_attributes(reader, e, arv, sheet_name.unwrap(), false);
                        self.set_ole_object(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"mc:Choice" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:Choice"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: Option<&usize>,
    ) {
        // mc:Choice
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.requires.has_value() {
            attributes.push(("Requires", self.requires.get_value_string()));
        }
        for (title, path) in &self.sub_path {
            attributes.push((title, path));
        }
        write_start_tag(writer, "mc:Choice", attributes, false);

        // c14:style
        match &self.style {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // oleObject
        match &self.ole_object {
            Some(v) => {v.write_to(writer, r_id.unwrap());},
            None => {}
        }

        write_end_tag(writer, "mc:Choice");
    }
}
