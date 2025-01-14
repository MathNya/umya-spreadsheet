// x14:dataValidation
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use crate::{
    reader::driver::get_attribute,
    structs::{
        BooleanValue, DataValidationOperatorValues, DataValidationValues, EnumValue, StringValue,
        office::excel::ReferenceSequence,
        office2010::excel::{DataValidationForumla1, DataValidationForumla2},
    },
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Default, Debug, Clone)]
pub struct DataValidation {
    r#type: EnumValue<DataValidationValues>,
    operator: EnumValue<DataValidationOperatorValues>,
    allow_blank: BooleanValue,
    show_input_message: BooleanValue,
    show_error_message: BooleanValue,
    prompt_title: StringValue,
    prompt: StringValue,
    reference_sequence: ReferenceSequence,
    formula1: Option<Box<DataValidationForumla1>>,
    formula2: Option<Box<DataValidationForumla2>>,
}
impl DataValidation {
    #[inline]
    #[must_use]
    pub fn get_type(&self) -> &DataValidationValues {
        self.r#type.get_value()
    }

    #[inline]
    pub fn set_type(&mut self, value: DataValidationValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_operator(&self) -> &DataValidationOperatorValues {
        self.operator.get_value()
    }

    #[inline]
    pub fn set_operator(&mut self, value: DataValidationOperatorValues) -> &mut Self {
        self.operator.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_allow_blank(&self) -> bool {
        self.allow_blank.get_value()
    }

    #[inline]
    pub fn set_allow_blank(&mut self, value: bool) -> &mut Self {
        self.allow_blank.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_input_message(&self) -> bool {
        self.show_input_message.get_value()
    }

    #[inline]
    pub fn set_show_input_message(&mut self, value: bool) -> &mut Self {
        self.show_input_message.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_error_message(&self) -> bool {
        self.show_error_message.get_value()
    }

    #[inline]
    pub fn set_show_error_message(&mut self, value: bool) -> &mut Self {
        self.show_error_message.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_prompt_title(&self) -> &str {
        self.prompt_title.get_value_str()
    }

    #[inline]
    pub fn set_prompt_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt_title.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_prompt(&self) -> &str {
        self.prompt.get_value_str()
    }

    #[inline]
    pub fn set_prompt<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_reference_sequence(&self) -> &ReferenceSequence {
        &self.reference_sequence
    }

    #[inline]
    pub fn get_reference_sequence_mut(&mut self) -> &mut ReferenceSequence {
        &mut self.reference_sequence
    }

    #[inline]
    pub fn set_reference_sequence(&mut self, value: ReferenceSequence) -> &mut Self {
        self.reference_sequence = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_formula1(&self) -> Option<&DataValidationForumla1> {
        self.formula1.as_deref()
    }

    #[inline]
    pub fn get_formula1_mut(&mut self) -> Option<&mut DataValidationForumla1> {
        self.formula1.as_deref_mut()
    }

    #[inline]
    pub fn set_formula1(&mut self, value: DataValidationForumla1) -> &mut Self {
        self.formula1 = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_formula1(&mut self) -> &mut Self {
        self.formula1 = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_formula2(&self) -> Option<&DataValidationForumla2> {
        self.formula2.as_deref()
    }

    #[inline]
    pub fn get_formula2_mut(&mut self) -> Option<&mut DataValidationForumla2> {
        self.formula2.as_deref_mut()
    }

    #[inline]
    pub fn set_formula2(&mut self, value: DataValidationForumla2) -> &mut Self {
        self.formula2 = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_formula2(&mut self) -> &mut Self {
        self.formula2 = None;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        if let Some(v) = get_attribute(e, b"type") {
            self.r#type.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"operator") {
            self.operator.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"allowBlank") {
            self.allow_blank.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"showInputMessage") {
            self.show_input_message.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"showErrorMessage") {
            self.show_error_message.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"promptTitle") {
            self.prompt_title.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"prompt") {
            self.prompt.set_value_string(v);
        }

        if empty_flg {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"x14:formula1" => {
                        let mut obj = DataValidationForumla1::default();
                        obj.set_attributes(reader, e);
                        self.formula1 = Some(Box::new(obj));
                    }
                    b"x14:formula2" => {
                        let mut obj = DataValidationForumla2::default();
                        obj.set_attributes(reader, e);
                        self.formula2 = Some(Box::new(obj));
                    }
                    b"xm:sqref" => {
                        let mut obj = ReferenceSequence::default();
                        obj.set_attributes(reader, e);
                        self.reference_sequence = obj;
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => {
                    if e.name().into_inner() == b"x14:dataValidation" {
                        return;
                    }
                }
                Ok(Event::Eof) => {
                    panic!("Error: Could not find {} end element", "x14:dataValidation")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x14:dataValidation
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        if self.r#type.has_value() {
            attributes.push(("type", self.r#type.get_value_string()).into());
        }

        if self.allow_blank.has_value() {
            attributes.push(("allowBlank", self.allow_blank.get_value_string()).into());
        }

        if self.show_input_message.has_value() {
            attributes.push(
                (
                    "showInputMessage",
                    self.show_input_message.get_value_string(),
                )
                    .into(),
            );
        }

        if self.operator.has_value() {
            attributes.push(("operator", self.operator.get_value_string()).into());
        }

        if self.show_error_message.has_value() {
            attributes.push(
                (
                    "showErrorMessage",
                    self.show_error_message.get_value_string(),
                )
                    .into(),
            );
        }

        if self.prompt_title.has_value() {
            attributes.push(("promptTitle", self.prompt_title.get_value_str()).into());
        }

        if self.prompt.has_value() {
            attributes.push(("prompt", self.prompt.get_value_str()).into());
        }

        write_start_tag(writer, "x14:dataValidation", attributes, false);
        if let Some(v) = &self.formula1 {
            v.write_to(writer);
        }
        if let Some(v) = &self.formula2 {
            v.write_to(writer);
        }
        self.reference_sequence.write_to(writer);
        write_end_tag(writer, "x14:dataValidation");
    }
}
