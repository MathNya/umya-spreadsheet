// c:numCache
use super::FormatCode;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Address;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NumberingCache {
    format_code: FormatCode,
}
impl NumberingCache {
    pub fn get_format_code(&self) -> &FormatCode {
        &self.format_code
    }

    pub fn get_format_code_mut(&mut self) -> &mut FormatCode {
        &mut self.format_code
    }

    pub fn set_format_code(&mut self, value: FormatCode) -> &mut NumberingCache {
        self.format_code = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
                    b"c:formatCode" => {
                        self.format_code.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:numCache" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:numCache"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        address: &Address,
        spreadsheet: &Spreadsheet,
    ) {
        let cell_value_list = spreadsheet.get_cell_value_by_address_crate(address);
        let coll_value_count = cell_value_list.len().to_string();
        // c:numCache
        write_start_tag(writer, "c:numCache", vec![], false);

        // c:formatCode
        self.format_code.write_to(writer);

        // c:ptCount
        write_start_tag(
            writer,
            "c:ptCount",
            vec![("val", coll_value_count.as_str())],
            true,
        );

        for (idx, cell_value) in cell_value_list.into_iter().enumerate() {
            // c:pt
            write_start_tag(
                writer,
                "c:pt",
                vec![("idx", idx.to_string().as_str())],
                false,
            );

            // c:v
            write_start_tag(writer, "c:v", vec![], false);
            write_text_node(writer, cell_value.get_value());
            write_end_tag(writer, "c:v");

            write_end_tag(writer, "c:pt");
        }

        write_end_tag(writer, "c:numCache");
    }
}
