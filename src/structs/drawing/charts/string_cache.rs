use crate::drawing::charts::Formula;
// c:strCache
use crate::reader::driver::*;
use crate::structs::Address;
use crate::structs::Spreadsheet;
use crate::writer::driver::*;
use crate::CellValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct StringCache {}

impl StringCache {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().0 == b"c:strCache" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:strCache")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        formula: &Formula,
        spreadsheet: &Spreadsheet,
    ) {
        let mut cell = CellValue::default();
        let cell_value_list = match formula.has_string_value() {
            true => {
                cell.set_value(formula.get_address_str());
                vec![&cell]
            }
            false => spreadsheet.get_cell_value_by_address_crate(formula.get_address()),
        };
        let coll_value_count = cell_value_list.len().to_string();
        // c:strCache
        write_start_tag(writer, "c:strCache", vec![], false);

        // c:ptCount
        write_start_tag(writer, "c:ptCount", vec![("val", &coll_value_count)], true);

        for (idx, cell_value) in cell_value_list.into_iter().enumerate() {
            // c:pt
            write_start_tag(writer, "c:pt", vec![("idx", &idx.to_string())], false);

            // c:v
            write_start_tag(writer, "c:v", vec![], false);
            write_text_node(writer, cell_value.get_value());
            write_end_tag(writer, "c:v");

            write_end_tag(writer, "c:pt");
        }

        write_end_tag(writer, "c:strCache");
    }
}
