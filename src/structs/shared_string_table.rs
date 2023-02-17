// sst
use super::CellValue;
use super::SharedStringItem;
use hashbrown::HashMap;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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

    pub(crate) fn ensure_map(&mut self) -> bool {
        // let l1 = self.shared_string_item.len();
        // let l2 = self.map.len();
        // println!("{}:::{}",l1,l2);
        if !self.shared_string_item.is_empty() && self.map.is_empty() {
            let mut h: HashMap<u64, usize> = HashMap::with_capacity(self.shared_string_item.len());
            for i in 0..self.shared_string_item.len() {
                let hash = self.shared_string_item[i].get_hash_u64();

                match h.raw_entry_mut().from_key_hashed_nocheck(hash, &hash) {
                    hashbrown::hash_map::RawEntryMut::Occupied(mut o) => Some(o.insert(i)),
                    hashbrown::hash_map::RawEntryMut::Vacant(v) => {
                        v.insert(hash, i);
                        None
                    }
                };
            }
            self.map = h;
        }
        true
    }

    pub(crate) fn set_cell(&mut self, value: &CellValue) -> usize {
        self.regist_count += 1;

        let mut shared_string_item = SharedStringItem::default();
        match value.get_text() {
            Some(v) => {
                shared_string_item.set_text(v);
            }
            None => {}
        }
        match value.get_rich_text() {
            Some(v) => {
                shared_string_item.set_rich_text(v);
            }
            None => {}
        }

        let hash_code = shared_string_item.get_hash_u64();
        self.ensure_map();

        let mut is_new = false;
        let n = match self
            .map
            .raw_entry_mut()
            .from_key_hashed_nocheck(hash_code, &hash_code)
        {
            hashbrown::hash_map::RawEntryMut::Occupied(o) => o.get().to_owned(),
            hashbrown::hash_map::RawEntryMut::Vacant(v) => {
                let n = self.shared_string_item.len();
                is_new = true;
                v.insert(hash_code, n);
                n
            }
        };

        if is_new {
            self.set_shared_string_item(shared_string_item);
        }

        n
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
                    b"si" => {
                        let mut obj = SharedStringItem::default();
                        obj.set_attributes(reader, e);
                        self.set_shared_string_item(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"sst" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "sst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
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
