use hashbrown::HashMap;
use helper::formula::*;
use helper::number_format::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::borrow::Cow;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use structs::CellFormula;
use structs::CellFormulaValues;
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
use structs::UInt32Value;
use traits::AdjustmentCoordinate;
use traits::AdjustmentCoordinateWith2Sheet;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Cell {
    coordinate: Coordinate,
    pub(crate) cell_value: CellValue,
    style: Style,
    hyperlink: Option<Hyperlink>,
    cell_meta_index: UInt32Value,
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

    pub fn get_hyperlink(&self) -> Option<&Hyperlink> {
        self.hyperlink.as_ref()
    }

    pub fn get_hyperlink_mut(&mut self) -> &mut Hyperlink {
        if self.hyperlink.is_some() {
            return self.hyperlink.as_mut().unwrap();
        }
        self.set_hyperlink(Hyperlink::default());
        self.hyperlink.as_mut().unwrap()
    }

    pub fn set_hyperlink(&mut self, value: Hyperlink) -> &mut Self {
        self.hyperlink = Some(value);
        self
    }

    pub fn get_cell_meta_index(&self) -> &u32 {
        self.cell_meta_index.get_value()
    }

    pub fn set_cell_meta_index(&mut self, value: u32) -> &mut Self {
        self.cell_meta_index.set_value(value);
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
    /// - `Empty` - if the string was `""`
    /// - `Numeric` - if the string can be parsed to an `f64`
    /// - `Bool` - if the string was either `"TRUE"` or `"FALSE"`
    /// - `Error` - if the string was either `"#VALUE!"`,`"#REF!"`,`"#NUM!"`,`"#NULL!"`,`"#NAME?"`,`"#N/A"`,`"#DATA!"` or `"#DIV/0!"`
    /// - `String` - if the string does not fulfill any of the other conditions
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_value(value);
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

    pub(crate) fn set_value_bool_crate(&mut self, value: bool) -> &mut Self {
        self.cell_value.set_value_bool_crate(value);
        self
    }

    pub fn set_value_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.cell_value.set_value_number(value);
        self
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.cell_value.set_rich_text(value);
        self
    }

    pub fn set_error<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_error(value);
        self
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_formula(value);
        self
    }

    pub fn set_formula_result_default<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.cell_value.set_formula_result_default(value);
        self
    }

    pub fn set_blank(&mut self) -> &mut Self {
        self.cell_value.set_blank();
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        self.cell_value.set_shared_string_item(value);
        self
    }

    pub fn get_data_type(&self) -> &str {
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

    pub fn get_formula_obj(&self) -> Option<&CellFormula> {
        self.cell_value.get_formula_obj()
    }

    pub fn get_formula_shared_index(&self) -> Option<&u32> {
        if let Some(v) = self.get_formula_obj() {
            if v.get_formula_type() == &CellFormulaValues::Shared {
                return Some(v.get_shared_index());
            }
        }
        None
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
        let value = self.get_formatted_value();

        value.split('\n').fold(0f64, |mut acc, value| {
            let mut point = 0f64;
            for chr in value.chars() {
                let clen = if chr.len_utf8() > 1 { 1.5 } else { 1.0 };

                point += clen;
            }
            if point > acc {
                acc = point;
            }
            acc
        })
    }

    pub fn get_formatted_value(&self) -> String {
        let value = self.get_value();

        // convert value
        let result = match self.get_style().get_number_format() {
            Some(nmuber_format) => to_formatted_string(&value, nmuber_format.get_format_code()),
            None => to_formatted_string(&value, NumberingFormat::FORMAT_GENERAL),
        };
        result
    }

    pub(crate) fn set_obj(&mut self, cell: Self) -> &mut Self {
        self.cell_value = cell.cell_value;
        self.style = cell.style;
        self.hyperlink = cell.hyperlink;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
        empty_flag: bool,
        formula_shared_list: &mut HashMap<u32, (String, Vec<FormulaToken>)>,
    ) {
        let mut type_value: String = String::from("");
        let mut cell_reference: String = String::from("");

        if let Some(v) = get_attribute(e, b"r") {
            cell_reference = v;
            self.coordinate.set_coordinate(&cell_reference);
        }

        if let Some(v) = get_attribute(e, b"s") {
            let style = stylesheet.get_style(v.parse::<usize>().unwrap());
            self.set_style(style);
        }

        if let Some(v) = get_attribute(e, b"t") {
            type_value = v;
        }

        set_string_from_xml!(self, e, cell_meta_index, "cm");

        if empty_flag {
            return;
        }

        let mut string_value: String = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"f" => {
                        let mut obj = CellFormula::default();
                        obj.set_attributes(reader, e, false, &cell_reference, formula_shared_list);
                        self.cell_value.set_formula_obj(obj);
                    }
                    b"t" => {
                        if let Some(Ok(attribute)) = e.attributes().next() {
                            if attribute.key.into_inner() == b"xml:space"
                                && attribute.value.as_ref() == b"preserve"
                            {
                                reader.config_mut().trim_text(false);
                            }
                        }
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => {
                    if e.name().into_inner() == b"f" {
                        let mut obj = CellFormula::default();
                        obj.set_attributes(reader, e, true, &cell_reference, formula_shared_list);
                        self.cell_value.set_formula_obj(obj);
                    }
                }
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"v" => match type_value.as_str() {
                        "str" => {
                            self.set_value_string(&string_value);
                        }
                        "s" => {
                            let index = string_value.parse::<usize>().unwrap();
                            let shared_string_item = shared_string_table
                                .get_shared_string_item()
                                .get(index)
                                .unwrap();
                            self.set_shared_string_item(shared_string_item.clone());
                        }
                        "b" => {
                            let prm = string_value == "1";
                            self.set_value_bool_crate(prm);
                        }
                        "e" => {
                            self.set_error(&string_value);
                        }
                        "" | "n" => {
                            self.set_value_crate(&string_value);
                        }
                        _ => {}
                    },
                    b"is" => {
                        if type_value == "inlineStr" {
                            self.set_value_crate(&string_value);
                        }
                    }
                    b"c" => return,
                    b"t" => {
                        reader.config_mut().trim_text(true);
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "c"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        shared_string_table: &Arc<RwLock<SharedStringTable>>,
        stylesheet: &mut Stylesheet,
        formula_shared_list: &HashMap<&u32, (String, Option<String>)>,
    ) {
        let empty_flag_value = self.cell_value.is_empty();
        let empty_flag_style = self.style.is_empty();
        if empty_flag_value && empty_flag_style {
            return;
        }

        // c
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let coordinate = self.coordinate.to_string();
        attributes.push(("r", &coordinate));
        if self.get_data_type_crate() == "s"
            || self.get_data_type_crate() == "b"
            || self.get_data_type_crate() == "str"
            || self.get_data_type_crate() == "e"
        {
            attributes.push(("t", self.get_data_type_crate()));
        }
        let xf_index_str: String;
        let xf_index = stylesheet.set_style(self.get_style());
        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("s", &xf_index_str));
        }

        let cell_meta_index_str = self.cell_meta_index.get_value_string();
        if self.cell_meta_index.has_value() {
            // NOT SUPPORT
            //attributes.push(("cm", &cell_meta_index_str));
        }

        if empty_flag_value {
            write_start_tag(writer, "c", attributes, true);
            return;
        }

        write_start_tag(writer, "c", attributes, false);
        // f
        match &self.cell_value.formula {
            Some(v) => {
                v.write_to(writer, &coordinate, formula_shared_list);
            }
            None => {}
        }

        // v
        if self.cell_value.is_value_empty() {
            write_start_tag(writer, "v", vec![], true);
        } else {
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
        }

        write_end_tag(writer, "c");
    }
}
impl AdjustmentCoordinate for Cell {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.coordinate.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.coordinate.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
impl AdjustmentCoordinateWith2Sheet for Cell {
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.cell_value.adjustment_insert_coordinate_with_2sheet(
            self_sheet_name,
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.cell_value.adjustment_remove_coordinate_with_2sheet(
            self_sheet_name,
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
