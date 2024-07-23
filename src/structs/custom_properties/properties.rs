use helper::const_str::*;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::borrow::Cow;
use std::io::Cursor;
use structs::custom_properties::CustomDocumentProperty;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct Properties {
    custom_document_property_list: Vec<CustomDocumentProperty>,
}

impl Properties {
    pub fn get_custom_document_property_list(&self) -> &Vec<CustomDocumentProperty> {
        &self.custom_document_property_list
    }

    pub fn get_custom_document_property_list_mut(&mut self) -> &mut Vec<CustomDocumentProperty> {
        &mut self.custom_document_property_list
    }

    pub fn set_custom_document_property_list(
        &mut self,
        value: Vec<CustomDocumentProperty>,
    ) -> &mut Self {
        self.custom_document_property_list = value;
        self
    }

    pub fn add_custom_document_property_list(
        &mut self,
        value: CustomDocumentProperty,
    ) -> &mut Self {
        self.custom_document_property_list.push(value);
        self
    }

    pub fn remove_custom_document_property_list(
        &mut self,
        value: CustomDocumentProperty,
    ) -> &mut Self {
        self.custom_document_property_list.clear();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::from("");
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"property" {
                    let mut obj = CustomDocumentProperty::default();
                    obj.set_attributes(reader, e, true);
                    self.add_custom_document_property_list(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"property" {
                    let mut obj = CustomDocumentProperty::default();
                    obj.set_attributes(reader, e, false);
                    self.add_custom_document_property_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"Properties" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "Properties")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "Properties",
            vec![("xmlns", CUSTOM_PROPS_NS), ("xmlns:vt", VTYPES_NS)],
            false,
        );
        let mut pid = 2;
        for v in &self.custom_document_property_list {
            v.write_to(writer, &pid);
            pid += 1;
        }
        write_end_tag(writer, "Properties");
    }
}
