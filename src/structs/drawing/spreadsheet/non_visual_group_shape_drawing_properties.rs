// xdr:cNvGrpSpPr
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::super::GroupShapeLocks;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct NonVisualGroupShapeDrawingProperties {
    group_shape_locks: Option<GroupShapeLocks>,
}

impl NonVisualGroupShapeDrawingProperties {
    #[inline]
    #[must_use]
    pub fn get_group_shape_locks(&self) -> Option<&GroupShapeLocks> {
        self.group_shape_locks.as_ref()
    }

    #[inline]
    pub fn get_group_shape_locks_mut(&mut self) -> Option<&mut GroupShapeLocks> {
        self.group_shape_locks.as_mut()
    }

    #[inline]
    pub fn set_group_shape_locks(&mut self, value: GroupShapeLocks) -> &mut Self {
        self.group_shape_locks = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flg: bool,
    ) {
        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:grpSpLocks" {
                    let mut obj = GroupShapeLocks::default();
                    obj.set_attributes(reader, e);
                    self.set_group_shape_locks(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:cNvGrpSpPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:cNvGrpSpPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let is_empty = self.group_shape_locks.is_none();
        // xdr:cNvGrpSpPr
        write_start_tag(writer, "xdr:cNvGrpSpPr", vec![], is_empty);

        // a:grpSpLocks
        if let Some(v) = &self.group_shape_locks {
            v.write_to(writer);
        }

        if !is_empty {
            write_end_tag(writer, "xdr:cNvGrpSpPr");
        }
    }
}
