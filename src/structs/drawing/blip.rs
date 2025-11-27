// a:blip
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    helper::const_str::{
        DRAWING_MAIN_NS,
        REL_OFC_NS,
    },
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    structs::{
        MediaObject,
        raw::RawRelationships,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Blip {
    image:  MediaObject,
    cstate: Box<str>,
}

impl Blip {
    #[inline]
    #[must_use]
    pub fn image(&self) -> &MediaObject {
        &self.image
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use image()")]
    pub fn get_image(&self) -> &MediaObject {
        self.image()
    }

    #[inline]
    pub fn image_mut(&mut self) -> &mut MediaObject {
        &mut self.image
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_mut()")]
    pub fn get_image_mut(&mut self) -> &mut MediaObject {
        self.image_mut()
    }

    #[inline]
    pub fn set_image(&mut self, value: MediaObject) -> &mut Self {
        self.image = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn cstate(&self) -> &str {
        &self.cstate
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cstate()")]
    pub fn get_cstate(&self) -> &str {
        self.cstate()
    }

    #[inline]
    pub fn set_cstate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cstate = value.into().into_boxed_str();
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
        let relationship = drawing_relationships.relationship_by_rid(&picture_id);
        self.image_mut()
            .set_image_name(relationship.raw_file().file_name());
        self.image_mut()
            .set_image_data(relationship.raw_file().file_data());

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
        let r_id = self.image.rid(rel_list);
        let r_id_str = format!("rId{r_id}");
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("xmlns:r", REL_OFC_NS).into());
        attributes.push(("r:embed", &r_id_str).into());
        if !&self.cstate.is_empty() {
            attributes.push(("cstate", &self.cstate).into());
        }
        write_start_tag(writer, "a:blip", attributes, false);

        // a:extLst
        write_start_tag(writer, "a:extLst", vec![], false);

        // a:ext
        write_start_tag(
            writer,
            "a:ext",
            vec![("uri", "{28A0092B-C50C-407E-A947-70E740481C1C}").into()],
            false,
        );

        // a14:useLocalDpi
        write_start_tag(
            writer,
            "a14:useLocalDpi",
            vec![("xmlns:a14", DRAWING_MAIN_NS).into(), ("val", "0").into()],
            true,
        );
        write_end_tag(writer, "a:ext");
        write_end_tag(writer, "a:extLst");
        write_end_tag(writer, "a:blip");
    }
}
