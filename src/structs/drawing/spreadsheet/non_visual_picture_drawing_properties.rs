//xdr:cNvPicPr
use super::super::PictureLocks;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::BooleanValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualPictureDrawingProperties {
    prefer_relative_resize: BooleanValue,
    picture_locks: Option<PictureLocks>,
}

impl NonVisualPictureDrawingProperties {
    pub fn get_prefer_relative_resize(&self) -> &bool {
        self.prefer_relative_resize.get_value()
    }

    pub fn set_prefer_relative_resize(&mut self, value: bool) {
        self.prefer_relative_resize.set_value(value);
    }

    pub fn get_picture_locks(&self) -> &Option<PictureLocks> {
        &self.picture_locks
    }

    pub fn get_picture_locks_mut(&mut self) -> &mut Option<PictureLocks> {
        &mut self.picture_locks
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
            Event::Eof => panic!("Error not find {} end element", "xdr:cNvPicPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let is_empty = self.picture_locks.is_none();

        // xdr:cNvPicPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.prefer_relative_resize.has_value() {
            attributes.push((
                "preferRelativeResize",
                self.prefer_relative_resize.get_value_string(),
            ));
        }
        write_start_tag(writer, "xdr:cNvPicPr", attributes, is_empty);

        if !is_empty {
            // a:picLocks
            match &self.picture_locks {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "xdr:cNvPicPr");
        }
    }
}
