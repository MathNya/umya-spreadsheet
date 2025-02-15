use std::{
    collections::HashMap,
    io::Cursor,
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    helper::{
        coordinate::index_from_coordinate,
        formula::{
            FormulaToken,
            adjustment_insert_formula_coordinate,
            adjustment_remove_formula_coordinate,
            parse_to_tokens,
        },
    },
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::{
        BooleanValue,
        CellFormulaValues,
        EnumValue,
        StringValue,
        UInt32Value,
    },
    traits::AdjustmentCoordinateWith2Sheet,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node_conversion,
    },
};

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct CellFormula {
    bx:             BooleanValue,
    data_table_2d:  BooleanValue,
    data_table_row: BooleanValue,
    formula_type:   EnumValue<CellFormulaValues>,
    input_1deleted: BooleanValue,
    input_2deleted: BooleanValue,
    r1:             StringValue,
    r2:             StringValue,
    reference:      StringValue,
    shared_index:   UInt32Value,
    text:           StringValue,
    text_view:      StringValue,
}
impl CellFormula {
    #[inline]
    #[must_use]
    pub fn bx(&self) -> bool {
        self.bx.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bx()")]
    pub fn get_bx(&self) -> bool {
        self.bx()
    }

    #[inline]
    pub fn set_bx(&mut self, value: bool) -> &mut Self {
        self.bx.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn data_table_2d(&self) -> bool {
        self.data_table_2d.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bx()")]
    pub fn get_data_table_2d(&self) -> bool {
        self.data_table_2d()
    }

    #[inline]
    pub fn set_data_table_2d(&mut self, value: bool) -> &mut Self {
        self.data_table_2d.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn data_table_row(&self) -> bool {
        self.data_table_row.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_table_row()")]
    pub fn get_data_table_row(&self) -> bool {
        self.data_table_row()
    }

    #[inline]
    pub fn set_data_table_row(&mut self, value: bool) -> &mut Self {
        self.data_table_row.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn formula_type(&self) -> &CellFormulaValues {
        self.formula_type.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use formula_type()")]
    pub fn get_formula_type(&self) -> &CellFormulaValues {
        self.formula_type()
    }

    #[inline]
    pub fn set_formula_type(&mut self, value: CellFormulaValues) {
        self.formula_type.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn input_1deleted(&self) -> bool {
        self.input_1deleted.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use input_1deleted()")]
    pub fn get_input_1deleted(&self) -> bool {
        self.input_1deleted()
    }

    #[inline]
    pub fn set_input_1deleted(&mut self, value: bool) -> &mut Self {
        self.input_1deleted.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn input_2deleted(&self) -> bool {
        self.input_2deleted.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use input_2deleted()")]
    pub fn get_input_2deleted(&self) -> bool {
        self.input_2deleted()
    }

    #[inline]
    pub fn set_input_2deleted(&mut self, value: bool) -> &mut Self {
        self.input_2deleted.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn r1(&self) -> &str {
        self.r1.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use r1()")]
    pub fn get_r1(&self) -> &str {
        self.r1()
    }

    #[inline]
    pub fn set_r1<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.r1.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn r2(&self) -> &str {
        self.r2.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use r2()")]
    pub fn get_r2(&self) -> &str {
        self.r2()
    }
    
    #[inline]
    pub fn set_r2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.r2.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn reference(&self) -> &str {
        self.reference.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use reference()")]
    pub fn get_reference(&self) -> &str {
        self.reference()
    }

    #[inline]
    pub fn set_reference<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.reference.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn shared_index(&self) -> u32 {
        self.shared_index.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shared_index()")]
    pub fn get_shared_index(&self) -> u32 {
        self.shared_index()
    }

    #[inline]
    pub fn set_shared_index(&mut self, value: u32) -> &mut Self {
        self.shared_index.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn text(&self) -> &str {
        if self.text_view.has_value() {
            return self.text_view.value_str();
        }
        self.text.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use text()")]
    pub fn get_text(&self) -> &str {
        self.text()
    }

    #[inline]
    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text.set_value(value);
        self
    }

    #[inline]
    pub fn set_text_view<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text_view.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        is_empty: bool,
        cell_reference_str: &str,
        formula_shared_list: &mut HashMap<u32, (String, Vec<FormulaToken>)>,
    ) {
        set_string_from_xml!(self, e, bx, "bx");
        set_string_from_xml!(self, e, data_table_2d, "dt2D");
        set_string_from_xml!(self, e, data_table_row, "dtr");
        set_string_from_xml!(self, e, formula_type, "t");
        set_string_from_xml!(self, e, input_1deleted, "del1");
        set_string_from_xml!(self, e, input_2deleted, "del2");
        set_string_from_xml!(self, e, r1, "r1");
        set_string_from_xml!(self, e, r2, "r2");
        set_string_from_xml!(self, e, reference, "ref");
        set_string_from_xml!(self, e, shared_index, "si");

        if !is_empty {
            xml_read_loop!(
                reader,
                Event::Text(e) => {
                    self.text.set_value(e.unescape().unwrap().to_string());
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"f" {
                        break;
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "f")
            );
        }

        // Shared
        if self.formula_type.value() == &CellFormulaValues::Shared {
            match formula_shared_list.get(&self.shared_index.value()) {
                Some((parent_cell_reference_str, token)) => {
                    let parent_cell = index_from_coordinate(parent_cell_reference_str);
                    let self_cell = index_from_coordinate(cell_reference_str);
                    let parent_col_num = parent_cell.0.unwrap();
                    let parent_row_num = parent_cell.1.unwrap();
                    let self_col_num = self_cell.0.unwrap();
                    let self_row_num = self_cell.1.unwrap();

                    let root_col_num = parent_col_num;
                    let root_row_num = parent_row_num;
                    let offset_col_num = self_col_num - root_col_num;
                    let offset_row_num = self_row_num - parent_row_num;

                    let mut token_new = token.clone();
                    let value = adjustment_insert_formula_coordinate(
                        &mut token_new,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                        "",
                        "",
                        true,
                    );
                    self.text_view.set_value(value);
                }
                None => {
                    formula_shared_list.insert(
                        self.shared_index.value(),
                        (
                            cell_reference_str.to_string(),
                            parse_to_tokens(format!("={}", self.text.value_str())),
                        ),
                    );
                }
            }
        }
    }

    #[inline]
    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        coordinate: &str,
        formula_shared_list: &HashMap<u32, (String, Option<String>)>,
    ) {
        // f
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let bx_str = self.bx.value_string();
        if self.bx.has_value() {
            attributes.push(("bx", bx_str).into());
        }

        let data_table_2d_str = self.data_table_2d.value_string();
        if self.data_table_2d.has_value() {
            attributes.push(("dt2D", data_table_2d_str).into());
        }

        let data_table_row_str = self.data_table_row.value_string();
        if self.data_table_row.has_value() {
            attributes.push(("dtr", data_table_row_str).into());
        }

        let formula_type_str = self.formula_type.value_string();
        if self.formula_type.has_value() {
            // Not SUPPORT Array
            if self.formula_type.value() != &CellFormulaValues::Array {
                attributes.push(("t", formula_type_str).into());
            }
        }

        let input_1deleted_str = self.input_1deleted.value_string();
        if self.input_1deleted.has_value() {
            attributes.push(("del1", input_1deleted_str).into());
        }

        let input_2deleted_str = self.input_2deleted.value_string();
        if self.input_2deleted.has_value() {
            attributes.push(("del2", input_2deleted_str).into());
        }

        if self.r1.has_value() {
            attributes.push(("r1", self.r1.value_str()).into());
        }

        if self.r2.has_value() {
            attributes.push(("r2", self.r2.value_str()).into());
        }

        #[allow(unused_assignments)]
        let mut reference_str = String::new();
        if let Some((start_col, end_col)) = formula_shared_list.get(&self.shared_index.value())
        {
            if coordinate == start_col {
                reference_str = match end_col {
                    Some(v) => {
                        format!("{start_col}:{v}")
                    }
                    None => start_col.to_string(),
                };
                attributes.push(("ref", &reference_str).into());
            }
        }

        let shared_index_str = self.shared_index.value_string();
        if self.shared_index.has_value() {
            attributes.push(("si", &shared_index_str).into());
        }

        write_start_tag(writer, "f", attributes, false);
        write_text_node_conversion(writer, self.text.value_str());
        write_end_tag(writer, "f");
    }
}
impl AdjustmentCoordinateWith2Sheet for CellFormula {
    #[inline]
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if let Some(v) = self.text.value() {
            let formula = adjustment_insert_formula_coordinate(
                &mut parse_to_tokens(format!("={v}")),
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
                sheet_name,
                self_sheet_name,
                false,
            );
            self.text.set_value(formula);
        }
        if let Some(v) = self.text_view.value() {
            let formula = adjustment_insert_formula_coordinate(
                &mut parse_to_tokens(format!("={v}")),
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
                sheet_name,
                self_sheet_name,
                false,
            );
            self.text_view.set_value(formula);
        }
    }

    #[inline]
    fn adjustment_remove_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if let Some(v) = self.text.value() {
            let formula = adjustment_remove_formula_coordinate(
                &mut parse_to_tokens(format!("={v}")),
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
                sheet_name,
                self_sheet_name,
                false,
            );
            self.text.set_value(formula);
        }
        if let Some(v) = self.text_view.value() {
            let formula = adjustment_remove_formula_coordinate(
                &mut parse_to_tokens(format!("={v}")),
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
                sheet_name,
                self_sheet_name,
                false,
            );
            self.text_view.set_value(formula);
        }
    }
}
