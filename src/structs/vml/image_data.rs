use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ImageData {
    image_name: StringValue,
    title: StringValue,
}
impl ImageData {
    pub fn get_image_name(&self) -> &str {
        self.image_name.get_value()
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_name.set_value(value);
        self
    }

    pub fn get_title(&self) -> &str {
        self.title.get_value()
    }

    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        match get_attribute(e, b"o:relid") {
            Some(relid) => {
                let relationship = drawing_relationships.unwrap().get_relationship_by_rid(&relid);
                self.image_name
                    .set_value_string(relationship.get_raw_file().get_file_name());
            }
            None => {}
        }

        match get_attribute(e, b"o:title") {
            Some(v) => {
                self.title.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &usize) {
        // v:imagedata
        let r_id_str = format!("rId{}", r_id);
        write_start_tag(
            writer,
            "v:imagedata",
            vec![("o:relid", &r_id_str), ("o:title", self.title.get_value())],
            true,
        );
    }
}
