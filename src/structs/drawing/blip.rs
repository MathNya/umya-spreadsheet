// a:blip
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::MediaObject;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Blip {
    image: MediaObject,
    cstate: String,
}

impl Blip {
    pub fn get_image(&self) -> &MediaObject {
        &self.image
    }

    pub fn get_image_mut(&mut self) -> &mut MediaObject {
        &mut self.image
    }

    pub fn set_image(&mut self, value: MediaObject) -> &mut Self {
        self.image = value;
        self
    }

    pub fn get_cstate(&self) -> &str {
        &self.cstate
    }

    pub fn set_cstate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cstate = value.into();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: &RawRelationships,
        empty_flag: bool,
    ) {
        if let Some(v) = get_attribute(e, b"cstate") {
            self.set_cstate(v);
        }

        let picture_id = get_attribute(e, b"r:embed").unwrap();
        let relationship = drawing_relationships.get_relationship_by_rid(&picture_id);
        self.get_image_mut()
            .set_image_name(relationship.get_raw_file().get_file_name());
        self.get_image_mut()
            .set_image_data(relationship.get_raw_file().get_file_data().clone());

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:blip" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:blip")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // a:blip
        let r_id = self.image.get_rid(rel_list);
        let r_id_str = format!("rId{}", r_id);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("xmlns:r", REL_OFC_NS));
        attributes.push(("r:embed", r_id_str.as_str()));
        if !&self.cstate.is_empty() {
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
            vec![("xmlns:a14", DRAWING_MAIN_NS), ("val", "0")],
            true,
        );
        write_end_tag(writer, "a:ext");
        write_end_tag(writer, "a:extLst");
        write_end_tag(writer, "a:blip");
    }
}
