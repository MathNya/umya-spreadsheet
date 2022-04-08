#[derive(Clone, Default, Debug)]
pub struct Security {
    lock_revision: bool,
    lock_structure: bool,
    lock_windows: bool,
    revisions_password: String,
    workbook_password: String,
}
impl Security {
    pub(crate) fn is_security_enabled(&self) -> bool {
        if self.lock_revision {
            true
        } else if self.lock_structure {
            true
        } else {
            self.lock_windows
        }
    }
    pub fn get_lock_revision(&self) -> &bool {
        &self.lock_revision
    }
    pub fn get_lock_structure(&self) -> &bool {
        &self.lock_structure
    }
    pub fn get_lock_windows(&self) -> &bool {
        &self.lock_windows
    }
    pub fn get_revisions_password(&self) -> &str {
        &self.revisions_password
    }
    pub fn get_workbook_password(&self) -> &str {
        &self.workbook_password
    }
}
