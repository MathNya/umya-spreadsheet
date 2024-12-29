// xdr:cNvPicPr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::super::PictureLocks;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::BooleanValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct NonVisualPictureDrawingProperties {
    prefer_relative_resize: BooleanValue,
    picture_locks:          Option<PictureLocks>,
}

impl NonVisualPictureDrawingProperties {
    #[must_use]
    pub fn get_prefer_relative_resize(&self) -> bool {
        self.prefer_relative_resize.get_value()
    }

    pub fn set_prefer_relative_resize(&mut self, value: bool) {
        self.prefer_relative_resize.set_value(value);
    }

    #[must_use]
    pub fn get_picture_locks(&self) -> Option<&PictureLocks> {
        self.picture_locks.as_ref()
    }

    pub fn get_picture_locks_mut(&mut self) -> Option<&mut PictureLocks> {
        self.picture_locks.as_mut()
    }

    pub fn set_picture_locks(&mut self, value: PictureLocks) {
        self.picture_locks = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, prefer_relative_resize, "preferRelativeResize");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:picLocks" {
                    let mut obj = PictureLocks::default();
                    obj.set_attributes(reader, e);
                    self.set_picture_locks(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:cNvPicPr" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:cNvPicPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvPicPr
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.prefer_relative_resize.has_value() {
            attributes.push(
                (
                    "preferRelativeResize",
                    self.prefer_relative_resize.get_value_string(),
                )
                    .into(),
            );
        }

        match &self.picture_locks {
            Some(v) => {
                write_start_tag(writer, "xdr:cNvPicPr", attributes, false);
                v.write_to(writer);
                write_end_tag(writer, "xdr:cNvPicPr");
            }
            None => {
                write_start_tag(writer, "xdr:cNvPicPr", attributes, true);
            }
        }
    }
}
