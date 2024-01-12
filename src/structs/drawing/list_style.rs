// a:lstStyle
use super::EffectList;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ListStyle {
    effect_list: Option<EffectList>,
}

impl ListStyle {
    pub fn get_effect_list(&self) -> Option<&EffectList> {
        self.effect_list.as_ref()
    }

    pub fn get_effect_list_mut(&mut self) -> Option<&mut EffectList> {
        self.effect_list.as_mut()
    }

    pub fn set_effect_list(&mut self, value: EffectList) {
        self.effect_list = Some(value);
    }

    pub(crate) fn _set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:effectLst" {
                    let obj = EffectList::default();
                    self.set_effect_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lstStyle" {
                    return;
                }
            },
            Event::Eof => panic!("Error not find {} end element", "a:lstStyle")
        );
    }

    pub(crate) fn _write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lstStyle
        write_start_tag(writer, "a:lstStyle", vec![], true);
    }
}
