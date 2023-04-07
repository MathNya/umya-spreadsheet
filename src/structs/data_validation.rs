// dataValidation
use super::BooleanValue;
use super::DataValidationValues;
use super::EnumValue;
use super::SequenceOfReferences;
use super::StringValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use std::vec;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct DataValidation {
    r#type: EnumValue<DataValidationValues>,
    allow_blank: BooleanValue,
    show_input_message: BooleanValue,
    show_error_message: BooleanValue,
    prompt_title: StringValue,
    prompt: StringValue,
    sequence_of_references: SequenceOfReferences,
    formula1: StringValue,
    formula2: StringValue,
}
impl DataValidation {
    pub fn get_type(&self) -> &DataValidationValues {
        self.r#type.get_value()
    }

    pub fn set_type(&mut self, value: DataValidationValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    pub fn get_allow_blank(&self) -> &bool {
        self.allow_blank.get_value()
    }

    pub fn set_allow_blank(&mut self, value: bool) -> &mut Self {
        self.allow_blank.set_value(value);
        self
    }

    pub fn get_show_input_message(&self) -> &bool {
        self.show_input_message.get_value()
    }

    pub fn set_show_input_message(&mut self, value: bool) -> &mut Self {
        self.show_input_message.set_value(value);
        self
    }

    pub fn get_show_error_message(&self) -> &bool {
        self.show_error_message.get_value()
    }

    pub fn set_show_error_message(&mut self, value: bool) -> &mut Self {
        self.show_error_message.set_value(value);
        self
    }

    pub fn get_prompt_title(&self) -> &str {
        self.prompt_title.get_value()
    }

    pub fn set_prompt_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt_title.set_value(value);
        self
    }

    pub fn get_prompt(&self) -> &str {
        self.prompt.get_value()
    }

    pub fn set_prompt<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt.set_value(value);
        self
    }

    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) -> &mut Self {
        self.sequence_of_references = value;
        self
    }

    pub fn get_formula1(&self) -> &str {
        self.formula1.get_value()
    }

    pub fn set_formula1<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.formula1.set_value(value);
        self
    }

    pub fn get_formula2(&self) -> &str {
        self.formula2.get_value()
    }

    pub fn set_formula2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.formula2.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        match get_attribute(e, b"type") {
            Some(v) => {
                self.r#type.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"allowBlank") {
            Some(v) => {
                self.allow_blank.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"showInputMessage") {
            Some(v) => {
                self.show_input_message.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"showErrorMessage") {
            Some(v) => {
                self.show_error_message.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"promptTitle") {
            Some(v) => {
                self.prompt_title.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"prompt") {
            Some(v) => {
                self.prompt.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"sqref") {
            Some(v) => {
                self.sequence_of_references.set_sqref(v);
            }
            None => {}
        }

        if empty_flg {
            return;
        }

        let mut value: String = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    value = e.unescape().unwrap().to_string();
                }
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"formula1" => {
                        self.formula1.set_value_string(value);
                        value = String::from("");
                    }
                    b"formula2" => {
                        self.formula2.set_value_string(value);
                        value = String::from("");
                    }
                    b"dataValidation" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "dataValidation"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let is_inner = self.formula1.has_value() || self.formula2.has_value();

        // dataValidation
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        if self.r#type.has_value() {
            attributes.push(("type", self.r#type.get_value_string()));
        }

        if self.allow_blank.has_value() {
            attributes.push(("allowBlank", self.allow_blank.get_value_string()));
        }

        if self.show_input_message.has_value() {
            attributes.push((
                "showInputMessage",
                self.show_input_message.get_value_string(),
            ));
        }

        if self.show_error_message.has_value() {
            attributes.push((
                "showErrorMessage",
                self.show_error_message.get_value_string(),
            ));
        }

        if self.prompt_title.has_value() {
            attributes.push(("promptTitle", self.prompt_title.get_value_string()));
        }

        if self.prompt.has_value() {
            attributes.push(("prompt", self.prompt.get_value_string()));
        }

        let sequence_of_references = &self.sequence_of_references.get_sqref();
        if sequence_of_references != "" {
            attributes.push(("sqref", sequence_of_references));
        }

        write_start_tag(writer, "dataValidation", attributes, !is_inner);
        if is_inner {
            if self.formula1.has_value() {
                write_start_tag(writer, "formula1", vec![], false);
                write_text_node(writer, self.formula1.get_value_string());
                write_end_tag(writer, "formula1");
            }
            if self.formula2.has_value() {
                write_start_tag(writer, "formula2", vec![], false);
                write_text_node(writer, self.formula2.get_value_string());
                write_end_tag(writer, "formula2");
            }
            write_end_tag(writer, "dataValidation");
        }
    }
}
