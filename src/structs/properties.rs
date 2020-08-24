#[derive(Debug)]
pub struct Properties {
    creator: String,
    last_modified_by: String,
    created: String,
    modified: String,
    title: String,
    description: String,
    subject: String,
    keywords: String,
    category: String,
    manager: String,
    company: String,
    revision: String,
    version: String,
    custom_properties: Vec<String>,
}
impl Default for Properties {
    fn default() -> Self {
        Self {
            creator: String::from(""),
            last_modified_by: String::from(""),
            created: String::from("2006-09-16T00:00:00Z"),
            modified: String::from("2006-09-16T00:00:00Z"),
            title: String::from(""),
            description: String::from(""),
            subject: String::from(""),
            keywords: String::from(""),
            category: String::from(""),
            manager: String::from(""),
            company: String::from(""),
            revision: String::from(""),
            version: String::from(""),
            custom_properties: Vec::new()
        }
    }
}
impl Properties {
    pub fn get_creator(&self) -> &str {
        &self.creator
    }
    pub(crate) fn set_creator<S: Into<String>>(&mut self, value:S) {
        self.creator = value.into();
    }
    pub fn get_last_modified_by(&self) -> &str {
        &self.last_modified_by
    }
    pub(crate) fn set_last_modified_by<S: Into<String>>(&mut self, value:S) {
        self.last_modified_by = value.into();
    }
    pub fn get_created(&self) -> &str {
        &self.created
    }
    pub(crate) fn set_created<S: Into<String>>(&mut self, value:S) {
        self.created = value.into();
    }
    pub fn get_modified(&self) -> &str {
        &self.modified
    }
    pub(crate) fn set_modified<S: Into<String>>(&mut self, value:S) {
        self.modified = value.into();
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub(crate) fn set_title<S: Into<String>>(&mut self, value:S) {
        self.title = value.into();
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub(crate) fn set_description<S: Into<String>>(&mut self, value:S) {
        self.description = value.into();
    }
    pub fn get_subject(&self) -> &str {
        &self.subject
    }
    pub(crate) fn set_subject<S: Into<String>>(&mut self, value:S) {
        self.subject = value.into();
    }
    pub fn get_keywords(&self) -> &str {
        &self.keywords
    }
    pub(crate) fn set_keywords<S: Into<String>>(&mut self, value:S) {
        self.keywords = value.into();
    }
    pub fn get_revision(&self) -> &str {
        &self.revision
    }
    pub(crate) fn set_revision<S: Into<String>>(&mut self, value:S) {
        self.revision = value.into();
    }
    pub fn get_category(&self) -> &str {
        &self.category
    }
    pub(crate) fn set_category<S: Into<String>>(&mut self, value:S) {
        self.category = value.into();
    }
    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub(crate) fn set_version<S: Into<String>>(&mut self, value:S)
    {
        self.version = value.into();
    }
    pub fn get_manager(&self) -> &str {
        &self.manager
    }
    pub(crate) fn set_manager<S: Into<String>>(&mut self, value:S) {
        self.manager = value.into();
    }
    pub fn get_company(&self) -> &str {
        &self.company
    }
    pub(crate) fn set_company<S: Into<String>>(&mut self, value:S) {
        self.company = value.into();
    }
    pub fn get_custom_properties(&self) -> &Vec<String> {
        &self.custom_properties
    }
    pub(crate) fn set_custom_properties(&mut self, value:Vec<String>) {
        self.custom_properties = value;
    }
}