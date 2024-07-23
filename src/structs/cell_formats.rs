// cellXfs
use super::CellFormat;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct CellFormats {
    cell_format: Vec<CellFormat>,
}

impl CellFormats {
    pub(crate) fn get_cell_format(&self) -> &Vec<CellFormat> {
        &self.cell_format
    }

    pub(crate) fn _get_cell_format_mut(&mut self) -> &mut Vec<CellFormat> {
        &mut self.cell_format
    }

    pub(crate) fn set_cell_format(&mut self, value: CellFormat) -> &mut Self {
        self.cell_format.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"xf" {
                    let mut obj = CellFormat::default();
                    obj.set_attributes(reader, e, true);
                    self.set_cell_format(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"xf" {
                    let mut obj = CellFormat::default();
                    obj.set_attributes(reader, e, false);
                    self.set_cell_format(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cellXfs" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cellXfs")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.cell_format.is_empty() {
            // cellXfs
            write_start_tag(
                writer,
                "cellXfs",
                vec![("count", &self.cell_format.len().to_string())],
                false,
            );

            // xf
            for cell_format in &self.cell_format {
                cell_format.write_to(writer, true);
            }

            write_end_tag(writer, "cellXfs");
        }
    }
}
