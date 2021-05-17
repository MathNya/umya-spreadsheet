// a:effectLst
use super::outer_shadow::OuterShadow;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct EffectList {
    outer_shadow: Option<OuterShadow>,
}
impl EffectList {
    pub fn get_outer_shadow(&self) -> &Option<OuterShadow> {
        &self.outer_shadow
    }

    pub fn get_outer_shadow_mut(&mut self) -> &mut Option<OuterShadow> {
        &mut self.outer_shadow
    }

    pub fn set_outer_shadow(&mut self, value:OuterShadow) {
        self.outer_shadow = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:outerShdw" => {
                            let mut outer_shadow = OuterShadow::default();
                            outer_shadow.set_attributes(reader, e);
                            &mut self.set_outer_shadow(outer_shadow);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:effectLst" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:effectLst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:effectLst
        write_start_tag(writer, "a:effectLst", vec![], false);

        // a:outerShdow
        match &self.outer_shadow {
            Some(v) => v.write_to(writer),
            None => {},
        }

        write_end_tag(writer, "a:effectLst");
    }
}
