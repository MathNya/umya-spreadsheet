// dataValidation
use std::io::Cursor;

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
    DataValidationErrorStyleValues,
    DataValidationType,
    EnumValue,
    SequenceOfReferences,
    StringValue,
};
use crate::{
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    traits::AdjustmentCoordinate,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Default, Debug, Clone)]
pub struct DataValidation {
    r#type:                 DataValidationType,
    allow_blank:            BooleanValue,
    show_input_message:     BooleanValue,
    show_error_message:     BooleanValue,
    /// A boolean value indicating whether to display a dropdown combo box for a
    /// list type data validation. Note that it has an inverted logic, false
    /// shows the dropdown list and true will render the dropdown as plain text
    /// instead. See `<https://openpyxl.readthedocs.io/en/stable/validation.html>`
    show_drop_down:         BooleanValue,
    prompt_title:           StringValue,
    prompt:                 StringValue,
    error_title:            StringValue,
    error_message:          StringValue,
    error_style:            EnumValue<DataValidationErrorStyleValues>,
    sequence_of_references: SequenceOfReferences,
}
impl DataValidation {
    #[inline]
    #[must_use]
    pub fn get_type(&self) -> &DataValidationType {
        &self.r#type
    }

    #[inline]
    pub fn set_type(&mut self, value: DataValidationType) -> &mut Self {
        self.r#type = value;
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
        self.error_message.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use error_message()")]
    pub fn get_error_message(&self) -> &str {
        self.error_message()
    }

    #[inline]
    pub fn set_error_message<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.error_message.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn error_style(&self) -> &DataValidationErrorStyleValues {
        self.error_style.value()
    }

    #[inline]
    pub fn set_error_style(&mut self, value: DataValidationErrorStyleValues) -> &mut Self {
        self.error_style.set_value(value);
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

    /// A boolean value indicating whether to display a dropdown combo box for a
    /// list type data validation. Note that it has an inverted logic, false
    /// shows the dropdown list and true will render the dropdown as plain text
    /// instead. See `<https://openpyxl.readthedocs.io/en/stable/validation.html>`
    #[inline]
    #[must_use]
    pub fn show_drop_down(&self) -> bool {
        self.show_drop_down.value()
    }

    #[inline]
    pub fn set_show_drop_down(&mut self, value: bool) -> &mut Self {
        self.show_drop_down.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        let r#type = get_attribute(e, b"type");
        let operator = get_attribute(e, b"operator");

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
            self.error_message.set_value_string(v);
        }

        if let Some(v) = get_attribute(e, b"errorStyle") {
            self.error_style.set_value_string(v);
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

        if let Some(v) = get_attribute(e, b"showDropDown") {
            self.show_drop_down.set_value_string(v);
        }

        let type_parser =
            DataValidationType::with_type_and_operator(r#type.as_deref(), operator.as_deref());

        if empty_flg {
            self.r#type = type_parser
                .apply(None, None)
                .unwrap_or_else(|err| panic!("Data validation error: {err}"));
            return;
        }

        let mut formula1 = None;
        let mut formula2 = None;

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"formula1" => {
                        let mut buf = Vec::new();
                        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
                        formula1 = Some(crate::helper::utils::unescape_xml_text(&text));
                    }
                    b"formula2" => {
                        let mut buf = Vec::new();
                        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
                        formula2 = Some(crate::helper::utils::unescape_xml_text(&text));
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dataValidation" {
                    let t = type_parser.apply(formula1, formula2).unwrap_or_else(|err| panic!("Data validation error: {err}"));
                    self.r#type = t;
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "dataValidation")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataValidation
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        let type_writer = self.r#type.attrs_and_formulas();
        let has_formulas = type_writer.has_formulas();

        attributes.push(("type", type_writer.r#type).into());

        if self.allow_blank.has_value() {
            attributes.push(("allowBlank", self.allow_blank.value_string()).into());
        }

        if self.show_input_message.has_value() {
            attributes.push(("showInputMessage", self.show_input_message.value_string()).into());
        }

        if let Some(v) = type_writer.operator {
            attributes.push(("operator", v).into());
        }

        if self.show_error_message.has_value() {
            attributes.push(("showErrorMessage", self.show_error_message.value_string()).into());
        }

        if self.error_title.has_value() {
            attributes.push(("errorTitle", self.error_title.value_str()).into());
        }

        if self.error_message.has_value() {
            attributes.push(("error", self.error_message.value_str()).into());
        }

        if self.error_style.has_value() {
            attributes.push(("errorStyle", self.error_style.value_string()).into());
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

        if self.show_drop_down.has_value() {
            attributes.push(("showDropDown", self.show_drop_down.value_string()).into());
        }

        write_start_tag(writer, "dataValidation", attributes, !has_formulas);
        if has_formulas {
            // formula1
            if let Some(formula1) = type_writer.formula1 {
                if !formula1.is_empty() {
                    write_start_tag(writer, "formula1", vec![], false);
                    write_text_node(writer, formula1);
                    write_end_tag(writer, "formula1");
                }
            }
            // formula2
            if let Some(formula2) = type_writer.formula2 {
                if !formula2.is_empty() {
                    write_start_tag(writer, "formula2", vec![], false);
                    write_text_node(writer, formula2);
                    write_end_tag(writer, "formula2");
                }
            }
            write_end_tag(writer, "dataValidation");
        }
    }
}

impl AdjustmentCoordinate for DataValidation {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.sequence_of_references.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.sequence_of_references.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        self.sequence_of_references.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::Reader;

    use super::*;
    use crate::{
        DataValidationExpression,
        DataValidationListOrRange,
    };

    #[test]
    fn formula1_preserves_numeric_character_references() {
        // Excel/LibreOffice may emit en-dashes as `&#8211;`. quick-xml surfaces
        // those as GeneralRef events between Text fragments; we must append
        // both.
        let xml = concat!(
            r#"<dataValidation type="list" sqref="D41" allowBlank="1">"#,
            r#"<formula1>"0&#8211;3 years ago,4&#8211;5 years ago,6&#8211;7 years ago"</formula1>"#,
            r#"</dataValidation>"#,
        );
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let Event::Start(e) = reader.read_event_into(&mut buf).unwrap() else {
            panic!("expected Start dataValidation");
        };
        let mut dv = DataValidation::default();
        dv.set_attributes(&mut reader, &e, false);
        assert_eq!(dv.sequence_of_references().get_sqref(), "D41");
        assert_eq!(
            dv.get_type(),
            &DataValidationType::List(DataValidationListOrRange::List(vec![
                "0–3 years ago".into(),
                "4–5 years ago".into(),
                "6–7 years ago".into()
            ]))
        );
    }

    #[test]
    fn formula1_plain_text_still_works() {
        let xml = concat!(
            r#"<dataValidation type="list" sqref="C5" allowBlank="1">"#,
            r#"<formula1>"petrol,diesel,LPG,electric"</formula1>"#,
            r#"</dataValidation>"#,
        );
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let Event::Start(e) = reader.read_event_into(&mut buf).unwrap() else {
            panic!("expected Start dataValidation");
        };
        let mut dv = DataValidation::default();
        dv.set_attributes(&mut reader, &e, false);
        assert_eq!(
            dv.get_type(),
            &DataValidationType::List(DataValidationListOrRange::List(vec![
                "petrol".into(),
                "diesel".into(),
                "LPG".into(),
                "electric".into()
            ]))
        );
    }

    #[test]
    fn write_keeps_both_formulas() {
        let mut dv = DataValidation::default();
        dv.set_type(DataValidationType::Whole(
            DataValidationExpression::Between("1".into(), "10".into()),
        ));
        dv.sequence_of_references_mut().set_sqref("A1");

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        dv.write_to(&mut writer);
        let xml = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        assert!(
            xml.contains("<formula1>1</formula1><formula2>10</formula2>"),
            "{xml}"
        );
    }
}
