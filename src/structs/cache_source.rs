// cacheSource
use crate::structs::EnumValue;
use crate::structs::SourceValues;
use crate::structs::WorksheetSource;

use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CacheSource {
    r#type: EnumValue<SourceValues>,
    worksheet_source: Option<WorksheetSource>,
}

impl CacheSource {
    pub fn get_type(&self) -> &SourceValues {
        self.r#type.get_value()
    }

    pub fn set_type(&mut self, value: SourceValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    pub fn get_worksheet_source(&self) -> Option<&WorksheetSource> {
        self.worksheet_source.as_ref()
    }

    pub fn get_worksheet_source_mut(&mut self) -> Option<&mut WorksheetSource> {
        self.worksheet_source.as_mut()
    }

    pub fn set_worksheet_source_mut(&mut self, value: WorksheetSource) -> &mut Self {
        self.worksheet_source = Some(value);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, r#type, "type");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"worksheetSource" {
                    let mut obj = WorksheetSource::default();
                    obj.set_attributes(reader, e);
                    self.set_worksheet_source_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cacheSource" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cacheSource")
        );
    }

    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cacheSource
        let empty_flg = self.worksheet_source.is_none();
        let attributes = vec![("type", self.r#type.get_hash_string())];

        write_start_tag(writer, "cacheSource", attributes, empty_flg);

        if !empty_flg {
            // worksheetSource
            if let Some(v) = &self.worksheet_source {
                v.write_to(writer)
            }
            write_end_tag(writer, "cacheSource");
        }
    }
}
