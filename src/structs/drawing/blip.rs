// a:blip
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;
use reader::xlsx::drawing_rels;
use reader::xlsx::media;

#[derive(Default, Debug)]
pub struct Blip {
    image_name: String,
    image_data: Option<Vec<u8>>,
    cstate: String,
}
impl Blip {
    pub fn get_image_name(&self) -> &String {
        &self.image_name
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value:S) {
        self.image_name = value.into();
    }

    pub fn get_image_data(&self) -> &Option<Vec<u8>> {
        &self.image_data
    }

    pub fn set_image_data(&mut self, value:Vec<u8>) {
        self.image_data = Some(value);
    }

    pub fn get_cstate(&self) -> &String {
        &self.cstate
    }

    pub fn set_cstate<S: Into<String>>(&mut self, value:S) {
        self.cstate = value.into();
    }

    pub(crate) fn get_extension(&self)->String {
        let v: Vec<&str> = self.image_name.split('.').collect();
        let extension = v.last().unwrap().clone();
        let extension_lower = extension.to_lowercase();
        extension_lower
    }

    pub(crate) fn is_jpeg(&self)-> bool
    {
        self.get_extension() == "jpeg"
    }

    pub(crate) fn is_jpg(&self)-> bool
    {
        self.get_extension() == "jpg"
    }

    pub(crate) fn is_png(&self)-> bool
    {
        self.get_extension() == "png"
    }

    pub(crate) fn is_tiff(&self)-> bool
    {
        self.get_extension() == "tiff"
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        dir: &TempDir,
        target: &str,
    ) {
        match get_attribute(e, b"cstate") {
            Some(v) => {&mut self.set_cstate(v);},
            None => {}
        }
        
        let picture_id = get_attribute(e, b"r:embed").unwrap();
        let drawing_rel = drawing_rels::read(dir, target).unwrap();
        for (drawing_id, _, drawing_target) in &drawing_rel {
            if &picture_id == drawing_id {
                let v: Vec<&str> = drawing_target.split('/').collect();
                let image_name = v.last().unwrap().clone();
                &mut self.set_image_name(image_name);
                &mut self.set_image_data(media::read(&dir, &drawing_target).unwrap());
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // a:blip
        let r_id_str = format!("rId{}", r_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("xmlns:r" , "http://schemas.openxmlformats.org/officeDocument/2006/relationships"));
        attributes.push(("r:embed", r_id_str.as_str()));
        if &self.cstate != "" {
            attributes.push(("cstate", &self.cstate));
        }
        write_start_tag(writer, "a:blip", attributes, false);

        // a:extLst
        write_start_tag(writer, "a:extLst", vec![], false);

        // a:ext
        write_start_tag(writer, "a:ext", vec![
            ("uri", "{28A0092B-C50C-407E-A947-70E740481C1C}"),
        ], false);

        // a14:useLocalDpi
        write_start_tag(writer, "a14:useLocalDpi", vec![
            ("xmlns:a14", "http://schemas.microsoft.com/office/drawing/2010/main"),
            ("val", "0"),
        ], true);
        write_end_tag(writer, "a:ext");
        write_end_tag(writer, "a:extLst");
        write_end_tag(writer, "a:blip");
    }
}
