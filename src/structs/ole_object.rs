use super::EmbeddedObjectProperties;
use super::StringValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::drawing::spreadsheet::TwoCellAnchor;
use structs::raw::RawRelationships;
use structs::vml::Shape;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OleObject {
    requires: StringValue,
    prog_id: StringValue,
    object_extension: String,
    object_data: Option<Vec<u8>>,
    embedded_object_properties: EmbeddedObjectProperties,
    two_cell_anchor: TwoCellAnchor,
    shape: Shape,
}
impl OleObject {
    pub fn get_requires(&self) -> &str {
        self.requires.get_value()
    }

    pub fn set_requires<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.requires.set_value(value);
        self
    }

    pub fn get_prog_id(&self) -> &str {
        self.prog_id.get_value()
    }

    pub fn set_prog_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prog_id.set_value(value);
        self
    }

    pub fn get_object_extension(&self) -> &str {
        &self.object_extension
    }

    pub fn set_object_extension<S: Into<String>>(&mut self, value: S) {
        self.object_extension = value.into();
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

    pub fn get_embedded_object_properties(&self) -> &EmbeddedObjectProperties {
        &self.embedded_object_properties
    }

    pub fn get_embedded_object_properties_mut(&mut self) -> &mut EmbeddedObjectProperties {
        &mut self.embedded_object_properties
    }

    pub fn set_embedded_object_properties(&mut self, value: EmbeddedObjectProperties) -> &mut Self {
        self.embedded_object_properties = value;
        self
    }

    pub fn get_two_cell_anchor(&self) -> &TwoCellAnchor {
        &self.two_cell_anchor
    }

    pub fn get_two_cell_anchor_mut(&mut self) -> &mut TwoCellAnchor {
        &mut self.two_cell_anchor
    }

    pub fn set_two_cell_anchor(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor = value;
        self
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = value;
        self
    }

    pub(crate) fn is_bin(&self) -> bool {
        &self.object_extension == "bin"
    }

    pub(crate) fn is_xlsx(&self) -> bool {
        &self.object_extension == "xlsx"
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        relationships: &RawRelationships,
    ) {
        let mut alternate_content = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"mc:Choice" => {
                        alternate_content = String::from("Choice");
                        match get_attribute(e, b"Requires") {
                            Some(v) => {
                                self.requires.set_value_string(v);
                            }
                            None => {}
                        }
                    }
                    b"mc:Fallback" => {
                        alternate_content = String::from("Fallback");
                    }
                    b"oleObject" => {
                        if alternate_content.as_str() == "Choice" {
                            self.prog_id
                                .set_value_string(get_attribute(e, b"progId").unwrap());

                            let r_id = get_attribute(e, b"r:id").unwrap();
                            let attached_file =
                                relationships.get_relationship_by_rid(&r_id).get_raw_file();
                            self.set_object_extension(attached_file.get_extension());
                            self.set_object_data(attached_file.get_file_data().clone());
                        }
                    }
                    b"objectPr" => {
                        let mut obj = EmbeddedObjectProperties::default();
                        obj.set_attributes(reader, e, relationships);
                        self.set_embedded_object_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"mc:AlternateContent" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:AlternateContent"),
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
        ole_id: &usize,
    ) {
        // mc:AlternateContent
        write_start_tag(
            writer,
            "mc:AlternateContent",
            vec![(
                "xmlns:mc",
                "http://schemas.openxmlformats.org/markup-compatibility/2006",
            )],
            false,
        );

        // mc:Choice
        write_start_tag(
            writer,
            "mc:Choice",
            vec![("Requires", self.requires.get_value_string())],
            false,
        );

        // oleObject
        let r_id_str = format!("rId{}", r_id);
        let shape_id_str = format!("{}", ole_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("progId", self.prog_id.get_value_string()));
        attributes.push(("shapeId", shape_id_str.as_str()));
        attributes.push(("r:id", r_id_str.as_str()));
        write_start_tag(writer, "oleObject", attributes, false);

        // objectPr
        let _ = &self
            .embedded_object_properties
            .write_to(writer, &(r_id + 1));

        write_end_tag(writer, "oleObject");

        write_end_tag(writer, "mc:Choice");

        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // oleObject
        let r_id_str = format!("rId{}", r_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("progId", self.prog_id.get_value_string()));
        attributes.push(("shapeId", shape_id_str.as_str()));
        attributes.push(("r:id", r_id_str.as_str()));
        write_start_tag(writer, "oleObject", attributes, true);

        write_end_tag(writer, "mc:Fallback");

        write_end_tag(writer, "mc:AlternateContent");
    }
}
