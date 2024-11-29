// workbookProtection
use super::BooleanValue;
use super::StringValue;
use super::UInt32Value;
use helper::crypt::*;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct WorkbookProtection {
    workbook_algorithm_name: StringValue,
    workbook_hash_value: StringValue,
    workbook_salt_value: StringValue,
    workbook_spin_count: UInt32Value,
    workbook_password: StringValue,
    revisions_algorithm_name: StringValue,
    revisions_hash_value: StringValue,
    revisions_salt_value: StringValue,
    revisions_spin_count: UInt32Value,
    revisions_password: StringValue,
    lock_revision: BooleanValue,
    lock_structure: BooleanValue,
    lock_windows: BooleanValue,
}
impl WorkbookProtection {
    #[inline]
    pub fn get_workbook_algorithm_name(&self) -> &str {
        self.workbook_algorithm_name.get_value_str()
    }

    #[inline]
    pub fn set_workbook_algorithm_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_algorithm_name.set_value(value);
        self
    }

    #[inline]
    pub fn get_workbook_hash_value(&self) -> &str {
        self.workbook_hash_value.get_value_str()
    }

    #[inline]
    pub fn set_workbook_hash_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_hash_value.set_value(value);
        self
    }

    #[inline]
    pub fn get_workbook_salt_value(&self) -> &str {
        self.workbook_salt_value.get_value_str()
    }

    #[inline]
    pub fn set_workbook_salt_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.workbook_salt_value.set_value(value);
        self
    }

    #[inline]
    pub fn get_workbook_spin_count(&self) -> &u32 {
        self.workbook_spin_count.get_value()
    }

    #[inline]
    pub fn set_workbook_spin_count(&mut self, value: u32) -> &mut Self {
        self.workbook_spin_count.set_value(value);
        self
    }

    #[inline]
    pub fn get_workbook_password_raw(&self) -> &str {
        self.workbook_password.get_value_str()
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
    pub fn get_revisions_algorithm_name(&self) -> &str {
        self.revisions_algorithm_name.get_value_str()
    }

    #[inline]
    pub fn set_revisions_algorithm_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_algorithm_name.set_value(value);
        self
    }

    #[inline]
    pub fn get_revisions_hash_value(&self) -> &str {
        self.revisions_hash_value.get_value_str()
    }

    #[inline]
    pub fn set_revisions_hash_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_hash_value.set_value(value);
        self
    }

    #[inline]
    pub fn get_revisions_salt_value(&self) -> &str {
        self.revisions_salt_value.get_value_str()
    }

    #[inline]
    pub fn set_revisions_salt_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revisions_salt_value.set_value(value);
        self
    }

    #[inline]
    pub fn get_revisions_spin_count(&self) -> &u32 {
        self.revisions_spin_count.get_value()
    }

    #[inline]
    pub fn set_revisions_spin_count(&mut self, value: u32) -> &mut Self {
        self.revisions_spin_count.set_value(value);
        self
    }

    #[inline]
    pub fn get_revisions_password_raw(&self) -> &str {
        self.revisions_password.get_value_str()
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
    pub fn get_lock_revision(&self) -> &bool {
        self.lock_revision.get_value()
    }

    #[inline]
    pub fn set_lock_revision(&mut self, value: bool) -> &mut Self {
        self.lock_revision.set_value(value);
        self
    }

    #[inline]
    pub fn get_lock_structure(&self) -> &bool {
        self.lock_structure.get_value()
    }

    #[inline]
    pub fn set_lock_structure(&mut self, value: bool) -> &mut Self {
        self.lock_structure.set_value(value);
        self
    }

    #[inline]
    pub fn get_lock_windows(&self) -> &bool {
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.workbook_algorithm_name.has_value() {
            attributes.push((
                "workbookAlgorithmName",
                self.workbook_algorithm_name.get_value_str(),
            ));
        }
        if self.workbook_hash_value.has_value() {
            attributes.push((
                "workbookHashValue",
                self.workbook_hash_value.get_value_str(),
            ));
        }
        if self.workbook_salt_value.has_value() {
            attributes.push((
                "workbookSaltValue",
                self.workbook_salt_value.get_value_str(),
            ));
        }
        let workbook_spin_count = self.workbook_spin_count.get_value_string();
        if self.workbook_spin_count.has_value() {
            attributes.push(("workbookSpinCount", &workbook_spin_count));
        }
        if self.workbook_password.has_value() {
            attributes.push(("workbookPassword", self.workbook_password.get_value_str()));
        }
        if self.revisions_algorithm_name.has_value() {
            attributes.push((
                "revisionsAlgorithmName",
                self.revisions_algorithm_name.get_value_str(),
            ));
        }
        if self.revisions_hash_value.has_value() {
            attributes.push((
                "revisionsHashValue",
                self.revisions_hash_value.get_value_str(),
            ));
        }
        if self.revisions_salt_value.has_value() {
            attributes.push((
                "revisionsSaltValue",
                self.revisions_salt_value.get_value_str(),
            ));
        }
        let revisions_spin_count = self.revisions_spin_count.get_value_string();
        if self.revisions_spin_count.has_value() {
            attributes.push(("revisionsSpinCount", &revisions_spin_count));
        }
        if self.revisions_password.has_value() {
            attributes.push(("revisionsPassword", self.revisions_password.get_value_str()));
        }
        if self.lock_revision.has_value() {
            attributes.push(("lockRevision", self.lock_revision.get_value_string()));
        }
        if self.lock_structure.has_value() {
            attributes.push(("lockStructure", self.lock_structure.get_value_string()));
        }
        if self.lock_windows.has_value() {
            attributes.push(("lockWindows", self.lock_windows.get_value_string()));
        }

        write_start_tag(writer, "workbookProtection", attributes, true);
    }
}
