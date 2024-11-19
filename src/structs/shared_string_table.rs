// sst
use super::drawing::Theme;
use super::CellValue;
use super::SharedStringItem;
use drawing::charts::View3D;
use hashbrown::HashMap;
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use thin_vec::ThinVec;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct SharedStringTable {
    shared_string_item: ThinVec<SharedStringItem>,
    map: HashMap<u64, usize>,
    regist_count: usize,
}

impl SharedStringTable {
    #[inline]
    pub(crate) fn get_shared_string_item(&self) -> &[SharedStringItem] {
        &self.shared_string_item
    }

    #[inline]
    pub(crate) fn get_shared_string_item_mut(&mut self) -> &mut ThinVec<SharedStringItem> {
        &mut self.shared_string_item
    }

    #[inline]
    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        self.shared_string_item.push(value);
        self
    }

    #[inline]
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

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut n: usize = 0;
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"si" {
                    let mut shared_string_item = SharedStringItem::default();
                    shared_string_item.set_attributes(reader, e);

                    let hash_code = shared_string_item.get_hash_u64();
                    self.map.insert(hash_code, n);
                    self.set_shared_string_item(shared_string_item);

                    n += 1;
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"sst" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "sst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sst
        write_start_tag(
            writer,
            "sst",
            vec![
                ("xmlns", SHEET_MAIN_NS),
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
