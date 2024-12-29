// sst
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
    CellValue,
    SharedStringItem,
};
use crate::{
    helper::const_str::SHEET_MAIN_NS,
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct SharedStringTable {
    shared_string_item: Vec<SharedStringItem>,
    map:                HashMap<u64, usize>,
    regist_count:       usize,
}

impl SharedStringTable {
    pub(crate) fn get_shared_string_item(&self) -> &[SharedStringItem] {
        &self.shared_string_item
    }

    #[allow(dead_code)]
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
        let n = if let Some(v) = self.map.get(&hash_code) {
            v.to_owned()
        } else {
            let n = self.shared_string_item.len();
            self.map.insert(hash_code, n);
            self.set_shared_string_item(shared_string_item);
            n
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
                ("xmlns", SHEET_MAIN_NS).into(),
                ("count", self.regist_count.to_string()).into(),
                (
                    "uniqueCount",
                    self.shared_string_item.len().to_string().as_str(),
                )
                    .into(),
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
