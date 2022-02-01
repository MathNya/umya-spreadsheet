// cellXfs
use super::CellFormat;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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

    pub(crate) fn init_setup(&mut self) -> &mut Self {
        let obj = CellFormat::get_defalut_value();
        self.set_cell_format(obj);
        self
    }

    pub(crate) fn set_cell_format_crate(&mut self, value: CellFormat) -> u32 {
        let hash_code = value.get_hash_code();
        let mut id = 0;
        for cell_format in &self.cell_format {
            if cell_format.get_hash_code() == hash_code {
                return id;
            }
            id += 1;
        }
        self.set_cell_format(value);
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
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"xf" => {
                        let mut obj = CellFormat::default();
                        obj.set_attributes(reader, e, true);
                        self.set_cell_format(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name() {
                    b"xf" => {
                        let mut obj = CellFormat::default();
                        obj.set_attributes(reader, e, false);
                        self.set_cell_format(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"cellXfs" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "cellXfs"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.cell_format.len() > 0 {
            // cellXfs
            write_start_tag(
                writer,
                "cellXfs",
                vec![("count", &self.cell_format.len().to_string())],
                false,
            );

            // xf
            for cell_format in &self.cell_format {
                cell_format.write_to(writer);
            }

            write_end_tag(writer, "cellXfs");
        }
    }
}
