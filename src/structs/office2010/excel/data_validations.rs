// x14:dataValidations
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::office2010::excel::DataValidation;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct DataValidations {
    data_validation_list: Vec<DataValidation>,
}

impl DataValidations {
    pub fn get_data_validation_list(&self) -> &Vec<DataValidation> {
        &self.data_validation_list
    }

    pub fn get_data_validation_list_mut(&mut self) -> &mut Vec<DataValidation> {
        &mut self.data_validation_list
    }

    pub fn set_data_validation_list(&mut self, value: Vec<DataValidation>) -> &mut Self {
        self.data_validation_list = value;
        self
    }

    pub fn add_data_validation_list(&mut self, value: DataValidation) -> &mut Self {
        self.data_validation_list.push(value);
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
                if e.name().into_inner() == b"x14:dataValidation" {
                    let mut obj = DataValidation::default();
                    obj.set_attributes(reader, e, true);
                    self.add_data_validation_list(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"x14:dataValidation" {
                    let mut obj = DataValidation::default();
                    obj.set_attributes(reader, e, false);
                    self.add_data_validation_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"x14:dataValidations" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x14:dataValidations")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // ext
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("uri", "{CCE6A557-97BC-4b89-ADB6-D9C93CAAB3DF}"));
        attributes.push(("xmlns:x14", SHEET_MS_MAIN_NS));
        write_start_tag(writer, "ext", attributes, false);

        // dataValidations
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let count = self.data_validation_list.len().to_string();
        attributes.push(("count", &count));
        attributes.push(("xmlns:xm", EXCEL_MAIN_NS));

        write_start_tag(writer, "x14:dataValidations", attributes, false);

        for obj in &self.data_validation_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, "x14:dataValidations");
        write_end_tag(writer, "ext");
    }
}
