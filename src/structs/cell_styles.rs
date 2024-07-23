// cellStyles
use super::CellStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CellStyles {
    cell_style: Vec<CellStyle>,
}

impl CellStyles {
    pub fn _get_cell_style(&self) -> &Vec<CellStyle> {
        &self.cell_style
    }

    pub fn _get_cell_style_mut(&mut self) -> &mut Vec<CellStyle> {
        &mut self.cell_style
    }

    pub fn add_cell_style(&mut self, value: CellStyle) -> &mut Self {
        self.cell_style.push(value);
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
                if e.name().into_inner() == b"cellStyle" {
                    let mut obj = CellStyle::default();
                    obj.set_attributes(reader, e);
                    self.add_cell_style(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cellStyles" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cellStyles")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.cell_style.is_empty() {
            // cellStyles
            write_start_tag(
                writer,
                "cellStyles",
                vec![("count", &self.cell_style.len().to_string())],
                false,
            );

            // cellStyle
            for cell_style in &self.cell_style {
                cell_style.write_to(writer);
            }

            write_end_tag(writer, "cellStyles");
        }
    }
}
