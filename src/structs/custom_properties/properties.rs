use std::io::Cursor;

use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};

use crate::{
    helper::const_str::{CUSTOM_PROPS_NS, VTYPES_NS},
    reader::driver::xml_read_loop,
    structs::custom_properties::CustomDocumentProperty,
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Default, Debug, Clone)]
pub struct Properties {
    custom_document_property_list: Vec<CustomDocumentProperty>,
}

impl Properties {
    #[inline]
    #[must_use]
    pub fn get_custom_document_property_list(&self) -> &[CustomDocumentProperty] {
        &self.custom_document_property_list
    }

    #[inline]
    pub fn get_custom_document_property_list_mut(&mut self) -> &mut Vec<CustomDocumentProperty> {
        &mut self.custom_document_property_list
    }

    #[inline]
    pub fn set_custom_document_property_list(
        &mut self,
        value: impl Into<Vec<CustomDocumentProperty>>,
    ) -> &mut Self {
        self.custom_document_property_list = value.into();
        self
    }

    #[inline]
    pub fn add_custom_document_property_list(
        &mut self,
        value: CustomDocumentProperty,
    ) -> &mut Self {
        self.custom_document_property_list.push(value);
        self
    }

    #[inline]
    pub fn remove_custom_document_property_list(
        &mut self,
        _value: CustomDocumentProperty,
    ) -> &mut Self {
        self.custom_document_property_list.clear();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
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
            vec![
                ("xmlns", CUSTOM_PROPS_NS).into(),
                ("xmlns:vt", VTYPES_NS).into(),
            ],
            false,
        );
        let mut pid = 2;
        for v in &self.custom_document_property_list {
            v.write_to(writer, pid);
            pid += 1;
        }
        write_end_tag(writer, "Properties");
    }
}
