use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use reader::xlsx::vml_drawing_rels;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ImageData {
    image_name: StringValue,
    title: StringValue,
}
impl ImageData {
    pub fn get_image_name(&self) -> &str {
        &self.image_name.get_value()
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_name.set_value(value);
        self
    }

    pub fn get_title(&self) -> &str {
        &self.title.get_value()
    }

    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        target: &str,
    ) {
        match get_attribute(e, b"o:relid") {
            Some(relid) => {
                let (_type_value, target_value) =
                    vml_drawing_rels::read(arv, target, &relid).unwrap();
                let v: Vec<&str> = target_value.split('/').collect();
                let image_name = v.last().unwrap().clone();
                self.image_name.set_value_string(image_name);
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
            vec![("o:relid", &r_id_str), ("o:title", &self.title.get_value())],
            true,
        );
    }
}
