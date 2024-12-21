use crate::helper::const_str::{
    COREPROPS_NS, DCMITYPE_NS, DCORE_NS, DCTERMS_NS, VTYPES_NS, XPROPS_NS, XSI_NS,
};
use crate::reader::driver::xml_read_loop;
use crate::structs::custom_properties::Properties as CustomProperties;
use crate::structs::StringValue;
use crate::structs::Worksheet;
use crate::writer::driver::{write_end_tag, write_start_tag, write_text_node};
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct Properties {
    creator: StringValue,
    last_modified_by: StringValue,
    created: StringValue,
    modified: StringValue,
    title: StringValue,
    description: StringValue,
    subject: StringValue,
    keywords: StringValue,
    category: StringValue,
    manager: StringValue,
    company: StringValue,
    revision: StringValue,
    version: StringValue,
    custom_properties: CustomProperties,
}
impl Default for Properties {
    #[inline]
    fn default() -> Self {
        let mut created = StringValue::default();
        let mut modified = StringValue::default();
        created.set_value("2006-09-16T00:00:00Z");
        modified.set_value("2006-09-16T00:00:00Z");
        Self {
            creator: StringValue::default(),
            last_modified_by: StringValue::default(),
            created,
            modified,
            title: StringValue::default(),
            description: StringValue::default(),
            subject: StringValue::default(),
            keywords: StringValue::default(),
            category: StringValue::default(),
            manager: StringValue::default(),
            company: StringValue::default(),
            revision: StringValue::default(),
            version: StringValue::default(),
            custom_properties: CustomProperties::default(),
        }
    }
}
impl Properties {
    #[inline]
    #[must_use]
    pub fn get_creator(&self) -> &str {
        self.creator.get_value_str()
    }

    #[inline]
    pub fn set_creator<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.creator.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_last_modified_by(&self) -> &str {
        self.last_modified_by.get_value_str()
    }

    #[inline]
    pub fn set_last_modified_by<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.last_modified_by.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_created(&self) -> &str {
        self.created.get_value_str()
    }

    #[inline]
    pub fn set_created<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.created.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_modified(&self) -> &str {
        self.modified.get_value_str()
    }

    #[inline]
    pub fn set_modified<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.modified.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_title(&self) -> &str {
        self.title.get_value_str()
    }

    #[inline]
    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_description(&self) -> &str {
        self.description.get_value_str()
    }

    #[inline]
    pub fn set_description<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.description.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_subject(&self) -> &str {
        self.subject.get_value_str()
    }

    #[inline]
    pub fn set_subject<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.subject.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_keywords(&self) -> &str {
        self.keywords.get_value_str()
    }

    #[inline]
    pub fn set_keywords<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.keywords.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_revision(&self) -> &str {
        self.revision.get_value_str()
    }

    #[inline]
    pub fn set_revision<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.revision.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_category(&self) -> &str {
        self.category.get_value_str()
    }

    #[inline]
    pub fn set_category<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.category.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_version(&self) -> &str {
        self.version.get_value_str()
    }

    #[inline]
    pub fn set_version<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_manager(&self) -> &str {
        self.manager.get_value_str()
    }

    #[inline]
    pub fn set_manager<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.manager.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_company(&self) -> &str {
        self.company.get_value_str()
    }

    #[inline]
    pub fn set_company<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.company.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_custom_properties(&self) -> &CustomProperties {
        &self.custom_properties
    }

    #[inline]
    pub fn get_custom_properties_mut(&mut self) -> &mut CustomProperties {
        &mut self.custom_properties
    }

    #[inline]
    pub fn set_custom_properties(&mut self, value: CustomProperties) -> &mut Self {
        self.custom_properties = value;
        self
    }

    pub(crate) fn set_attributes_core<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::new();
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                value = e.unescape().unwrap().to_string();
            },
            Event::End(ref e) => match e.name().into_inner() {
                b"dc:title" => {self.set_title(std::mem::take(&mut value));},
                b"dc:subject" => {self.set_subject(std::mem::take(&mut value));},
                b"dc:creator" => {self.set_creator(std::mem::take(&mut value));},
                b"cp:keywords" => {self.set_keywords(std::mem::take(&mut value));},
                b"dc:description" => {self.set_description(std::mem::take(&mut value));},
                b"cp:lastModifiedBy" => {self.set_last_modified_by(std::mem::take(&mut value));},
                b"cp:revision" => {self.set_revision(std::mem::take(&mut value));},
                b"dcterms:created" => {self.set_created(std::mem::take(&mut value));},
                b"dcterms:modified" => {self.set_modified(std::mem::take(&mut value));},
                b"cp:category" => {self.set_category(std::mem::take(&mut value));},
                b"cp:version" => {self.set_version(std::mem::take(&mut value));},
                b"Manager" => {self.set_manager(std::mem::take(&mut value));},
                b"Company" => {self.set_company(std::mem::take(&mut value));},
                _ => {}
            },
            Event::Eof => return,
        );
    }

    pub(crate) fn set_attributes_app<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::new();
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner(){
                    b"Manager" => {value = String::new();},
                    b"Company" => {value = String::new();},
                    _ => {}
                }
            },
            Event::Text(e) => {
                value = e.unescape().unwrap().to_string();
            },
            Event::End(ref e) => match e.name().into_inner() {
                b"Manager" => {self.set_manager(std::mem::take(&mut value));}
                b"Company" => {self.set_company(std::mem::take(&mut value));}
                _ =>{}
            },
            Event::Eof => return,
        );
    }

    #[inline]
    pub(crate) fn set_attributes_custom<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut obj = CustomProperties::default();
        obj.set_attributes(reader, e);
        self.set_custom_properties(obj);
    }

    pub(crate) fn write_to_core(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cp:coreProperties
        write_start_tag(
            writer,
            "cp:coreProperties",
            vec![
                ("xmlns:cp", COREPROPS_NS),
                ("xmlns:dc", DCORE_NS),
                ("xmlns:dcterms", DCTERMS_NS),
                ("xmlns:dcmitype", DCMITYPE_NS),
                ("xmlns:xsi", XSI_NS),
            ],
            false,
        );

        // dc:title
        if self.title.has_value() {
            write_start_tag(writer, "dc:title", vec![], false);
            write_text_node(writer, self.title.get_value_str());
            write_end_tag(writer, "dc:title");
        }

        // dc:subject
        if self.subject.has_value() {
            write_start_tag(writer, "dc:subject", vec![], false);
            write_text_node(writer, self.subject.get_value_str());
            write_end_tag(writer, "dc:subject");
        }

        // dc:creator
        if self.creator.has_value() {
            write_start_tag(writer, "dc:creator", vec![], false);
            write_text_node(writer, self.creator.get_value_str());
            write_end_tag(writer, "dc:creator");
        }

        // cp:keywords
        if self.keywords.has_value() {
            write_start_tag(writer, "cp:keywords", vec![], false);
            write_text_node(writer, self.keywords.get_value_str());
            write_end_tag(writer, "cp:keywords");
        }

        // dc:description
        if self.description.has_value() {
            write_start_tag(writer, "dc:description", vec![], false);
            write_text_node(writer, self.description.get_value_str());
            write_end_tag(writer, "dc:description");
        }

        // cp:lastModifiedBy
        if self.last_modified_by.has_value() {
            write_start_tag(writer, "cp:lastModifiedBy", vec![], false);
            write_text_node(writer, self.last_modified_by.get_value_str());
            write_end_tag(writer, "cp:lastModifiedBy");
        }

        // cp:revision
        if self.revision.has_value() {
            write_start_tag(writer, "cp:revision", vec![], false);
            write_text_node(writer, self.revision.get_value_str());
            write_end_tag(writer, "cp:revision");
        }

        // dcterms:created
        if self.created.has_value() {
            write_start_tag(
                writer,
                "dcterms:created",
                vec![("xsi:type", "dcterms:W3CDTF")],
                false,
            );
            write_text_node(writer, self.created.get_value_str());
            write_end_tag(writer, "dcterms:created");
        }

        // dcterms:modified
        if self.modified.has_value() {
            write_start_tag(
                writer,
                "dcterms:modified",
                vec![("xsi:type", "dcterms:W3CDTF")],
                false,
            );
            write_text_node(writer, self.modified.get_value_str());
            write_end_tag(writer, "dcterms:modified");
        }

        // cp:category
        if self.category.has_value() {
            write_start_tag(writer, "cp:category", vec![], false);
            write_text_node(writer, self.category.get_value_str());
            write_end_tag(writer, "cp:category");
        }

        // cp:version
        if self.version.has_value() {
            write_start_tag(writer, "cp:version", vec![], false);
            write_text_node(writer, self.version.get_value_str());
            write_end_tag(writer, "cp:version");
        }

        write_end_tag(writer, "cp:coreProperties");
    }

    pub(crate) fn write_to_app(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        work_sheet_collection: &[Worksheet],
    ) {
        let sheet_count_str = work_sheet_collection.len().to_string();

        // Properties
        write_start_tag(
            writer,
            "Properties",
            vec![("xmlns", XPROPS_NS), ("xmlns:vt", VTYPES_NS)],
            false,
        );

        // Application
        write_start_tag(writer, "Application", vec![], false);
        write_text_node(writer, "Microsoft Excel");
        write_end_tag(writer, "Application");

        // DocSecurity
        write_start_tag(writer, "DocSecurity", vec![], false);
        write_text_node(writer, "0");
        write_end_tag(writer, "DocSecurity");

        // ScaleCrop
        write_start_tag(writer, "ScaleCrop", vec![], false);
        write_text_node(writer, "false");
        write_end_tag(writer, "ScaleCrop");

        // HeadingPairs
        write_start_tag(writer, "HeadingPairs", vec![], false);

        // vt:vector
        write_start_tag(
            writer,
            "vt:vector",
            vec![("size", "2"), ("baseType", "variant")],
            false,
        );

        // vt:variant
        write_start_tag(writer, "vt:variant", vec![], false);

        // vt:i4
        write_start_tag(writer, "vt:lpstr", vec![], false);
        write_text_node(writer, "Worksheets");
        write_end_tag(writer, "vt:lpstr");

        write_end_tag(writer, "vt:variant");

        // vt:variant
        write_start_tag(writer, "vt:variant", vec![], false);

        // vt:i4
        write_start_tag(writer, "vt:i4", vec![], false);
        write_text_node(writer, &sheet_count_str);
        write_end_tag(writer, "vt:i4");

        write_end_tag(writer, "vt:variant");

        write_end_tag(writer, "vt:vector");

        write_end_tag(writer, "HeadingPairs");

        // TitlesOfParts
        write_start_tag(writer, "TitlesOfParts", vec![], false);

        // vt:vector
        write_start_tag(
            writer,
            "vt:vector",
            vec![("size", &sheet_count_str), ("baseType", "lpstr")],
            false,
        );

        for workseet in work_sheet_collection {
            // vt:lpstr
            write_start_tag(writer, "vt:lpstr", vec![], false);
            write_text_node(writer, workseet.get_name());
            write_end_tag(writer, "vt:lpstr");
        }

        write_end_tag(writer, "vt:vector");

        write_end_tag(writer, "TitlesOfParts");

        // Manager
        write_start_tag(writer, "Manager", vec![], false);
        write_text_node(writer, self.get_manager());
        write_end_tag(writer, "Manager");

        // Company
        write_start_tag(writer, "Company", vec![], false);
        write_text_node(writer, self.get_company());
        write_end_tag(writer, "Company");

        // LinksUpToDate
        write_start_tag(writer, "LinksUpToDate", vec![], false);
        write_text_node(writer, "false");
        write_end_tag(writer, "LinksUpToDate");

        // SharedDoc
        write_start_tag(writer, "SharedDoc", vec![], false);
        write_text_node(writer, "false");
        write_end_tag(writer, "SharedDoc");

        // HyperlinksChanged
        write_start_tag(writer, "HyperlinksChanged", vec![], false);
        write_text_node(writer, "false");
        write_end_tag(writer, "HyperlinksChanged");

        // AppVersion
        write_start_tag(writer, "AppVersion", vec![], false);
        write_text_node(writer, "14.0300");
        write_end_tag(writer, "AppVersion");

        write_end_tag(writer, "Properties");
    }

    #[inline]
    pub(crate) fn write_to_custom(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.custom_properties.write_to(writer);
    }
}
