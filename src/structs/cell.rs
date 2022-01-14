use super::CellValue;
use super::Hyperlink;
use super::Coordinate;
use super::Style;
use super::RichText;
use super::SharedStringItem;
use super::Stylesheet;
use super::SharedStringTable;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Cell {
    coordinate: Coordinate,
    pub(crate)cell_value: CellValue,
    style: Style,
    hyperlink: Option<Hyperlink>,
}
impl Cell {
    // Data types
    pub const TYPE_STRING2: &'static str = "str";
    pub const TYPE_STRING: &'static str = "s";
    pub const TYPE_FORMULA: &'static str = "f";
    pub const TYPE_NUMERIC: &'static str = "n";
    pub const TYPE_BOOL: &'static str = "b";
    pub const TYPE_NULL: &'static str = "null";
    pub const TYPE_INLINE: &'static str = "inlineStr";
    pub const TYPE_ERROR: &'static str = "e";

    pub fn get_cell_value(&self)-> &CellValue {
        &self.cell_value
    }

    pub fn get_cell_value_mut(&mut self)-> &mut CellValue {
        &mut self.cell_value
    }

    pub fn set_cell_value(&mut self, value:CellValue)-> &mut Self {
        self.cell_value = value;
        self
    }


    pub fn get_style(&self)-> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self)-> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value:Style)-> &mut Self {
        self.style = value;
        self
    }

    pub fn get_coordinate(&self)-> &Coordinate {
        &self.coordinate
    }

    pub fn get_coordinate_mut(&mut self)-> &mut Coordinate {
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

    pub fn set_hyperlink(&mut self, value:Hyperlink)-> &mut Self {
        self.hyperlink = Some(value);
        self
    }

    pub fn get_value(&self)-> &str {
        match &self.cell_value.value {
            Some(v) => {return v;},
            None => {},
        }
        match &self.cell_value.rich_text {
            Some(v) => {
                return v.get_text();
            },
            None => {},
        }
        ""
    }
    
    pub(crate) fn get_value_crate(&self)-> &Option<String> {
        &self.cell_value.value
    }

    pub fn get_rich_text(&self)-> &Option<RichText> {
        &self.cell_value.rich_text
    }

    pub fn set_value<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.cell_value.set_value(value);
        self
    }

    pub fn set_value_from_string<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.cell_value.set_value_from_string(value);
        self
    }

    pub fn set_value_from_bool(&mut self, value:bool)-> &mut Self {
        self.cell_value.set_value_from_bool(value);
        self
    }

    pub fn set_value_from_bool_ref(&mut self, value:&bool)-> &mut Self {
        self.cell_value.set_value_from_bool_ref(value);
        self
    }

    pub fn set_value_from_u16(&mut self, value:u16)-> &mut Self {
        self.cell_value.set_value_from_u16(value);
        self
    }

    pub fn set_value_from_u16_ref(&mut self, value:&u16)-> &mut Self {
        self.cell_value.set_value_from_u16_ref(value);
        self
    }

    pub fn set_value_from_u32(&mut self, value:u32)-> &mut Self {
        self.cell_value.set_value_from_u32(value);
        self
    }

    pub fn set_value_from_u32_ref(&mut self, value:&u32)-> &mut Self {
        self.cell_value.set_value_from_u32_ref(value);
        self
    }

    pub fn set_value_from_u64(&mut self, value:u64)-> &mut Self {
        self.cell_value.set_value_from_u64(value);
        self
    }

    pub fn set_value_from_u64_ref(&mut self, value:&u64)-> &mut Self {
        self.cell_value.set_value_from_u64_ref(value);
        self
    }

    pub fn set_value_from_i16(&mut self, value:i16)-> &mut Self {
        self.cell_value.set_value_from_i16(value);
        self
    }

    pub fn set_value_from_i16_ref(&mut self, value:&i16)-> &mut Self {
        self.cell_value.set_value_from_i16_ref(value);
        self
    }

    pub fn set_value_from_i32(&mut self, value:i32)-> &mut Self {
        self.cell_value.set_value_from_i32(value);
        self
    }

    pub fn set_value_from_i32_ref(&mut self, value:&i32)-> &mut Self {
        self.cell_value.set_value_from_i32_ref(value);
        self
    }

    pub fn set_value_from_i64(&mut self, value:i64)-> &mut Self {
        self.cell_value.set_value_from_i64(value);
        self
    }

    pub fn set_value_from_i64_ref(&mut self, value:&i64)-> &mut Self {
        self.cell_value.set_value_from_i64_ref(value);
        self
    }

    pub fn set_value_from_usize(&mut self, value:usize)-> &mut Self {
        self.cell_value.set_value_from_usize(value);
        self
    }

    pub fn set_value_from_usize_ref(&mut self, value:&usize)-> &mut Self {
        self.cell_value.set_value_from_usize_ref(value);
        self
    }

    pub fn set_rich_text(&mut self, value:RichText)-> &mut Self {
        self.cell_value.set_rich_text(value);
        self
    }

    pub fn set_rich_text_ref(&mut self, value:&RichText)-> &mut Self {
        self.cell_value.set_rich_text_ref(value);
        self
    }
    
    pub fn set_formula<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.cell_value.set_formula(value);
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value:SharedStringItem)-> &mut Self {
        self.cell_value.set_shared_string_item(value);
        self
    }

    pub fn get_data_type(&self)-> &str {
        &self.cell_value.data_type
    }

    pub fn set_data_type<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.cell_value.set_data_type(value);
        self
    }

    pub fn is_formula(&self) -> bool {
        self.cell_value.is_formula()
    }

    pub fn get_formula(&self)-> &str {
        &self.cell_value.get_formula()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        e:&BytesStart,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
        empty_flag:bool,
    ) {
        let mut type_value: String = String::from("");

        match get_attribute(e, b"r") {
            Some(v) => {
                self.coordinate.set_coordinate(v);
            },
            None => {}
        }

        match get_attribute(e, b"s") {
            Some(v) => {
                let style = stylesheet.get_style(v.parse::<usize>().unwrap());
                self.set_style(style);
            },
            None => {}
        }

        match get_attribute(e, b"t") {
            Some(v) => {
                type_value = v;
            },
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut string_value: String = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"f" => {
                            self.set_formula(string_value.clone());
                        },
                        b"v" => {
                            if type_value == "s" {
                                let index = string_value.parse::<usize>().unwrap();
                                let shared_string_item = shared_string_table.get_shared_string_item().get(index).unwrap();
                                self.set_shared_string_item(shared_string_item.clone());
                            } else if type_value == "b" {
                                let prm = if &string_value == "1" {true} else {false};
                                let _ = self.set_value_from_bool(prm);
                            } else if type_value == "" || type_value == "n" {
                                let _ = self.set_value(&string_value);
                            };
                        },
                        b"c" => return,
                        _ => (),
                    }
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
        shared_string_table: &mut SharedStringTable,
        stylesheet: &mut Stylesheet,
    ) {
        let empty_flag = self.cell_value.is_empty();

        // c
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let coordinate = self.coordinate.get_coordinate();
        attributes.push(("r", &coordinate));
        if self.get_data_type() == "s" || self.get_data_type() == "b" {
            attributes.push(("t", self.get_data_type()));
        }
        let xf_index_str:String;
        let xf_index = stylesheet.set_style(self.get_style());
        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("s", &xf_index_str));
        }
        write_start_tag(writer, "c", attributes, empty_flag);

        if empty_flag == false {
            // f
            match &self.cell_value.formula {
                Some(v) => {
                    write_start_tag(writer, "f", vec![], false);
                    write_text_node(writer, v);
                    write_end_tag(writer, "f");
                },
                None => {},
            }

            // v
            write_start_tag(writer, "v", vec![], false);
            match self.get_data_type() {
                "s" => {
                    let val_index = shared_string_table.set_cell(self.get_cell_value());
                    write_text_node(writer, val_index.to_string());
                },
                "b" => {
                    let upper_value = self.get_value().to_uppercase();
                    let prm = if upper_value == "TRUE" {"1"} else {"0"};
                    write_text_node(writer, prm);
                },
                _ => write_text_node(writer, self.get_value()),
            }
            write_end_tag(writer, "v");

            write_end_tag(writer, "c");
        }
    }
}
