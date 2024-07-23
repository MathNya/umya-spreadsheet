// dxfs
use super::DifferentialFormat;
use super::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct DifferentialFormats {
    differential_format: Vec<DifferentialFormat>,
}

impl DifferentialFormats {
    pub(crate) fn _get_differential_format(&self) -> &Vec<DifferentialFormat> {
        &self.differential_format
    }

    pub(crate) fn _get_differential_format_mut(&mut self) -> &mut Vec<DifferentialFormat> {
        &mut self.differential_format
    }

    pub(crate) fn set_differential_format(&mut self, value: DifferentialFormat) -> &mut Self {
        self.differential_format.push(value);
        self
    }

    pub(crate) fn get_style(&self, id: usize) -> Style {
        let differential_format = self.differential_format.get(id).unwrap().clone();
        differential_format.get_style()
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        let mut differential_format = DifferentialFormat::default();
        differential_format.set_style(style);

        let hash_code = differential_format.get_hash_code();
        let mut id = 0;
        for v in &self.differential_format {
            if v.get_hash_code() == hash_code {
                return id;
            }
            id += 1;
        }

        self.set_differential_format(differential_format);
        id
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"dxf" {
                    let mut obj = DifferentialFormat::default();
                    obj.set_attributes(reader, e);
                    self.set_differential_format(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dxfs" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "dxfs")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.differential_format.is_empty() {
            // dxfs
            write_start_tag(
                writer,
                "dxfs",
                vec![("count", &self.differential_format.len().to_string())],
                false,
            );

            // dxf
            for differential_format in &self.differential_format {
                differential_format.write_to(writer);
            }

            write_end_tag(writer, "dxfs");
        }
    }
}
