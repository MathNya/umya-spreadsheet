// dataValidations
use super::DataValidation;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"dataValidation" => {
                        let mut obj = DataValidation::default();
                        obj.set_attributes(reader, e, true);
                        self.add_data_validation_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"dataValidation" => {
                        let mut obj = DataValidation::default();
                        obj.set_attributes(reader, e, false);
                        self.add_data_validation_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"dataValidations" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "dataValidations"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataValidations
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let count = self.data_validation_list.len().to_string();
        attributes.push(("count", &count));

        write_start_tag(writer, "dataValidations", attributes, false);

        for obj in &self.data_validation_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, "dataValidations");
    }
}
