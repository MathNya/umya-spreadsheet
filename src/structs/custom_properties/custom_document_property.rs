use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::borrow::Cow;
use std::io::Cursor;
use structs::custom_properties::CustomDocumentPropertyValue;
use structs::StringValue;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct CustomDocumentProperty {
    name: StringValue,
    link_target: StringValue,
    custom_document_property_value: CustomDocumentPropertyValue,
}

impl CustomDocumentProperty {
    pub fn get_name(&self) -> &str {
        self.name.get_value_str()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_link_target(&self) -> &str {
        self.link_target.get_value_str()
    }

    pub fn set_link_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.link_target.set_value(value);
        self
    }

    pub fn get_value(&self) -> Cow<'static, str> {
        self.custom_document_property_value.to_string().into()
    }

    pub fn get_value_number(&self) -> Option<i32> {
        self.custom_document_property_value.get_number()
    }

    pub fn get_value_bool(&self) -> Option<bool> {
        self.custom_document_property_value.get_bool()
    }

    pub fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.custom_document_property_value = CustomDocumentPropertyValue::String(value.into());
        self
    }

    pub fn set_value_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<i32>,
    {
        self.custom_document_property_value = CustomDocumentPropertyValue::Numeric(value.into());
        self
    }

    pub fn set_value_date(&mut self, year: i32, month: i32, day: i32) -> &mut Self {
        let value = format!("{:>04}-{:>02}-{:>02}T10:00:00Z", year, month, day);
        self.custom_document_property_value = CustomDocumentPropertyValue::Date(value);
        self
    }

    pub fn set_value_date_manual<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.custom_document_property_value = CustomDocumentPropertyValue::Date(value.into());
        self
    }

    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.custom_document_property_value = CustomDocumentPropertyValue::Bool(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, link_target, "linkTarget");

        if empty_flag {
            return;
        }

        let mut value: String = String::from("");
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                value = e.unescape().unwrap().to_string();
            },
            Event::End(ref e) => {
                match e.name().into_inner(){
                    b"vt:lpwstr" =>{self.set_value_string(&value);}
                    b"vt:filetime" =>{self.set_value_date_manual(&value);}
                    b"vt:i4"=> {self.set_value_number(value.parse::<i32>().unwrap());}
                    b"vt:bool"=> {self.set_value_bool(matches!(value.as_str(), "true" | "1"));}
                    b"property"=> {return}
                    _=>{}
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "property")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, pid: &i32) {
        let is_inner = self.custom_document_property_value.get_tag().is_some();

        // property
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        attributes.push(("fmtid", r#"{D5CDD505-2E9C-101B-9397-08002B2CF9AE}"#));

        let pid_str = pid.to_string();
        attributes.push(("pid", &pid_str));

        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_str()));
        }

        if self.link_target.has_value() {
            attributes.push(("linkTarget", self.link_target.get_value_str()));
        }

        write_start_tag(writer, "property", attributes, !is_inner);

        if is_inner {
            let tag = self.custom_document_property_value.get_tag().unwrap();
            let value_str = self.custom_document_property_value.to_string();
            write_start_tag(writer, tag, vec![], !is_inner);
            write_text_node(writer, &value_str);
            write_end_tag(writer, tag);

            write_end_tag(writer, "property");
        }
    }
}
