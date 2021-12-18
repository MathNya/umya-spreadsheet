// mc:Fallback
use super::drawing::charts::Style;
use super::OleObject;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct AlternateContentFallback {
    style: Option<Style>,
    ole_object: Option<OleObject>,
}
impl AlternateContentFallback {
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
        _e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        sheet_name: Option<&str>,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:style" => {
                            let mut obj = Style::default();
                            obj.set_attributes(reader, e);
                            self.set_style(obj);
                        },
                        b"oleObject" => {
                            let mut obj = OleObject::default();
                            obj.set_attributes(reader, e, arv, sheet_name.unwrap(), true);
                            self.set_ole_object(obj);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"mc:Fallback" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:Fallback"),
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
        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

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

        write_end_tag(writer, "mc:Fallback");
    }
}
