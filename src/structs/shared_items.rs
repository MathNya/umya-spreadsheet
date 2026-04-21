// sharedItems
use std::{fmt, str::FromStr};
use std::io::Cursor;
use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::writer::driver::write_end_tag;
use crate::{
    CellErrorType, reader::driver::{
        get_attribute,
        set_string_from_xml,
    }, structs::{
        BooleanValue,
        DoubleValue,
    }, writer::driver::write_start_tag, xml_read_loop
};

#[derive(Clone, Default, Debug)]
pub struct SharedItems {
    contains_semi_mixed_types: BooleanValue,
    contains_string:           BooleanValue,
    contains_number:           BooleanValue,
    contains_integer:          BooleanValue,
    min_value:                 DoubleValue,
    max_value:                 DoubleValue,
    items:                     Vec<SharedItemValue>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub enum SharedItemValue {
    Bool(bool),
    Date(Box<str>),
    Error(CellErrorType),
    #[default]
    Empty,
    Numeric(f64),
    String(Box<str>),
}
impl fmt::Display for SharedItemValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            Self::Date(v) | Self::String(v) => write!(f, "{v}"),
            Self::Error(e) => write!(f, "{e}"),
            Self::Empty => write!(f, ""),
            Self::Numeric(v) => write!(f, "{}", &v),
        }
    }
}

impl SharedItems {
    #[must_use]
    pub fn contains_semi_mixed_types(&self) -> bool {
        self.contains_semi_mixed_types.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use contains_semi_mixed_types()")]
    pub fn get_contains_semi_mixed_types(&self) -> bool {
        self.contains_semi_mixed_types()
    }

    pub fn set_contains_semi_mixed_types(&mut self, value: bool) -> &mut Self {
        self.contains_semi_mixed_types.set_value(value);
        self
    }

    #[must_use]
    pub fn contains_string(&self) -> bool {
        self.contains_string.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use contains_string()")]
    pub fn get_contains_string(&self) -> bool {
        self.contains_string()
    }

    pub fn set_contains_string(&mut self, value: bool) -> &mut Self {
        self.contains_string.set_value(value);
        self
    }

    #[must_use]
    pub fn contains_number(&self) -> bool {
        self.contains_number.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use contains_number()")]
    pub fn get_contains_number(&self) -> bool {
        self.contains_number()
    }

    pub fn set_contains_number(&mut self, value: bool) -> &mut Self {
        self.contains_number.set_value(value);
        self
    }

    #[must_use]
    pub fn contains_integer(&self) -> bool {
        self.contains_integer.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use contains_integer()")]
    pub fn get_contains_integer(&self) -> bool {
        self.contains_integer()
    }

    pub fn set_contains_integer(&mut self, value: bool) -> &mut Self {
        self.contains_integer.set_value(value);
        self
    }

    #[must_use]
    pub fn min_value(&self) -> f64 {
        self.min_value.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use min_value()")]
    pub fn get_min_value(&self) -> f64 {
        self.min_value()
    }

    pub fn set_min_value(&mut self, value: f64) -> &mut Self {
        self.min_value.set_value(value);
        self
    }

    #[must_use]
    pub fn max_value(&self) -> f64 {
        self.max_value.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use max_value()")]
    pub fn get_max_value(&self) -> f64 {
        self.max_value()
    }

    pub fn set_max_value(&mut self, value: f64) -> &mut Self {
        self.max_value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}{}{}{}",
                &self.contains_semi_mixed_types.value_string(),
                &self.contains_string.value_string(),
                &self.contains_number.value_string(),
                &self.contains_integer.value_string(),
                &self.min_value.value_string(),
                &self.max_value.value_string(),
                self.items.iter().map(|v|match v {
                    SharedItemValue::Bool(v) => {format!("Bool|||{v}")},
                    SharedItemValue::Date(v) => {format!("Date|||{v}")},
                    SharedItemValue::Error(v) => {format!("Error|||{v}")},
                    SharedItemValue::Empty => {"Empty|||".to_string()},
                    SharedItemValue::Numeric(v) => {format!("Numeric|||{v}")},
                    SharedItemValue::String(v) => {format!("String|||{v}")},
                })
                .collect::<String>()
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, contains_semi_mixed_types, "containsSemiMixedTypes");
        set_string_from_xml!(self, e, contains_string, "containsString");
        set_string_from_xml!(self, e, contains_number, "containsNumber");
        set_string_from_xml!(self, e, contains_integer, "containsInteger");
        set_string_from_xml!(self, e, min_value, "minValue");
        set_string_from_xml!(self, e, max_value, "maxValue");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"b" => {
                        if let Some(v) = get_attribute(e, b"v") {
                            let obj = SharedItemValue::Bool(matches!(v.as_str(), "true" | "1"));
                            self.items.push(obj);
                        }
                    },
                    b"d" => {
                        if let Some(v) = get_attribute(e, b"v") {
                            let obj = SharedItemValue::Date(v.into_boxed_str());
                            self.items.push(obj);
                        }
                    },
                    b"e" => {
                        if let Some(v) = get_attribute(e, b"v") {
                            if let Ok(e) = CellErrorType::from_str(&v) {
                                let obj = SharedItemValue::Error(e);
                                self.items.push(obj);
                            }
                        }
                    },
                    b"m" => {
                        let obj = SharedItemValue::Empty;
                        self.items.push(obj);
                    },
                    b"n" => {
                        if let Some(v) = get_attribute(e, b"v") {
                            if let Ok(f) = v.parse::<f64>() {
                                let obj = SharedItemValue::Numeric(f);
                                self.items.push(obj);
                           }
                        }
                    },
                    b"s" => {
                        if let Some(v) = get_attribute(e, b"v") {
                            let obj = SharedItemValue::String(v.into_boxed_str());
                            self.items.push(obj);
                        }
                    },
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"sharedItems" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "sharedItems")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sharedItems
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.contains_semi_mixed_types.has_value() {
            attributes.push(("containsSemiMixedTypes", self.contains_semi_mixed_types.value_string()).into());
        }
        if self.contains_string.has_value() {
            attributes.push(("containsString", self.contains_string.value_string()).into());
        }
        if self.contains_number.has_value() {
            attributes.push(("containsNumber", self.contains_number.value_string()).into());
        }
        if self.contains_integer.has_value() {
            attributes.push(("containsInteger", self.contains_integer.value_string()).into());
        }
        if self.min_value.has_value() {
            attributes.push(("minValue", self.min_value.value_string()).into());
        }
        if self.max_value.has_value() {
            attributes.push(("maxValue", self.max_value.value_string()).into());
        }
        if !self.items.is_empty() {
            attributes.push(("count", self.items.len().to_string()).into());
        }
        write_start_tag(
            writer,
            "sharedItems",
            attributes,
            self.items.is_empty(),
        );

        if !self.items.is_empty() {
            for item in &self.items {
                let (tag, v) = match item {
                    SharedItemValue::Bool(v) => ("b", Some(v.to_string())),
                    SharedItemValue::Date(v) => ("d", Some(v.to_string())),
                    SharedItemValue::Error(v) => ("e", Some(v.to_string())),
                    SharedItemValue::Empty => ("m", None),
                    SharedItemValue::Numeric(v) => ("n", Some(v.to_string())),
                    SharedItemValue::String(v) => ("s", Some(v.to_string())),
                };
                let mut attributes: crate::structs::AttrCollection<'_> = Vec::new();
                if let Some(value) = v {
                    attributes.push(("v", value).into());
                }
                write_start_tag(
                    writer,
                    tag,
                    attributes,
                    true,
                );
            }
            write_end_tag(writer, "sharedItems");
        }
    }
}
