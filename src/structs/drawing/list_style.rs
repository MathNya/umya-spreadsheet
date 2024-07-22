// a:lstStyle
use super::EffectList;
use super::TextParagraphPropertiesType;
use hashbrown::HashMap;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ListStyle {
    effect_list: Option<EffectList>,
    text_paragraph_properties_type: HashMap<String, TextParagraphPropertiesType>,
}

impl ListStyle {
    pub fn get_effect_list(&self) -> Option<&EffectList> {
        self.effect_list.as_ref()
    }

    pub fn get_effect_list_mut(&mut self) -> Option<&mut EffectList> {
        self.effect_list.as_mut()
    }

    pub fn set_effect_list(&mut self, value: EffectList) -> &mut Self {
        self.effect_list = Some(value);
        self
    }

    pub fn get_default_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("def")
    }

    pub fn get_default_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("def")
    }

    pub fn set_default_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("def"), value);
        self
    }

    pub fn get_level1_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv1")
    }

    pub fn get_level1_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv1")
    }

    pub fn set_level1_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv1"), value);
        self
    }

    pub fn get_level2_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv2")
    }

    pub fn get_level2_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv2")
    }

    pub fn set_level2_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv2"), value);
        self
    }

    pub fn get_level3_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv3")
    }

    pub fn get_level3_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv3")
    }

    pub fn set_level3_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv3"), value);
        self
    }

    pub fn get_level4_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv4")
    }

    pub fn get_level4_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv4")
    }

    pub fn set_level4_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv4"), value);
        self
    }

    pub fn get_level5_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv5")
    }

    pub fn get_level5_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv5")
    }

    pub fn set_level5_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv5"), value);
        self
    }

    pub fn get_level6_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv6")
    }

    pub fn get_level6_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv6")
    }

    pub fn set_level6_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv6"), value);
        self
    }

    pub fn get_level7_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv7")
    }

    pub fn get_level7_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv7")
    }

    pub fn set_level7_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv7"), value);
        self
    }

    pub fn get_level8_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv8")
    }

    pub fn get_level8_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv8")
    }

    pub fn set_level8_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv8"), value);
        self
    }

    pub fn get_level9_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get("lv9")
    }

    pub fn get_level9_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type.get_mut("lv9")
    }

    pub fn set_level9_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv9"), value);
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
                match e.name().into_inner() {
                    b"a:effectLst" => {
                        let obj = EffectList::default();
                        self.set_effect_list(obj);
                    }
                    b"a:defPPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_default_paragraph_properties(obj);
                    }
                    b"a:lvl1pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level1_paragraph_properties(obj);
                    }
                    b"a:lvl2pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level2_paragraph_properties(obj);
                    }
                    b"a:lvl3pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level3_paragraph_properties(obj);
                    }
                    b"a:lvl4pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level4_paragraph_properties(obj);
                    }
                    b"a:lvl5pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level5_paragraph_properties(obj);
                    }
                    b"a:lvl6pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level6_paragraph_properties(obj);
                    }
                    b"a:lvl7pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level7_paragraph_properties(obj);
                    }
                    b"a:lvl8pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level8_paragraph_properties(obj);
                    }
                    b"a:lvl9pPr" => {
                        let mut obj = TextParagraphPropertiesType::default();
                        obj.set_attributes(reader, e);
                        self.set_level9_paragraph_properties(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lstStyle" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lstStyle")
        );
    }

    fn is_empty(&self) -> bool {
        self.effect_list.is_none() && self.text_paragraph_properties_type.is_empty()
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lstStyle
        let is_empty = self.is_empty();
        write_start_tag(writer, "a:lstStyle", vec![], is_empty);

        // a:defPPr
        match &self.text_paragraph_properties_type.get("def") {
            Some(v) => {
                v.write_to_default(writer);
            }
            None => {}
        }

        // a:lvl1pPr
        match &self.text_paragraph_properties_type.get("lv1") {
            Some(v) => {
                v.write_to_lvl1(writer);
            }
            None => {}
        }

        // a:lvl2pPr
        match &self.text_paragraph_properties_type.get("lv2") {
            Some(v) => {
                v.write_to_lvl2(writer);
            }
            None => {}
        }

        // a:lvl3pPr
        match &self.text_paragraph_properties_type.get("lv3") {
            Some(v) => {
                v.write_to_lvl3(writer);
            }
            None => {}
        }

        // a:lvl4pPr
        match &self.text_paragraph_properties_type.get("lv4") {
            Some(v) => {
                v.write_to_lvl4(writer);
            }
            None => {}
        }

        // a:lvl5pPr
        match &self.text_paragraph_properties_type.get("lv5") {
            Some(v) => {
                v.write_to_lvl5(writer);
            }
            None => {}
        }

        // a:lvl6pPr
        match &self.text_paragraph_properties_type.get("lv6") {
            Some(v) => {
                v.write_to_lvl6(writer);
            }
            None => {}
        }

        // a:lvl7pPr
        match &self.text_paragraph_properties_type.get("lv7") {
            Some(v) => {
                v.write_to_lvl7(writer);
            }
            None => {}
        }

        // a:lvl8pPr
        match &self.text_paragraph_properties_type.get("lv8") {
            Some(v) => {
                v.write_to_lvl8(writer);
            }
            None => {}
        }

        // a:lvl9pPr
        match &self.text_paragraph_properties_type.get("lv9") {
            Some(v) => {
                v.write_to_lvl9(writer);
            }
            None => {}
        }

        if !is_empty {
            write_end_tag(writer, "a:lstStyle");
        }
    }
}
