// dataValidation
use std::{
    io::Cursor,
    vec,
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    BooleanValue,
    DataValidationOperatorValues,
    DataValidationValues,
    EnumValue,
    SequenceOfReferences,
    StringValue,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Default, Debug, Clone)]
pub struct DataValidation {
    r#type:                 EnumValue<DataValidationValues>,
    operator:               EnumValue<DataValidationOperatorValues>,
    allow_blank:            BooleanValue,
    show_input_message:     BooleanValue,
    show_error_message:     BooleanValue,
    prompt_title:           StringValue,
    prompt:                 StringValue,
    error_title:            StringValue,
    error_messsage:         StringValue,
    sequence_of_references: SequenceOfReferences,
    formula1:               StringValue,
    formula2:               StringValue,
}
impl DataValidation {
    #[inline]
    #[must_use]
    pub fn get_type(&self) -> &DataValidationValues {
        self.r#type.value()
    }

    #[inline]
    pub fn set_type(&mut self, value: DataValidationValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn operator(&self) -> &DataValidationOperatorValues {
        self.operator.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use operator()")]
    pub fn get_operator(&self) -> &DataValidationOperatorValues {
        self.operator()
    }

    #[inline]
    pub fn set_operator(&mut self, value: DataValidationOperatorValues) -> &mut Self {
        self.operator.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn allow_blank(&self) -> bool {
        self.allow_blank.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use allow_blank()")]
    pub fn get_allow_blank(&self) -> bool {
        self.allow_blank()
    }

    #[inline]
    pub fn set_allow_blank(&mut self, value: bool) -> &mut Self {
        self.allow_blank.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn show_input_message(&self) -> bool {
        self.show_input_message.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use show_input_message()")]
    pub fn get_show_input_message(&self) -> bool {
        self.show_input_message()
    }

    #[inline]
    pub fn set_show_input_message(&mut self, value: bool) -> &mut Self {
        self.show_input_message.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn show_error_message(&self) -> bool {
        self.show_error_message.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use show_error_message()")]
    pub fn get_show_error_message(&self) -> bool {
        self.show_error_message()
    }

    #[inline]
    pub fn set_show_error_message(&mut self, value: bool) -> &mut Self {
        self.show_error_message.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn prompt_title(&self) -> &str {
        self.prompt_title.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use prompt_title()")]
    pub fn get_prompt_title(&self) -> &str {
        self.prompt_title()
    }

    #[inline]
    pub fn set_prompt_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt_title.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn error_title(&self) -> &str {
        self.error_title.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use error_title()")]
    pub fn get_error_title(&self) -> &str {
        self.error_title()
    }

    #[inline]
    pub fn set_error_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.error_title.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn error_message(&self) -> &str {
        self.error_messsage.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use error_messsage()")]
    pub fn get_error_message(&self) -> &str {
        self.error_message()
    }

    #[inline]
    pub fn set_error_message<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.error_messsage.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn prompt(&self) -> &str {
        self.prompt.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use prompt()")]
    pub fn get_prompt(&self) -> &str {
        self.prompt()
    }

    #[inline]
    pub fn set_prompt<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prompt.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sequence_of_references()")]
    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        self.sequence_of_references()
    }

    #[inline]
    pub fn sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sequence_of_references_mut()")]
    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        self.sequence_of_references_mut()
    }

    #[inline]
    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) -> &mut Self {
        self.sequence_of_references = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn formula1(&self) -> &str {
        self.formula1.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use formula1()")]
    pub fn get_formula1(&self) -> &str {
        self.formula1()
    }

    #[inline]
    pub fn set_formula1<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.formula1.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn formula2(&self) -> &str {
        self.formula2.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use formula2()")]
    pub fn get_formula2(&self) -> &str {
        self.formula2()
    }

    #[inline]
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

        if let Some(v) = get_attribute(e, b"errorTitle") {
            self.error_title.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"error") {
            self.error_messsage.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"promptTitle") {
            self.prompt_title.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"prompt") {
            self.prompt.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"sqref") {
            self.sequence_of_references.set_sqref(v);
        }

        if empty_flg {
            return;
        }

        let mut value: String = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    value = e.unescape().unwrap().to_string();
                }
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"formula1" => {
                        self.formula1.set_value_string(std::mem::take(&mut value));
                    }
                    b"formula2" => {
                        self.formula2.set_value_string(std::mem::take(&mut value));
                    }
                    b"dataValidation" => return,
                    _ => {}
                },
                Ok(Event::Eof) => {
                    panic!("Error: Could not find {} end element", "dataValidation")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => {}
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let is_inner = self.formula1.has_value() || self.formula2.has_value();

        // dataValidation
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        if self.r#type.has_value() {
            attributes.push(("type", self.r#type.value_string()).into());
        }

        if self.allow_blank.has_value() {
            attributes.push(("allowBlank", self.allow_blank.value_string()).into());
        }

        if self.show_input_message.has_value() {
            attributes.push(
                (
                    "showInputMessage",
                    self.show_input_message.value_string(),
                )
                    .into(),
            );
        }

        if self.operator.has_value() {
            attributes.push(("operator", self.operator.value_string()).into());
        }

        if self.show_error_message.has_value() {
            attributes.push(
                (
                    "showErrorMessage",
                    self.show_error_message.value_string(),
                )
                    .into(),
            );
        }

        if self.error_title.has_value() {
            attributes.push(("errorTitle", self.error_title.value_str()).into());
        }

        if self.error_messsage.has_value() {
            attributes.push(("error", self.error_messsage.value_str()).into());
        }

        if self.prompt_title.has_value() {
            attributes.push(("promptTitle", self.prompt_title.value_str()).into());
        }

        if self.prompt.has_value() {
            attributes.push(("prompt", self.prompt.value_str()).into());
        }

        let sequence_of_references = &self.sequence_of_references.get_sqref();
        if !sequence_of_references.is_empty() {
            attributes.push(("sqref", sequence_of_references).into());
        }

        write_start_tag(writer, "dataValidation", attributes, !is_inner);
        if is_inner {
            if self.formula1.has_value() {
                write_start_tag(writer, "formula1", vec![], false);
                write_text_node(writer, self.formula1.value_str());
                write_end_tag(writer, "formula1");
            }
            if self.formula2.has_value() {
                write_start_tag(writer, "formula2", vec![], false);
                write_text_node(writer, self.formula2.value_str());
                write_end_tag(writer, "formula2");
            }
            write_end_tag(writer, "dataValidation");
        }
    }
}
