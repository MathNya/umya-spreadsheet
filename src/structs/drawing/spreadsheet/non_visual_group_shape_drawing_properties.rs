// xdr:cNvGrpSpPr
use super::super::GroupShapeLocks;
use super::NonVisualDrawingProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualGroupShapeDrawingProperties {
    group_shape_locks: Option<GroupShapeLocks>,
}

impl NonVisualGroupShapeDrawingProperties {
    pub fn get_group_shape_locks(&self) -> Option<&GroupShapeLocks> {
        self.group_shape_locks.as_ref()
    }

    pub fn get_group_shape_locks_mut(&mut self) -> Option<&mut GroupShapeLocks> {
        self.group_shape_locks.as_mut()
    }

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
        match &self.group_shape_locks {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        if !is_empty {
            write_end_tag(writer, "xdr:cNvGrpSpPr");
        }
    }
}
