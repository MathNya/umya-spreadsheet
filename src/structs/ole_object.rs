use super::StringValue;
use super::UInt32Value;
use super::EmbeddedObjectProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;
use reader::driver::*;
use reader::xlsx::worksheet_rels;
use reader::xlsx::embeddings;

#[derive(Default, Debug)]
pub struct OleObject {
    prog_id: StringValue,
    shape_id: UInt32Value,
    object_name: String,
    object_data: Option<Vec<u8>>,
    embedded_object_properties: Option<EmbeddedObjectProperties>,
}
impl OleObject {
    pub fn get_prog_id(&self) -> &str {
        &self.prog_id.get_value()
    }

    pub fn set_prog_id<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.prog_id.set_value(value);
        self
    }

    pub fn get_shape_id(&self) -> &u32 {
        &self.shape_id.get_value()
    }

    pub fn set_shape_id(&mut self, value:u32) -> &mut Self {
        self.shape_id.set_value(value);
        self
    }

    pub fn get_object_name(&self) -> &str {
        &self.object_name
    }

    pub fn set_object_name<S: Into<String>>(&mut self, value: S) {
        self.object_name = value.into();
    }

    pub fn get_object_data(&self) -> &Option<Vec<u8>> {
        &self.object_data
    }

    pub fn get_object_data_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.object_data
    }

    pub fn set_object_data(&mut self, value: Vec<u8>) -> &mut Self {
        self.object_data = Some(value);
        self
    }

    pub fn get_embedded_object_properties(&self) -> &Option<EmbeddedObjectProperties> {
        &self.embedded_object_properties
    }

    pub fn get_embedded_object_properties_mut(&mut self) -> &mut Option<EmbeddedObjectProperties> {
        &mut self.embedded_object_properties
    }

    pub fn set_embedded_object_properties(&mut self, value: EmbeddedObjectProperties) -> &mut Self {
        self.embedded_object_properties = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        sheet_name: &str,
        empty_flag:bool,
    ) {
        &mut self.prog_id.set_value_string(get_attribute(e, b"progId").unwrap());
        &mut self.shape_id.set_value_string(get_attribute(e, b"shapeId").unwrap());

        let r_id = get_attribute(e, b"r:id").unwrap();
        let (_, target_value) = worksheet_rels::read_rid(arv, sheet_name, &r_id).unwrap();

        let v: Vec<&str> = target_value.split('/').collect();
        let object_name = v.last().unwrap().clone();
        &mut self.set_object_name(object_name);
        &mut self.set_object_data(embeddings::read(arv, object_name).unwrap());

        if empty_flag {
            return;
        }
        
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"objectPr" => {
                        let mut obj = EmbeddedObjectProperties::default();
                        obj.set_attributes(reader, e, arv, sheet_name);
                        &mut self.set_embedded_object_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"oleObject" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "oleObject"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: &usize,
    ) {
        // oleObject
        let r_id_str = format!("rId{}", r_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("progId", &self.prog_id.get_value_string()));
        attributes.push(("shapeId", &self.shape_id.get_value_string()));
        attributes.push(("r:id", r_id_str.as_str()));
        write_start_tag(writer, "oleObject", attributes, false);

        // objectPr
        match &self.embedded_object_properties {
            Some(v) => {
                v.write_to(writer, &(r_id+1));
            }
            None => {}
        }

        write_end_tag(writer, "oleObject");
    }
}
