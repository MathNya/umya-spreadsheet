// a:lnSpc
use super::SpacingPercent;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LineSpacing {
    spacing_percent: Option<SpacingPercent>,
}

impl LineSpacing {
    pub fn get_spacing_percent(&self) -> Option<&SpacingPercent> {
        self.spacing_percent.as_ref()
    }

    pub fn get_spacing_percent_mut(&mut self) -> Option<&mut SpacingPercent> {
        self.spacing_percent.as_mut()
    }

    pub fn set_spacing_percent(&mut self, value: SpacingPercent) -> &mut Self {
        self.spacing_percent = Some(value);
        self
    }

    pub fn remove_spacing_percent(&mut self) {
        self.spacing_percent = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:spcPct" {
                    let mut obj = SpacingPercent::default();
                    obj.set_attributes(reader, e);
                    self.set_spacing_percent(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lnSpc" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lnSpc")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lnSpc
        write_start_tag(writer, "a:lnSpc", vec![], false);

        // a:spcPct
        if let Some(v) = &self.spacing_percent {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:lnSpc");
    }
}
