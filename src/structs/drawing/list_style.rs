// a:lstStyle
use super::EffectList;
use super::TextParagraphPropertiesType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ListStyle {
    effect_list: Option<EffectList>,
    default_paragraph_properties: Option<TextParagraphPropertiesType>,
    level1_paragraph_properties: Option<TextParagraphPropertiesType>,
    level2_paragraph_properties: Option<TextParagraphPropertiesType>,
    level3_paragraph_properties: Option<TextParagraphPropertiesType>,
    level4_paragraph_properties: Option<TextParagraphPropertiesType>,
    level5_paragraph_properties: Option<TextParagraphPropertiesType>,
    level6_paragraph_properties: Option<TextParagraphPropertiesType>,
    level7_paragraph_properties: Option<TextParagraphPropertiesType>,
    level8_paragraph_properties: Option<TextParagraphPropertiesType>,
    level9_paragraph_properties: Option<TextParagraphPropertiesType>,
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
        self.default_paragraph_properties.as_ref()
    }

    pub fn get_default_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.default_paragraph_properties.as_mut()
    }

    pub fn set_default_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.default_paragraph_properties = Some(value);
        self
    }

    pub fn get_level1_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level1_paragraph_properties.as_ref()
    }

    pub fn get_level1_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level1_paragraph_properties.as_mut()
    }

    pub fn set_level1_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level1_paragraph_properties = Some(value);
        self
    }

    pub fn get_level2_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level2_paragraph_properties.as_ref()
    }

    pub fn get_level2_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level2_paragraph_properties.as_mut()
    }

    pub fn set_level2_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level2_paragraph_properties = Some(value);
        self
    }

    pub fn get_level3_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level3_paragraph_properties.as_ref()
    }

    pub fn get_level3_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level3_paragraph_properties.as_mut()
    }

    pub fn set_level3_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level3_paragraph_properties = Some(value);
        self
    }

    pub fn get_level4_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level4_paragraph_properties.as_ref()
    }

    pub fn get_level4_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level4_paragraph_properties.as_mut()
    }

    pub fn set_level4_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level4_paragraph_properties = Some(value);
        self
    }

    pub fn get_level5_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level5_paragraph_properties.as_ref()
    }

    pub fn get_level5_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level5_paragraph_properties.as_mut()
    }

    pub fn set_level5_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level5_paragraph_properties = Some(value);
        self
    }

    pub fn get_level6_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level6_paragraph_properties.as_ref()
    }

    pub fn get_level6_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level6_paragraph_properties.as_mut()
    }

    pub fn set_level6_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level6_paragraph_properties = Some(value);
        self
    }

    pub fn get_level7_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level7_paragraph_properties.as_ref()
    }

    pub fn get_level7_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level7_paragraph_properties.as_mut()
    }

    pub fn set_level7_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level7_paragraph_properties = Some(value);
        self
    }

    pub fn get_level8_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level8_paragraph_properties.as_ref()
    }

    pub fn get_level8_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level8_paragraph_properties.as_mut()
    }

    pub fn set_level8_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level8_paragraph_properties = Some(value);
        self
    }

    pub fn get_level9_paragraph_properties(&self) -> Option<&TextParagraphPropertiesType> {
        self.level9_paragraph_properties.as_ref()
    }

    pub fn get_level9_paragraph_properties_mut(
        &mut self,
    ) -> Option<&mut TextParagraphPropertiesType> {
        self.level9_paragraph_properties.as_mut()
    }

    pub fn set_level9_paragraph_properties(
        &mut self,
        value: TextParagraphPropertiesType,
    ) -> &mut Self {
        self.level9_paragraph_properties = Some(value);
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
            Event::Eof => panic!("Error not find {} end element", "a:lstStyle")
        );
    }

    fn is_empty(&self) -> bool {
        self.effect_list.is_none()
            && self.default_paragraph_properties.is_none()
            && self.level1_paragraph_properties.is_none()
            && self.level2_paragraph_properties.is_none()
            && self.level3_paragraph_properties.is_none()
            && self.level4_paragraph_properties.is_none()
            && self.level5_paragraph_properties.is_none()
            && self.level6_paragraph_properties.is_none()
            && self.level7_paragraph_properties.is_none()
            && self.level8_paragraph_properties.is_none()
            && self.level9_paragraph_properties.is_none()
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lstStyle
        let is_empty = self.is_empty();
        write_start_tag(writer, "a:lstStyle", vec![], is_empty);

        // a:defPPr
        match &self.default_paragraph_properties {
            Some(v) => {
                v.write_to_default(writer);
            }
            None => {}
        }

        // a:lvl1pPr
        match &self.level1_paragraph_properties {
            Some(v) => {
                v.write_to_lvl1(writer);
            }
            None => {}
        }

        // a:lvl2pPr
        match &self.level2_paragraph_properties {
            Some(v) => {
                v.write_to_lvl2(writer);
            }
            None => {}
        }

        // a:lvl3pPr
        match &self.level3_paragraph_properties {
            Some(v) => {
                v.write_to_lvl3(writer);
            }
            None => {}
        }

        // a:lvl4pPr
        match &self.level4_paragraph_properties {
            Some(v) => {
                v.write_to_lvl4(writer);
            }
            None => {}
        }

        // a:lvl5pPr
        match &self.level5_paragraph_properties {
            Some(v) => {
                v.write_to_lvl5(writer);
            }
            None => {}
        }

        // a:lvl6pPr
        match &self.level6_paragraph_properties {
            Some(v) => {
                v.write_to_lvl6(writer);
            }
            None => {}
        }

        // a:lvl7pPr
        match &self.level7_paragraph_properties {
            Some(v) => {
                v.write_to_lvl7(writer);
            }
            None => {}
        }

        // a:lvl8pPr
        match &self.level8_paragraph_properties {
            Some(v) => {
                v.write_to_lvl8(writer);
            }
            None => {}
        }

        // a:lvl9pPr
        match &self.level9_paragraph_properties {
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
