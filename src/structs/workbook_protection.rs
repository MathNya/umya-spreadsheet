// workbookProtection
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
    helper::crypt::{
        encrypt_revisions_protection,
        encrypt_workbook_protection,
    },
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct WorkbookProtection {
    workbook_algorithm_name:  StringValue,
    workbook_hash_value:      StringValue,
    workbook_salt_value:      StringValue,
    workbook_spin_count:      UInt32Value,
    workbook_password:        StringValue,
    revisions_algorithm_name: StringValue,
    revisions_hash_value:     StringValue,
    revisions_salt_value:     StringValue,
    revisions_spin_count:     UInt32Value,
    revisions_password:       StringValue,
    lock_revision:            BooleanValue,
    lock_structure:           BooleanValue,
    lock_windows:             BooleanValue,
}
impl WorkbookProtection {
    #[inline]
    #[must_use]
    pub fn get_workbook_algorithm_name(&self) -> &str {
        self.workbook_algorithm_name.value_str()
    }

    #[inline]
    pub fn set_workbook_algorithm_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_algorithm_name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_workbook_hash_value(&self) -> &str {
        self.workbook_hash_value.value_str()
    }

    #[inline]
    pub fn set_workbook_hash_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_hash_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_workbook_salt_value(&self) -> &str {
        self.workbook_salt_value.value_str()
    }

    #[inline]
    pub fn set_workbook_salt_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_salt_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_workbook_spin_count(&self) -> u32 {
        self.workbook_spin_count.value()
    }

    #[inline]
    pub fn set_workbook_spin_count(&mut self, value: u32) -> &mut Self {
        self.workbook_spin_count.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_workbook_password_raw(&self) -> &str {
        self.workbook_password.value_str()
    }

    #[inline]
    pub fn set_workbook_password_raw<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_password.set_value(value);
        self
    }

    #[inline]
    pub fn remove_workbook_password_raw(&mut self) -> &mut Self {
        self.workbook_password.remove_value();
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revisions_algorithm_name(&self) -> &str {
        self.revisions_algorithm_name.value_str()
    }

    #[inline]
    pub fn set_revisions_algorithm_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_algorithm_name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revisions_hash_value(&self) -> &str {
        self.revisions_hash_value.value_str()
    }

    #[inline]
    pub fn set_revisions_hash_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_hash_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revisions_salt_value(&self) -> &str {
        self.revisions_salt_value.value_str()
    }

    #[inline]
    pub fn set_revisions_salt_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_salt_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revisions_spin_count(&self) -> u32 {
        self.revisions_spin_count.value()
    }

    #[inline]
    pub fn set_revisions_spin_count(&mut self, value: u32) -> &mut Self {
        self.revisions_spin_count.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revisions_password_raw(&self) -> &str {
        self.revisions_password.value_str()
    }

    #[inline]
    pub fn set_revisions_password_raw<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_password.set_value(value);
        self
    }

    #[inline]
    pub fn remove_revisions_password_raw(&mut self) -> &mut Self {
        self.revisions_password.remove_value();
        self
    }

    #[inline]
    #[must_use]
    pub fn get_lock_revision(&self) -> bool {
        self.lock_revision.get_value()
    }

    #[inline]
    pub fn set_lock_revision(&mut self, value: bool) -> &mut Self {
        self.lock_revision.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_lock_structure(&self) -> bool {
        self.lock_structure.get_value()
    }

    #[inline]
    pub fn set_lock_structure(&mut self, value: bool) -> &mut Self {
        self.lock_structure.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_lock_windows(&self) -> bool {
        self.lock_windows.get_value()
    }

    #[inline]
    pub fn set_lock_windows(&mut self, value: bool) -> &mut Self {
        self.lock_windows.set_value(value);
        self
    }

    #[inline]
    pub fn set_workbook_password(&mut self, password: &str) -> &mut Self {
        encrypt_workbook_protection(password, self);
        self
    }

    #[inline]
    pub fn set_revisions_password(&mut self, password: &str) -> &mut Self {
        encrypt_revisions_protection(password, self);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, workbook_algorithm_name, "workbookAlgorithmName");
        set_string_from_xml!(self, e, workbook_hash_value, "workbookHashValue");
        set_string_from_xml!(self, e, workbook_salt_value, "workbookSaltValue");
        set_string_from_xml!(self, e, workbook_spin_count, "workbookSpinCount");
        set_string_from_xml!(self, e, workbook_password, "workbookPassword");
        set_string_from_xml!(self, e, revisions_algorithm_name, "revisionsAlgorithmName");
        set_string_from_xml!(self, e, revisions_hash_value, "revisionsHashValue");
        set_string_from_xml!(self, e, revisions_salt_value, "revisionsSaltValue");
        set_string_from_xml!(self, e, revisions_spin_count, "revisionsSpinCount");
        set_string_from_xml!(self, e, revisions_password, "revisionsPassword");
        set_string_from_xml!(self, e, lock_revision, "lockRevision");
        set_string_from_xml!(self, e, lock_structure, "lockStructure");
        set_string_from_xml!(self, e, lock_windows, "lockWindows");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // workbookProtection
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.workbook_algorithm_name.has_value() {
            attributes.push(
                (
                    "workbookAlgorithmName",
                    self.workbook_algorithm_name.value_str(),
                )
                    .into(),
            );
        }
        if self.workbook_hash_value.has_value() {
            attributes.push(
                (
                    "workbookHashValue",
                    self.workbook_hash_value.value_str(),
                )
                    .into(),
            );
        }
        if self.workbook_salt_value.has_value() {
            attributes.push(
                (
                    "workbookSaltValue",
                    self.workbook_salt_value.value_str(),
                )
                    .into(),
            );
        }
        let workbook_spin_count = self.workbook_spin_count.value_string();
        if self.workbook_spin_count.has_value() {
            attributes.push(("workbookSpinCount", &workbook_spin_count).into());
        }
        if self.workbook_password.has_value() {
            attributes.push(("workbookPassword", self.workbook_password.value_str()).into());
        }
        if self.revisions_algorithm_name.has_value() {
            attributes.push(
                (
                    "revisionsAlgorithmName",
                    self.revisions_algorithm_name.value_str(),
                )
                    .into(),
            );
        }
        if self.revisions_hash_value.has_value() {
            attributes.push(
                (
                    "revisionsHashValue",
                    self.revisions_hash_value.value_str(),
                )
                    .into(),
            );
        }
        if self.revisions_salt_value.has_value() {
            attributes.push(
                (
                    "revisionsSaltValue",
                    self.revisions_salt_value.value_str(),
                )
                    .into(),
            );
        }
        let revisions_spin_count = self.revisions_spin_count.value_string();
        if self.revisions_spin_count.has_value() {
            attributes.push(("revisionsSpinCount", &revisions_spin_count).into());
        }
        if self.revisions_password.has_value() {
            attributes.push(("revisionsPassword", self.revisions_password.value_str()).into());
        }
        if self.lock_revision.has_value() {
            attributes.push(("lockRevision", self.lock_revision.get_value_string()).into());
        }
        if self.lock_structure.has_value() {
            attributes.push(("lockStructure", self.lock_structure.get_value_string()).into());
        }
        if self.lock_windows.has_value() {
            attributes.push(("lockWindows", self.lock_windows.get_value_string()).into());
        }

        write_start_tag(writer, "workbookProtection", attributes, true);
    }
}
