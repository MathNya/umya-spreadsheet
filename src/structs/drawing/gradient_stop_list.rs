// a:gsLst
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::GradientStop;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct GradientStopList {
    gradient_stop: Vec<GradientStop>,
}

impl GradientStopList {
    #[inline]
    #[must_use]
    pub fn get_gradient_stop(&self) -> &[GradientStop] {
        &self.gradient_stop
    }

    #[inline]
    pub fn get_gradient_stop_mut(&mut self) -> &mut Vec<GradientStop> {
        &mut self.gradient_stop
    }

    #[inline]
    pub fn set_gradient_stop(
        &mut self,
        value: impl Into<Vec<GradientStop>>,
    ) -> &mut GradientStopList {
        self.gradient_stop = value.into();
        self
    }

    #[inline]
    pub fn add_gradient_stop(&mut self, value: GradientStop) -> &mut GradientStopList {
        self.gradient_stop.push(value);
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
                if e.name().into_inner() == b"a:gs" {
                    let mut obj = GradientStop::default();
                    obj.set_attributes(reader, e);
                    self.add_gradient_stop(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:gsLst" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:gsLst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:gsLst
        write_start_tag(writer, "a:gsLst", vec![], false);

        // a:gs
        for v in &self.gradient_stop {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:gsLst");
    }
}
