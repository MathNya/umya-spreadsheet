// a:lstStyle
use std::{
    collections::HashMap,
    io::Cursor,
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    EffectList,
    TextParagraphPropertiesType,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ListStyle {
    effect_list: Option<Box<EffectList>>,
    text_paragraph_properties_type: HashMap<Box<str>, Box<TextParagraphPropertiesType>>,
}

impl ListStyle {
    #[inline]
    #[must_use]
    pub fn get_effect_list(&self) -> Option<&EffectList> {
        self.effect_list.as_deref()
    }

    #[inline]
    pub fn get_effect_list_mut(&mut self) -> Option<&mut EffectList> {
        self.effect_list.as_deref_mut()
    }

    #[inline]
    pub fn set_effect_list(&mut self, value: EffectList) -> &mut Self {
        self.effect_list = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn get_default_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("def")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_default_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("def")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_default_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("def").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level1_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv1")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level1_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv1")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level1_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv1").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level2_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv2")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level2_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv2")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level2_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv2").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level3_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv3")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level3_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv3")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level3_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv3").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level4_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv4")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level4_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv4")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level4_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv4").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level5_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv5")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level5_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv5")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level5_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv5").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level6_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv6")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level6_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv6")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level6_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv6").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level7_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv7")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level7_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv7")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level7_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv7").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level8_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv8")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level8_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv8")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level8_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv8").into_boxed_str(), Box::new(value));
        self
    }

    #[inline]
    pub fn get_level9_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get("lv9")
            .map(Box::as_ref)
    }

    #[inline]
    pub fn get_level9_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.text_paragraph_properties_type
            .get_mut("lv9")
            .map(Box::as_mut)
    }

    #[inline]
    pub fn set_level9_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.text_paragraph_properties_type
            .insert(String::from("lv9").into_boxed_str(), Box::new(value));
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

    #[inline]
    fn is_empty(&self) -> bool {
        self.effect_list.is_none() && self.text_paragraph_properties_type.is_empty()
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lstStyle
        let is_empty = self.is_empty();
        write_start_tag(writer, "a:lstStyle", vec![], is_empty);

        // a:defPPr
        if let Some(v) = &self.text_paragraph_properties_type.get("def") {
            v.write_to_default(writer);
        }

        // a:lvl1pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv1") {
            v.write_to_lvl1(writer);
        }

        // a:lvl2pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv2") {
            v.write_to_lvl2(writer);
        }

        // a:lvl3pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv3") {
            v.write_to_lvl3(writer);
        }

        // a:lvl4pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv4") {
            v.write_to_lvl4(writer);
        }

        // a:lvl5pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv5") {
            v.write_to_lvl5(writer);
        }

        // a:lvl6pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv6") {
            v.write_to_lvl6(writer);
        }

        // a:lvl7pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv7") {
            v.write_to_lvl7(writer);
        }

        // a:lvl8pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv8") {
            v.write_to_lvl8(writer);
        }

        // a:lvl9pPr
        if let Some(v) = &self.text_paragraph_properties_type.get("lv9") {
            v.write_to_lvl9(writer);
        }

        if !is_empty {
            write_end_tag(writer, "a:lstStyle");
        }
    }
}
