// cellXfs
use super::CellFormat;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub(crate) struct CellFormats {
    cell_format: ThinVec<CellFormat>,
}

impl CellFormats {
    #[inline]
    pub(crate) fn get_cell_format(&self) -> &[CellFormat] {
        &self.cell_format
    }

    #[inline]
    pub(crate) fn _get_cell_format_mut(&mut self) -> &mut ThinVec<CellFormat> {
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
