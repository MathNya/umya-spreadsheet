use super::BooleanValue;
use super::Cell;
use super::Cells;
use super::DoubleValue;
use super::SharedStringTable;
use super::Style;
use super::Stylesheet;
use super::UInt32Value;
use hashbrown::HashMap;
use helper::formula::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use traits::AdjustmentValue;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Row {
    row_num: UInt32Value,
    height: DoubleValue,
    descent: DoubleValue,
    thick_bot: BooleanValue,
    custom_height: BooleanValue,
    hidden: BooleanValue,
    style: Style,
}

impl Row {
    pub fn get_row_num(&self) -> &u32 {
        self.row_num.get_value()
    }

    pub(crate) fn set_row_num(&mut self, value: u32) -> &mut Self {
        self.row_num.set_value(value);
        self
    }

    pub fn get_height(&self) -> &f64 {
        self.height.get_value()
    }

    pub fn set_height(&mut self, value: f64) -> &mut Self {
        self.height.set_value(value);
        self.custom_height.set_value(true);
        self
    }

    pub fn get_descent(&self) -> &f64 {
        self.descent.get_value()
    }

    pub fn set_descent(&mut self, value: f64) -> &mut Self {
        self.descent.set_value(value);
        self
    }

    pub fn get_thick_bot(&self) -> &bool {
        self.thick_bot.get_value()
    }

    pub fn set_thick_bot(&mut self, value: bool) -> &mut Self {
        self.thick_bot.set_value(value);
        self
    }

    pub fn get_custom_height(&self) -> &bool {
        self.custom_height.get_value()
    }

    pub fn set_custom_height(&mut self, value: bool) -> &mut Self {
        self.custom_height.set_value(value);
        self
    }

    pub fn get_hidden(&self) -> &bool {
        self.hidden.get_value()
    }

    pub fn set_hidden(&mut self, value: bool) -> &mut Self {
        self.hidden.set_value(value);
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

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        cells: &mut Cells,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
        formula_shared_list: &mut HashMap<u32, (String, Vec<FormulaToken>)>,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, row_num, "r");
        set_string_from_xml!(self, e, height, "ht");
        set_string_from_xml!(self, e, thick_bot, "thickBot");
        set_string_from_xml!(self, e, custom_height, "customHeight");
        set_string_from_xml!(self, e, hidden, "hidden");

        if let Some(v) = get_attribute(e, b"x14ac:dyDescent") {
            if !v.is_empty() {
                self.descent.set_value_string(v);
            }
        }

        if let Some(v) = get_attribute(e, b"s") {
            let style = stylesheet.get_style(v.parse::<usize>().unwrap());
            self.set_style(style);
        }

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"c" {
                    let mut obj = Cell::default();
                    obj.set_attributes(reader, e, shared_string_table, stylesheet, true, formula_shared_list);
                    cells.set_fast(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"c" {
                    let mut obj = Cell::default();
                    obj.set_attributes(reader, e, shared_string_table, stylesheet, false, formula_shared_list);
                    cells.set_fast(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"row" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "row")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        stylesheet: &mut Stylesheet,
        spans: String,
        empty_flag: bool,
    ) {
        let xf_index_str: String;
        let xf_index = stylesheet.set_style(self.get_style());

        // row
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let row_num = self.row_num.get_value_string();
        attributes.push(("r", &row_num));
        if !empty_flag {
            attributes.push(("spans", &spans));
        }
        let height = self.height.get_value_string();
        if self.height.get_value() != &0f64 {
            attributes.push(("ht", &height));
        }
        if *self.thick_bot.get_value() {
            attributes.push(("thickBot", self.thick_bot.get_value_string()));
        }
        if *self.custom_height.get_value() {
            attributes.push(("customHeight", self.custom_height.get_value_string()));
        }
        if xf_index > 0 {
            attributes.push(("customFormat", "1"));
        }
        if *self.hidden.get_value() {
            attributes.push(("hidden", self.hidden.get_value_string()));
        }
        let descent = self.descent.get_value_string();
        if self.descent.has_value() {
            attributes.push(("x14ac:dyDescent", &descent));
        }

        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("s", &xf_index_str));
        }

        write_start_tag(writer, "row", attributes, empty_flag);
    }
}
impl AdjustmentValue for Row {
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32) {
        if self.row_num.get_value() >= root_num {
            self.row_num
                .set_value(self.row_num.get_value() + offset_num);
        }
    }

    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32) {
        if self.row_num.get_value() >= root_num {
            self.row_num
                .set_value(self.row_num.get_value() - offset_num);
        }
    }

    fn is_remove_value(&self, root_num: &u32, offset_num: &u32) -> bool {
        self.row_num.get_value() >= root_num
            && self.row_num.get_value() <= &(root_num + offset_num - 1)
    }
}
