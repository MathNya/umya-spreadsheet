// a:lnSpc
use super::SpacingPercent;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LineSpacing {
    spacing_percent: Option<SpacingPercent>,
}
impl LineSpacing {
    pub fn get_spacing_percent(&self) -> &Option<SpacingPercent> {
        &self.spacing_percent
    }

    pub fn get_spacing_percent_mut(&mut self) -> &mut Option<SpacingPercent> {
        &mut self.spacing_percent
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:spcPct" => {
                        let mut obj = SpacingPercent::default();
                        obj.set_attributes(reader, e);
                        self.set_spacing_percent(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:lnSpc" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:lnSpc"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lnSpc
        write_start_tag(writer, "a:lnSpc", vec![], false);

        // a:spcPct
        match &self.spacing_percent {
            Some(v) => v.write_to(writer),
            None => {}
        }

        write_end_tag(writer, "a:lnSpc");
    }
}
