#[derive(Clone, Debug)]
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
            custom_properties: Vec::new(),
        }
    }
}
impl Properties {
    pub fn get_creator(&self) -> &str {
        &self.creator
    }

    pub fn set_creator<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.creator = value.into();
        self
    }

    pub fn get_last_modified_by(&self) -> &str {
        &self.last_modified_by
    }

    pub fn set_last_modified_by<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.last_modified_by = value.into();
        self
    }

    pub fn get_created(&self) -> &str {
        &self.created
    }

    pub fn set_created<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.created = value.into();
        self
    }

    pub fn get_modified(&self) -> &str {
        &self.modified
    }

    pub fn set_modified<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.modified = value.into();
        self
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.title = value.into();
        self
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.description = value.into();
        self
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn set_subject<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.subject = value.into();
        self
    }

    pub fn get_keywords(&self) -> &str {
        &self.keywords
    }

    pub fn set_keywords<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.keywords = value.into();
        self
    }

    pub fn get_revision(&self) -> &str {
        &self.revision
    }

    pub fn set_revision<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.revision = value.into();
        self
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }

    pub fn set_category<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.category = value.into();
        self
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn set_version<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.version = value.into();
        self
    }

    pub fn get_manager(&self) -> &str {
        &self.manager
    }

    pub fn set_manager<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.manager = value.into();
        self
    }

    pub fn get_company(&self) -> &str {
        &self.company
    }

    pub fn set_company<S: Into<String>>(&mut self, value: S) -> &mut Properties {
        self.company = value.into();
        self
    }

    pub fn get_custom_properties(&self) -> &Vec<String> {
        &self.custom_properties
    }

    pub fn set_custom_properties(&mut self, value: Vec<String>) -> &mut Properties {
        self.custom_properties = value;
        self
    }
}
