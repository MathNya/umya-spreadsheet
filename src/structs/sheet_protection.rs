// sheetProtection
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    BooleanValue,
    StringValue,
    UInt32Value,
};
use crate::{
    helper::crypt::encrypt_sheet_protection,
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct SheetProtection {
    algorithm_name: StringValue,
    hash_value: StringValue,
    salt_value: StringValue,
    spin_count: UInt32Value,
    password: StringValue,
    sheet: BooleanValue,
    objects: BooleanValue,
    delete_rows: BooleanValue,
    insert_columns: BooleanValue,
    delete_columns: BooleanValue,
    insert_hyperlinks: BooleanValue,
    auto_filter: BooleanValue,
    scenarios: BooleanValue,
    format_cells: BooleanValue,
    format_columns: BooleanValue,
    insert_rows: BooleanValue,
    format_rows: BooleanValue,
    pivot_tables: BooleanValue,
    select_locked_cells: BooleanValue,
    select_unlocked_cells: BooleanValue,
    sort: BooleanValue,
}
impl SheetProtection {
    #[inline]
    #[must_use]
    pub fn get_algorithm_name(&self) -> &str {
        self.algorithm_name.get_value_str()
    }

    #[inline]
    pub fn set_algorithm_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.algorithm_name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_hash_value(&self) -> &str {
        self.hash_value.get_value_str()
    }

    #[inline]
    pub fn set_hash_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.hash_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_salt_value(&self) -> &str {
        self.salt_value.get_value_str()
    }

    #[inline]
    pub fn set_salt_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.salt_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_spin_count(&self) -> u32 {
        self.spin_count.get_value()
    }

    #[inline]
    pub fn set_spin_count(&mut self, value: u32) -> &mut Self {
        self.spin_count.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_password_raw(&self) -> &str {
        self.password.get_value_str()
    }

    #[inline]
    pub fn set_password_raw<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.password.set_value(value);
        self
    }

    #[inline]
    pub fn remove_password_raw(&mut self) -> &mut Self {
        self.password.remove_value();
        self
    }

    #[inline]
    #[must_use]
    pub fn get_sheet(&self) -> bool {
        self.sheet.get_value()
    }

    #[inline]
    pub fn set_sheet(&mut self, value: bool) -> &mut Self {
        self.sheet.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_objects(&self) -> bool {
        self.objects.get_value()
    }

    #[inline]
    pub fn set_objects(&mut self, value: bool) -> &mut Self {
        self.objects.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_delete_rows(&self) -> bool {
        self.delete_rows.get_value()
    }

    #[inline]
    pub fn set_delete_rows(&mut self, value: bool) -> &mut Self {
        self.delete_rows.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_insert_columns(&self) -> bool {
        self.insert_columns.get_value()
    }

    #[inline]
    pub fn set_insert_columns(&mut self, value: bool) -> &mut Self {
        self.insert_columns.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_delete_columns(&self) -> bool {
        self.delete_columns.get_value()
    }

    #[inline]
    pub fn set_delete_columns(&mut self, value: bool) -> &mut Self {
        self.delete_columns.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_insert_hyperlinks(&self) -> bool {
        self.insert_hyperlinks.get_value()
    }

    #[inline]
    pub fn set_insert_hyperlinks(&mut self, value: bool) -> &mut Self {
        self.insert_hyperlinks.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_auto_filter(&self) -> bool {
        self.auto_filter.get_value()
    }

    #[inline]
    pub fn set_auto_filter(&mut self, value: bool) -> &mut Self {
        self.auto_filter.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_scenarios(&self) -> bool {
        self.scenarios.get_value()
    }

    #[inline]
    pub fn set_scenarios(&mut self, value: bool) -> &mut Self {
        self.scenarios.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_format_cells(&self) -> bool {
        self.format_cells.get_value()
    }

    #[inline]
    pub fn set_format_cells(&mut self, value: bool) -> &mut Self {
        self.format_cells.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_format_columns(&self) -> bool {
        self.format_columns.get_value()
    }

    #[inline]
    pub fn set_format_columns(&mut self, value: bool) -> &mut Self {
        self.format_columns.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_insert_rows(&self) -> bool {
        self.insert_rows.get_value()
    }

    #[inline]
    pub fn set_insert_rows(&mut self, value: bool) -> &mut Self {
        self.insert_rows.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_format_rows(&self) -> bool {
        self.format_rows.get_value()
    }

    #[inline]
    pub fn set_format_rows(&mut self, value: bool) -> &mut Self {
        self.format_rows.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_pivot_tables(&self) -> bool {
        self.pivot_tables.get_value()
    }

    #[inline]
    pub fn set_pivot_tables(&mut self, value: bool) -> &mut Self {
        self.pivot_tables.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_select_locked_cells(&self) -> bool {
        self.select_locked_cells.get_value()
    }

    #[inline]
    pub fn set_select_locked_cells(&mut self, value: bool) -> &mut Self {
        self.select_locked_cells.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_select_unlocked_cells(&self) -> bool {
        self.select_unlocked_cells.get_value()
    }

    #[inline]
    pub fn set_select_unlocked_cells(&mut self, value: bool) -> &mut Self {
        self.select_unlocked_cells.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_sort(&self) -> bool {
        self.sort.get_value()
    }

    #[inline]
    pub fn set_sort(&mut self, value: bool) -> &mut Self {
        self.sort.set_value(value);
        self
    }

    #[inline]
    pub fn set_password(&mut self, password: &str) -> &mut Self {
        encrypt_sheet_protection(password, self);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, algorithm_name, "algorithmName");
        set_string_from_xml!(self, e, hash_value, "hashValue");
        set_string_from_xml!(self, e, salt_value, "saltValue");
        set_string_from_xml!(self, e, spin_count, "spinCount");
        set_string_from_xml!(self, e, password, "password");
        set_string_from_xml!(self, e, sheet, "sheet");
        set_string_from_xml!(self, e, objects, "objects");
        set_string_from_xml!(self, e, delete_rows, "deleteRows");
        set_string_from_xml!(self, e, insert_columns, "insertColumns");
        set_string_from_xml!(self, e, delete_columns, "deleteColumns");
        set_string_from_xml!(self, e, insert_hyperlinks, "insertHyperlinks");
        set_string_from_xml!(self, e, auto_filter, "autoFilter");
        set_string_from_xml!(self, e, scenarios, "scenarios");
        set_string_from_xml!(self, e, format_cells, "formatCells");
        set_string_from_xml!(self, e, format_columns, "formatColumns");
        set_string_from_xml!(self, e, insert_rows, "insertRows");
        set_string_from_xml!(self, e, format_rows, "formatRows");
        set_string_from_xml!(self, e, pivot_tables, "pivotTables");
        set_string_from_xml!(self, e, select_locked_cells, "selectLockedCells");
        set_string_from_xml!(self, e, select_unlocked_cells, "selectUnlockedCells");
        set_string_from_xml!(self, e, sort, "sort");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sheetProtection
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.algorithm_name.has_value() {
            attributes.push(("algorithmName", self.algorithm_name.get_value_str()));
        }
        if self.hash_value.has_value() {
            attributes.push(("hashValue", self.hash_value.get_value_str()));
        }
        if self.salt_value.has_value() {
            attributes.push(("saltValue", self.salt_value.get_value_str()));
        }
        let spin_count = self.spin_count.get_value_string();
        if self.spin_count.has_value() {
            attributes.push(("spinCount", &spin_count));
        }
        if self.password.has_value() {
            attributes.push(("password", self.password.get_value_str()));
        }
        if self.sheet.has_value() {
            attributes.push(("sheet", self.sheet.get_value_string()));
        }
        if self.objects.has_value() {
            attributes.push(("objects", self.objects.get_value_string()));
        }
        if self.delete_rows.has_value() {
            attributes.push(("deleteRows", self.delete_rows.get_value_string()));
        }
        if self.insert_columns.has_value() {
            attributes.push(("insertColumns", self.insert_columns.get_value_string()));
        }
        if self.delete_columns.has_value() {
            attributes.push(("deleteColumns", self.delete_columns.get_value_string()));
        }
        if self.insert_hyperlinks.has_value() {
            attributes.push((
                "insertHyperlinks",
                self.insert_hyperlinks.get_value_string(),
            ));
        }
        if self.auto_filter.has_value() {
            attributes.push(("autoFilter", self.auto_filter.get_value_string()));
        }
        if self.scenarios.has_value() {
            attributes.push(("scenarios", self.scenarios.get_value_string()));
        }
        if self.format_cells.has_value() {
            attributes.push(("formatCells", self.format_cells.get_value_string()));
        }
        if self.format_columns.has_value() {
            attributes.push(("formatColumns", self.format_columns.get_value_string()));
        }
        if self.insert_rows.has_value() {
            attributes.push(("insertRows", self.insert_rows.get_value_string()));
        }
        if self.format_rows.has_value() {
            attributes.push(("formatRows", self.format_rows.get_value_string()));
        }
        if self.pivot_tables.has_value() {
            attributes.push(("pivotTables", self.pivot_tables.get_value_string()));
        }
        if self.select_locked_cells.has_value() {
            attributes.push((
                "selectLockedCells",
                self.select_locked_cells.get_value_string(),
            ));
        }
        if self.select_unlocked_cells.has_value() {
            attributes.push((
                "selectUnlockedCells",
                self.select_unlocked_cells.get_value_string(),
            ));
        }
        if self.sort.has_value() {
            attributes.push(("sort", self.sort.get_value_string()));
        }

        write_start_tag(writer, "sheetProtection", attributes, true);
    }
}
