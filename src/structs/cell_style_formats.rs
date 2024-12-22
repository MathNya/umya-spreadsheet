// cellStyleXfs
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::CellFormat;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub(crate) struct CellStyleFormats {
    cell_format: Vec<CellFormat>,
}

impl CellStyleFormats {
    #[inline]
    pub(crate) fn get_cell_format(&self) -> &[CellFormat] {
        &self.cell_format
    }

    #[inline]
    pub(crate) fn get_cell_format_mut(&mut self) -> &mut Vec<CellFormat> {
        &mut self.cell_format
    }

    #[inline]
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
                if e.name().into_inner() == b"cellStyleXfs" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cellStyleXfs")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.cell_format.is_empty() {
            // cellStyleXfs
            write_start_tag(
                writer,
                "cellStyleXfs",
                vec![("count", &self.cell_format.len().to_string())],
                false,
            );

            // xf
            for cell_format in &self.cell_format {
                cell_format.write_to(writer, false);
            }

            write_end_tag(writer, "cellStyleXfs");
        }
    }
}
