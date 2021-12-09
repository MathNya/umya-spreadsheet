// sst
use super::CellValue;
use super::SharedStringItem;
use super::Text;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct SharedStringTable {
    shared_string_item: Vec<SharedStringItem>,
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

    pub(crate) fn init_setup(&mut self) -> &mut Self {
        self.shared_string_item.clear();
        self
    }

    pub(crate) fn set_cell(&mut self, value: &CellValue) -> u32 {
        self.regist_count += 1;

        let mut shared_string_item = SharedStringItem::default();
        match value.get_value_crate() {
            Some(v) => {
                let mut text = Text::default();
                text.set_value(v);
                shared_string_item.set_text(text);
            }
            None => {}
        }
        match value.get_rich_text() {
            Some(v) => {
                shared_string_item.set_rich_text(v.clone());
            }
            None => {}
        }

        let hash_code = shared_string_item.get_hash_code();
        let mut id = 0;
        for tar in &self.shared_string_item {
            if tar.get_hash_code() == hash_code {
                return id;
            }
            id += 1;
        }
        self.set_shared_string_item(shared_string_item);
        return id;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"si" => {
                        let mut obj = SharedStringItem::default();
                        obj.set_attributes(reader, e);
                        self.set_shared_string_item(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
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
