use crate::drawing::charts::View3D;

// sst
use super::drawing::Theme;
use super::CellValue;
use super::SharedStringItem;
use hashbrown::HashMap;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct SharedStringTable {
    shared_string_item: Vec<SharedStringItem>,
    map: HashMap<u64, usize>,
    regist_count: usize,
}

impl SharedStringTable {
    pub(crate) fn get_shared_string_item(&self) -> &Vec<SharedStringItem> {
        &self.shared_string_item
    }

    pub(crate) fn get_shared_string_item_mut(&mut self) -> &mut Vec<SharedStringItem> {
        &mut self.shared_string_item
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        self.shared_string_item.push(value);
        self
    }

    pub(crate) fn has_value(&self) -> bool {
        !self.shared_string_item.is_empty()
    }

    pub(crate) fn set_cell(&mut self, value: &CellValue) -> usize {
        self.regist_count += 1;

        let mut shared_string_item = SharedStringItem::default();
        if let Some(v) = value.get_text() {
            shared_string_item.set_text(v);
        }
        if let Some(v) = value.get_rich_text() {
            shared_string_item.set_rich_text(v);
        }

        let hash_code = shared_string_item.get_hash_u64();
        let n = match self.map.get(&hash_code) {
            Some(v) => v.to_owned(),
            None => {
                let n = self.shared_string_item.len();
                self.map.insert(hash_code, n);
                self.set_shared_string_item(shared_string_item);
                n
            }
        };
        n
    }

    pub(crate) fn reflash_map(&mut self) {
        self.map.clear();
        for i in 0..self.shared_string_item.len() {
            let obj = self.shared_string_item.get(i).unwrap();
            let hash_code = obj.get_hash_u64();
            match self.map.get(&hash_code) {
                Some(v) => {}
                None => {
                    self.map.insert(hash_code, i);
                }
            };
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        theme: &Theme,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"si" {
                    let mut obj = SharedStringItem::default();
                    obj.set_attributes(reader, e);

                    // set ThemeColor
                    if let Some(v) = obj.get_rich_text_mut() {
                        for element in v.get_rich_text_elements_mut() {
                            if let Some(r) = element.get_run_properties_crate() {
                                let color = r.get_color_mut();
                                color.set_argb_by_theme(&theme);
                            }
                        }
                    }

                    self.set_shared_string_item(obj);
                    self.reflash_map();
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"sst" {
                    return
                }
            },
            Event::Eof => panic!("Error not find {} end element", "sst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sst
        write_start_tag(
            writer,
            "sst",
            vec![
                (
                    "xmlns",
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                ),
                ("count", self.regist_count.to_string().as_str()),
                (
                    "uniqueCount",
                    self.shared_string_item.len().to_string().as_str(),
                ),
            ],
            false,
        );

        // si
        for obj in &self.shared_string_item {
            obj.write_to(writer);
        }

        write_end_tag(writer, "sst");
    }
}
