use super::EffectStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct EffectStyleList {
    effect_style_collection: Vec<EffectStyle>,
}
impl EffectStyleList {
    pub fn get_effect_style_collection(&self) -> &Vec<EffectStyle> {
        &self.effect_style_collection
    }

    pub fn get_effect_style_collection_mut(&mut self) -> &mut Vec<EffectStyle> {
        &mut self.effect_style_collection
    }

    pub fn set_effect_style_collection(&mut self, value: Vec<EffectStyle>) -> &mut Self {
        self.effect_style_collection = value;
        self
    }

    pub fn add_effect_style_collection(&mut self, value: EffectStyle) -> &mut Self {
        self.effect_style_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:effectStyle" => {
                        let mut obj = EffectStyle::default();
                        obj.set_attributes(reader, e);
                        self.effect_style_collection.push(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:effectStyleLst" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:effectStyleLst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:effectStyleLst
        write_start_tag(writer, "a:effectStyleLst", vec![], false);

        // a:effectStyle
        for v in &self.effect_style_collection {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:effectStyleLst");
    }
}
