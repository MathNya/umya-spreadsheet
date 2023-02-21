// a:fontScheme
use super::super::StringValue;
use super::BackgroundFillStyleList;
use super::EffectStyleList;
use super::FillStyleList;
use super::LineStyleList;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct FormatScheme {
    name: StringValue,
    fill_style_list: FillStyleList,
    line_style_list: LineStyleList,
    effect_style_list: EffectStyleList,
    background_fill_style_list: BackgroundFillStyleList,
}
impl FormatScheme {
    pub fn get_name(&self) -> &str {
        self.name.get_value()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_fill_style_list(&self) -> &FillStyleList {
        &self.fill_style_list
    }

    pub fn get_fill_style_list_mut(&mut self) -> &mut FillStyleList {
        &mut self.fill_style_list
    }

    pub fn set_fill_style_list(&mut self, value: FillStyleList) {
        self.fill_style_list = value;
    }

    pub fn get_line_style_list(&self) -> &LineStyleList {
        &self.line_style_list
    }

    pub fn get_line_style_list_mut(&mut self) -> &mut LineStyleList {
        &mut self.line_style_list
    }

    pub fn set_line_style_list(&mut self, value: LineStyleList) {
        self.line_style_list = value;
    }

    pub fn get_effect_style_list(&self) -> &EffectStyleList {
        &self.effect_style_list
    }

    pub fn get_effect_style_list_mut(&mut self) -> &mut EffectStyleList {
        &mut self.effect_style_list
    }

    pub fn set_effect_style_list(&mut self, value: EffectStyleList) {
        self.effect_style_list = value;
    }

    pub fn get_background_fill_style_list(&self) -> &BackgroundFillStyleList {
        &self.background_fill_style_list
    }

    pub fn get_background_fill_style_list_mut(&mut self) -> &mut BackgroundFillStyleList {
        &mut self.background_fill_style_list
    }

    pub fn set_background_fill_style_list_list(&mut self, value: BackgroundFillStyleList) {
        self.background_fill_style_list = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"name") {
            Some(v) => {
                self.name.set_value(v);
            }
            _ => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:fillStyleLst" => {
                        let mut obj = FillStyleList::default();
                        obj.set_attributes(reader, e);
                        self.fill_style_list = obj;
                    }
                    b"a:lnStyleLst" => {
                        let mut obj = LineStyleList::default();
                        obj.set_attributes(reader, e);
                        self.line_style_list = obj;
                    }
                    b"a:effectStyleLst" => {
                        let mut obj = EffectStyleList::default();
                        obj.set_attributes(reader, e);
                        self.effect_style_list = obj;
                    }
                    b"a:bgFillStyleLst" => {
                        let mut obj = BackgroundFillStyleList::default();
                        obj.set_attributes(reader, e);
                        self.background_fill_style_list = obj;
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:fmtScheme" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:fmtScheme"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fmtScheme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_string()));
        }
        write_start_tag(writer, "a:fmtScheme", attributes, false);

        // a:fillStyleLst
        let _ = &self.fill_style_list.write_to(writer);

        // a:lnStyleLst
        let _ = &self.line_style_list.write_to(writer);

        // a:effectStyleLst
        let _ = &self.effect_style_list.write_to(writer);

        // a:bgFillStyleLst
        let _ = &self.background_fill_style_list.write_to(writer);

        write_end_tag(writer, "a:fmtScheme");
    }
}
