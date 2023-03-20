use helper::number_format::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::borrow::Cow;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use structs::CellRawValue;
use structs::CellValue;
use structs::Coordinate;
use structs::Hyperlink;
use structs::NumberingFormat;
use structs::RichText;
use structs::SharedStringItem;
use structs::SharedStringTable;
use structs::Style;
use structs::Stylesheet;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Cell {
    coordinate: Coordinate,
    pub(crate) cell_value: CellValue,
    style: Style,
    hyperlink: Option<Hyperlink>,
}
impl Cell {
    pub fn get_cell_value(&self) -> &CellValue {
        &self.cell_value
    }

    pub fn get_cell_value_mut(&mut self) -> &mut CellValue {
        &mut self.cell_value
    }

    pub fn set_cell_value(&mut self, value: CellValue) -> &mut Self {
        self.cell_value = value;
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = value;
        self
    }

    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.coordinate
    }

    pub fn get_hyperlink(&self) -> &Option<Hyperlink> {
        &self.hyperlink
    }

    pub fn get_hyperlink_mut(&mut self) -> &mut Hyperlink {
        match &self.hyperlink {
            Some(_) => return self.hyperlink.as_mut().unwrap(),
            None => {}
        }
        let _ = self.set_hyperlink(Hyperlink::default());
        self.hyperlink.as_mut().unwrap()
    }

    pub fn set_hyperlink(&mut self, value: Hyperlink) -> &mut Self {
        self.hyperlink = Some(value);
        self
    }

    pub fn get_value(&self) -> Cow<'static, str> {
        self.cell_value.get_value()
    }

    pub fn get_value_number(&self) -> Option<f64> {
        self.cell_value.get_value_number()
    }

    pub fn get_value_lazy(&mut self) -> Cow<'static, str> {
        self.cell_value.get_value_lazy()
    }

    /// Set the cell's value after trying to convert `value` into one of the supported data types.
    /// <br />
    /// Types that `value` may be converted to:
    /// - `Null` - if the string was `"NULL"`
    /// - `Numeric` - if the string can be parsed to an `f64`
    /// - `Bool` - if the string was either `"TRUE"` or `"FALSE"`
    /// - `Error` - if the string was `"#VALUE!"`
    /// - `String` - if the string does not fulfill any of the other conditions
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value(value);
        self
    }

    #[deprecated(note = "use `set_value` or `set_value_string` instead")]
    pub fn set_value_from_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value_from_string(value);
        self
    }

    pub(crate) fn set_value_crate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value_crate(value);
        self
    }

    pub fn set_value_lazy<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value_lazy(value);
        self
    }

    pub fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value_string(value);
        self
    }

    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.cell_value.set_value_bool(value);
        self
    }

    #[deprecated(note = "use `set_value_bool` instead")]
    pub fn set_value_from_bool_ref(&mut self, value: &bool) -> &mut Self {
        self.cell_value.set_value_from_bool_ref(value);
        self
    }

    pub fn set_value_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.cell_value.set_value_number(value);
        self
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u16(&mut self, value: u16) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u16_ref(&mut self, value: &u16) -> &mut Self {
        self.set_value_number(*value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u32(&mut self, value: u32) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u32_ref(&mut self, value: &u32) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u64(&mut self, value: u64) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u64_ref(&mut self, value: &u64) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i16(&mut self, value: i16) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i16_ref(&mut self, value: &i16) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i32(&mut self, value: i32) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i32_ref(&mut self, value: &i32) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i64(&mut self, value: i64) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i64_ref(&mut self, value: &i64) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_usize(&mut self, value: usize) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_usize_ref(&mut self, value: &usize) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.cell_value.set_rich_text(value);
        self
    }

    #[deprecated(note = "use `set_rich_text` instead")]
    pub fn set_rich_text_ref(&mut self, value: &RichText) -> &mut Self {
        self.cell_value.set_rich_text_ref(value);
        self
    }

    pub fn set_error(&mut self) -> &mut Self {
        self.cell_value.set_error();
        self
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_formula(value);
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        self.cell_value.set_shared_string_item(value);
        self
    }

    pub fn get_data_type(&self) -> &CellRawValue {
        self.cell_value.get_data_type()
    }

    pub fn get_raw_value(&self) -> &CellRawValue {
        self.cell_value.get_raw_value()
    }

    pub(crate) fn get_data_type_crate(&self) -> &str {
        self.cell_value.get_data_type_crate()
    }

    pub fn is_formula(&self) -> bool {
        self.cell_value.is_formula()
    }

    pub fn get_formula(&self) -> &str {
        self.cell_value.get_formula()
    }

    pub(crate) fn get_formula_attributes(&self) -> Vec<(&str, &str)> {
        self.cell_value.get_formula_attributes()
    }

    pub(crate) fn set_formula_attributes(&mut self, attributes: Vec<(String, String)>) {
        self.cell_value.set_formula_attributes(attributes);
    }

    pub(crate) fn get_width_point(&self, column_font_size: &f64) -> f64 {
        // get cell value len.
        let char_cnt = self.get_width_point_cell();

        // get font size.
        let font_size = match self.get_style().get_font() {
            Some(font) => font.get_font_size().get_val(),
            None => column_font_size,
        };

        let mut column_width = 1.4 * char_cnt;
        column_width = column_width * font_size / 11f64;

        column_width
    }

    pub(crate) fn get_width_point_cell(&self) -> f64 {
        let mut max_point = 0f64;
        let value = self.get_formatted_value();
        let value_list: Vec<&str> = value.split('\n').collect();
        for value in value_list {
            let mut point = 0f64;
            for chr in value.chars() {
                let mut clen = chr.len_utf8() as f64;
                if clen > 1f64 {
                    clen = 1.5;
                }
                point += clen;
            }
            if point > max_point {
                max_point = point;
            }
        }
        max_point
    }

    pub(crate) fn get_formatted_value(&self) -> String {
        let value = self.get_value();

        // convert value
        let result = match self.get_style().get_number_format() {
            Some(nmuber_format) => to_formatted_string(&value, nmuber_format.get_format_code()),
            None => to_formatted_string(&value, NumberingFormat::FORMAT_GENERAL),
        };
        result
    }

    pub(crate) fn set_obj(&mut self, cell: Self) -> &mut Self {
        self.cell_value = cell.get_cell_value().clone();
        self.style = cell.get_style().clone();
        self.hyperlink = cell.get_hyperlink().clone();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
        empty_flag: bool,
    ) {
        let mut type_value: String = String::from("");

        match get_attribute(e, b"r") {
            Some(v) => {
                self.coordinate.set_coordinate(v);
            }
            None => {}
        }

        match get_attribute(e, b"s") {
            Some(v) => {
                let style = stylesheet.get_style(v.parse::<usize>().unwrap());
                self.set_style(style);
            }
            None => {}
        }

        match get_attribute(e, b"t") {
            Some(v) => {
                type_value = v;
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut string_value: String = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
                Ok(Event::Start(ref s)) => match s.name().into_inner() {
                    b"f" => {
                        let mut attrs = vec![];
                        s.attributes().for_each(|a| {
                            if let Ok(attribute) = a {
                                if let (Ok(key), Ok(value)) = (
                                    std::str::from_utf8(attribute.key.into_inner()),
                                    std::str::from_utf8(attribute.value.as_ref()),
                                ) {
                                    attrs.push((key.to_owned(), value.to_owned()));
                                }
                            }
                        });
                        self.set_formula_attributes(attrs);
                    }
                    b"t" => {
                        if let Some(Ok(attribute)) = s.attributes().next() {
                            if attribute.key.into_inner() == b"xml:space"
                                && attribute.value.as_ref() == b"preserve"
                            {
                                reader.trim_text(false);
                            }
                        }
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref s)) => {
                    if s.name().into_inner() == b"f" {
                        let mut attrs = vec![];
                        s.attributes().for_each(|a| {
                            if let Ok(attribute) = a {
                                if let (Ok(key), Ok(value)) = (
                                    std::str::from_utf8(attribute.key.into_inner()),
                                    std::str::from_utf8(attribute.value.as_ref()),
                                ) {
                                    attrs.push((key.to_owned(), value.to_owned()));
                                }
                            }
                        });
                        self.set_formula_attributes(attrs);
                    }
                }
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"f" => {
                        self.set_formula(string_value.clone());
                    }
                    b"v" => {
                        if type_value == "s" {
                            let index = string_value.parse::<usize>().unwrap();
                            let shared_string_item = shared_string_table
                                .get_shared_string_item()
                                .get(index)
                                .unwrap();
                            self.set_shared_string_item(shared_string_item.clone());
                        } else if type_value == "b" {
                            let prm = &string_value == "1";
                            let _ = self.set_value_bool(prm);
                        } else if type_value == "e" {
                            let _ = self.set_error();
                        } else if type_value.is_empty() || type_value == "n" {
                            let _ = self.set_value_crate(string_value.clone());
                        };
                    }
                    b"is" => {
                        if type_value == "inlineStr" {
                            self.set_value_crate(string_value.clone());
                        }
                    }
                    b"c" => return,
                    b"t" => {
                        reader.trim_text(true);
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        shared_string_table: Arc<RwLock<SharedStringTable>>,
        stylesheet: &mut Stylesheet,
    ) {
        let empty_flag_value = self.cell_value.is_empty();
        let empty_flag_style = self.style.is_empty();
        if empty_flag_value && empty_flag_style {
            return;
        }

        // c
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let coordinate = self.coordinate.get_coordinate();
        attributes.push(("r", &coordinate));
        if self.get_data_type_crate() == "s" || self.get_data_type_crate() == "b" {
            attributes.push(("t", self.get_data_type_crate()));
        }
        let xf_index_str: String;
        let xf_index = stylesheet.set_style(self.get_style());
        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("s", &xf_index_str));
        }

        if !empty_flag_value {
            write_start_tag(writer, "c", attributes, false);
            // f
            match &self.cell_value.formula {
                Some(v) => {
                    write_start_tag(writer, "f", self.get_formula_attributes(), false);
                    write_text_node(writer, v);
                    write_end_tag(writer, "f");
                }
                None => {
                    if !self.get_formula_attributes().is_empty() {
                        write_start_tag(writer, "f", self.get_formula_attributes(), true);
                    }
                }
            }

            // v
            if !self.cell_value.is_value_empty() {
                write_start_tag(writer, "v", vec![], false);

                //todo use typed value
                match self.get_data_type_crate() {
                    "s" => {
                        let val_index = shared_string_table
                            .write()
                            .unwrap()
                            .set_cell(self.get_cell_value());
                        write_text_node(writer, val_index.to_string());
                    }
                    "b" => {
                        let upper_value = self.get_value().to_uppercase();
                        let prm = if upper_value == "TRUE" { "1" } else { "0" };
                        write_text_node(writer, prm);
                    }
                    "e" => {
                        let prm = "#VALUE!";
                        write_text_node(writer, prm);
                    }
                    _ => write_text_node(writer, self.get_value()),
                }
                write_end_tag(writer, "v");
            } else {
                write_start_tag(writer, "v", vec![], true);
            }

            write_end_tag(writer, "c");
        } else {
            write_start_tag(writer, "c", attributes, true);
        }
    }
}
