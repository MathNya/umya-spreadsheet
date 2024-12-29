// dataFields
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::xml_read_loop,
    structs::DataField,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct DataFields {
    list: Vec<DataField>,
}
impl DataFields {
    #[must_use]
    pub fn get_list(&self) -> &[DataField] {
        &self.list
    }

    pub fn get_list_mut(&mut self) -> &mut Vec<DataField> {
        &mut self.list
    }

    pub fn add_list_mut(&mut self, value: DataField) -> &mut Self {
        self.list.push(value);
        self
    }

    #[allow(unused_variables)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"dataField" {
                    let mut obj = DataField::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dataFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "dataFields")
        );
    }

    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataFields
        write_start_tag(
            writer,
            "dataFields",
            vec![("count", self.list.len().to_string()).into()],
            false,
        );

        // dataField
        for sheet_view in &self.list {
            sheet_view.write_to(writer);
        }

        write_end_tag(writer, "dataFields");
    }
}
