// a:lstStyle
use super::EffectList;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ListStyle {
    effect_list: Option<EffectList>,
}
impl ListStyle {
    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value: EffectList) {
        self.effect_list = Some(value);
    }

    pub(crate) fn _set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:effectLst" => {
                        let obj = EffectList::default();
                        self.set_effect_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:lstStyle" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:lstStyle"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn _write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lstStyle
        write_start_tag(writer, "a:lstStyle", vec![], true);
    }
}
