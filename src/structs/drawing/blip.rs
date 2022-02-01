// a:blip
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use reader::xlsx::drawing_rels;
use reader::xlsx::media;
use std::io::Cursor;
use structs::Image;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Blip {
    image: Image,
    cstate: String,
}
impl Blip {
    pub fn get_image(&self) -> &Image {
        &self.image
    }

    pub fn get_image_mut(&mut self) -> &mut Image {
        &mut self.image
    }

    pub fn set_image(&mut self, value: Image) {
        self.image = value;
    }

    pub fn get_cstate(&self) -> &str {
        &self.cstate
    }

    pub fn set_cstate<S: Into<String>>(&mut self, value: S) {
        self.cstate = value.into();
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        target: &str,
    ) {
        match get_attribute(e, b"cstate") {
            Some(v) => {
                &mut self.set_cstate(v);
            }
            None => {}
        }

        let picture_id = get_attribute(e, b"r:embed").unwrap();
        let drawing_rel = drawing_rels::read(arv, target).unwrap();
        for (drawing_id, _, drawing_target) in &drawing_rel {
            if &picture_id == drawing_id {
                let v: Vec<&str> = drawing_target.split('/').collect();
                let image_name = v.last().unwrap().clone();
                &mut self.get_image_mut().set_image_name(image_name);
                &mut self
                    .get_image_mut()
                    .set_image_data(media::read(arv, &drawing_target).unwrap());
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // a:blip
        let r_id_str = format!("rId{}", r_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push((
            "xmlns:r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        ));
        attributes.push(("r:embed", r_id_str.as_str()));
        if &self.cstate != "" {
            attributes.push(("cstate", &self.cstate));
        }
        write_start_tag(writer, "a:blip", attributes, false);

        // a:extLst
        write_start_tag(writer, "a:extLst", vec![], false);

        // a:ext
        write_start_tag(
            writer,
            "a:ext",
            vec![("uri", "{28A0092B-C50C-407E-A947-70E740481C1C}")],
            false,
        );

        // a14:useLocalDpi
        write_start_tag(
            writer,
            "a14:useLocalDpi",
            vec![
                (
                    "xmlns:a14",
                    "http://schemas.microsoft.com/office/drawing/2010/main",
                ),
                ("val", "0"),
            ],
            true,
        );
        write_end_tag(writer, "a:ext");
        write_end_tag(writer, "a:extLst");
        write_end_tag(writer, "a:blip");
    }
}
