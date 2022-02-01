use super::BooleanValue;
use super::ObjectAnchor;
use super::StringValue;
use super::UInt32Value;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use reader::xlsx::media;
use reader::xlsx::worksheet_rels;
use std::io::Cursor;
use structs::Image;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct EmbeddedObjectProperties {
    prog_id: StringValue,
    shape_id: UInt32Value,
    image: Image,
    default_size: BooleanValue,
    auto_pict: BooleanValue,
    object_anchor: ObjectAnchor,
}
impl EmbeddedObjectProperties {
    pub fn get_prog_id(&self) -> &str {
        &self.prog_id.get_value()
    }

    pub fn set_prog_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prog_id.set_value(value);
        self
    }

    pub fn get_shape_id(&self) -> &u32 {
        &self.shape_id.get_value()
    }

    pub fn set_shape_id(&mut self, value: u32) -> &mut Self {
        self.shape_id.set_value(value);
        self
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }

    pub fn get_image_mut(&mut self) -> &mut Image {
        &mut self.image
    }

    pub fn set_image(&mut self, value: Image) {
        self.image = value;
    }

    pub fn get_default_size(&self) -> &bool {
        &self.default_size.get_value()
    }

    pub fn set_default_size(&mut self, value: bool) -> &mut Self {
        self.default_size.set_value(value);
        self
    }

    pub fn get_auto_pict(&self) -> &bool {
        &self.auto_pict.get_value()
    }

    pub fn set_auto_pict(&mut self, value: bool) -> &mut Self {
        self.auto_pict.set_value(value);
        self
    }

    pub fn get_object_anchor(&self) -> &ObjectAnchor {
        &self.object_anchor
    }

    pub fn get_object_anchor_mut(&mut self) -> &mut ObjectAnchor {
        &mut self.object_anchor
    }

    pub fn set_object_anchor(&mut self, value: ObjectAnchor) -> &mut Self {
        self.object_anchor = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        sheet_name: &str,
    ) {
        let r_id = get_attribute(e, b"r:id").unwrap();
        let (_, target_value) = worksheet_rels::read_rid(arv, sheet_name, &r_id).unwrap();

        let v: Vec<&str> = target_value.split('/').collect();
        let object_name = v.last().unwrap().clone();
        &mut self.get_image_mut().set_image_name(object_name);
        &mut self
            .get_image_mut()
            .set_image_data(media::read(arv, &target_value).unwrap());

        match get_attribute(e, b"defaultSize") {
            Some(v) => {
                self.default_size.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"autoPict") {
            Some(v) => {
                self.auto_pict.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"anchor" => {
                        &mut self.object_anchor.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"objectPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "objectPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &usize) {
        // objectPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.default_size.has_value() {
            attributes.push(("defaultSize", self.default_size.get_value_string()));
        }
        if self.auto_pict.has_value() {
            attributes.push(("autoPict", self.auto_pict.get_value_string()));
        }
        let r_id_str = format!("rId{}", r_id);
        attributes.push(("r:id", r_id_str.as_str()));
        write_start_tag(writer, "objectPr", attributes, false);

        // anchor
        &mut self.object_anchor.write_to(writer);

        write_end_tag(writer, "objectPr");
    }
}
