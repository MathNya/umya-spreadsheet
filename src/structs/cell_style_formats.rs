// cellStyleXfs
use super::CellFormat;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct CellStyleFormats {
    cell_format: Vec<CellFormat>,
}
impl CellStyleFormats {
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"xf" => {
                        let mut obj = CellFormat::default();
                        obj.set_attributes(reader, e, true);
                        self.set_cell_format(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"xf" => {
                        let mut obj = CellFormat::default();
                        obj.set_attributes(reader, e, false);
                        self.set_cell_format(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"cellStyleXfs" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "cellStyleXfs"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
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
