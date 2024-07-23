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
        self.name.get_value_str()
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
        if let Some(v) = get_attribute(e, b"name") {
            self.name.set_value(v);
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:fmtScheme" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:fmtScheme")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fmtScheme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_str()));
        }
        write_start_tag(writer, "a:fmtScheme", attributes, false);

        // a:fillStyleLst
        self.fill_style_list.write_to(writer);

        // a:lnStyleLst
        self.line_style_list.write_to(writer);

        // a:effectStyleLst
        self.effect_style_list.write_to(writer);

        // a:bgFillStyleLst
        self.background_fill_style_list.write_to(writer);

        write_end_tag(writer, "a:fmtScheme");
    }
}
