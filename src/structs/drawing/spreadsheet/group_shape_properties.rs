// xdr:grpSpPr
use std::io::Cursor;

use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};

use super::super::Transform2D;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct GroupShapeProperties {
    transform2d: Option<Transform2D>,
}

impl GroupShapeProperties {
    #[inline]
    #[must_use]
    pub fn get_transform2d(&self) -> Option<&Transform2D> {
        self.transform2d.as_ref()
    }

    #[inline]
    pub fn get_transform2d_mut(&mut self) -> Option<&mut Transform2D> {
        self.transform2d.as_mut()
    }

    #[inline]
    pub fn set_transform2d(&mut self, value: Transform2D) -> &mut Self {
        self.transform2d = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().0 == b"a:xfrm" {
                    let mut obj = Transform2D::default();
                    obj.set_attributes(reader, e);
                    self.set_transform2d(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"xdr:grpSpPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:grpSpPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:grpSpPr
        write_start_tag(writer, "xdr:grpSpPr", vec![], false);

        // a:xfrm
        if let Some(v) = &self.transform2d {
            v.write_to(writer);
        }

        write_end_tag(writer, "xdr:grpSpPr");
    }
}
