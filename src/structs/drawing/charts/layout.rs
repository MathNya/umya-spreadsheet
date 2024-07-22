// c:layout
use super::ManualLayout;
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Layout {
    manual_layout: Option<ManualLayout>,
}

impl Layout {
    pub fn get_manual_layout(&self) -> Option<&ManualLayout> {
        self.manual_layout.as_ref()
    }

    pub fn get_manual_layout_mut(&mut self) -> Option<&mut ManualLayout> {
        self.manual_layout.as_mut()
    }

    pub fn set_manual_layout(&mut self, value: ManualLayout) -> &mut Layout {
        self.manual_layout = Some(value);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.manual_layout.is_none()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"c:manualLayout" {
                    let mut obj = ManualLayout::default();
                    obj.set_attributes(reader, e);
                    self.set_manual_layout(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:layout" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:layout"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.is_empty() {
            // c:layout
            write_start_tag(writer, "c:layout", vec![], true);
        } else {
            // c:layout
            write_start_tag(writer, "c:layout", vec![], false);

            // c:manualLayout
            if let Some(v) = &self.manual_layout {
                v.write_to(writer);
            }

            write_end_tag(writer, "c:layout");
        }
    }
}
