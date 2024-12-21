// pivotCacheDefinition
use crate::helper::const_str::{MC_NS, REL_OFC_NS, SHEET_MAIN_NS, SHEET_MS_REVISION_NS};
use crate::reader::driver::{get_attribute, set_string_from_xml, xml_read_loop};
use crate::structs::ByteValue;
use crate::structs::CacheFields;
use crate::structs::CacheSource;
use crate::structs::DoubleValue;
use crate::structs::StringValue;
use crate::structs::UInt32Value;
use crate::writer::driver::{write_end_tag, write_start_tag};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct PivotCacheDefinition {
    id: StringValue,
    refreshed_by: StringValue,
    refreshed_date: DoubleValue,
    created_version: ByteValue,
    refreshed_version: ByteValue,
    min_refreshable_version: ByteValue,
    record_count: UInt32Value,
    cache_source: CacheSource,
    cache_fields: CacheFields,
}

impl PivotCacheDefinition {
    #[inline]
    #[must_use]
    pub fn get_id(&self) -> &str {
        self.id.get_value_str()
    }

    #[inline]
    pub fn set_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_refreshed_by(&self) -> &str {
        self.refreshed_by.get_value_str()
    }

    #[inline]
    pub fn set_refreshed_by<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.refreshed_by.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_refreshed_date(&self) -> f64 {
        self.refreshed_date.get_value()
    }

    #[inline]
    pub fn set_refreshed_date(&mut self, value: f64) -> &mut Self {
        self.refreshed_date.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_created_version(&self) -> u8 {
        self.created_version.get_value()
    }

    #[inline]
    pub fn set_created_version(&mut self, value: u8) -> &mut Self {
        self.created_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_refreshed_version(&self) -> u8 {
        self.refreshed_version.get_value()
    }

    #[inline]
    pub fn set_refreshed_version(&mut self, value: u8) -> &mut Self {
        self.refreshed_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_min_refreshable_version(&self) -> u8 {
        self.min_refreshable_version.get_value()
    }

    #[inline]
    pub fn set_min_refreshable_version(&mut self, value: u8) -> &mut Self {
        self.min_refreshable_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_record_count(&self) -> u32 {
        self.record_count.get_value()
    }

    #[inline]
    pub fn set_record_count(&mut self, value: u32) -> &mut Self {
        self.record_count.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_cache_source(&self) -> &CacheSource {
        &self.cache_source
    }

    #[inline]
    pub fn get_cache_source_mut(&mut self) -> &mut CacheSource {
        &mut self.cache_source
    }

    #[inline]
    pub fn set_cache_source(&mut self, value: CacheSource) -> &mut Self {
        self.cache_source = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_cache_fields(&self) -> &CacheFields {
        &self.cache_fields
    }

    #[inline]
    pub fn get_cache_fields_mut(&mut self) -> &mut CacheFields {
        &mut self.cache_fields
    }

    #[inline]
    pub fn set_cache_fields(&mut self, value: CacheFields) -> &mut Self {
        self.cache_fields = value;
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, id, "r:id");
        set_string_from_xml!(self, e, refreshed_by, "refreshedBy");
        set_string_from_xml!(self, e, refreshed_date, "refreshedDate");
        set_string_from_xml!(self, e, created_version, "createdVersion");
        set_string_from_xml!(self, e, refreshed_version, "refreshedVersion");
        set_string_from_xml!(self, e, min_refreshable_version, "minRefreshableVersion");
        set_string_from_xml!(self, e, record_count, "recordCount");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"cacheSource" {
                    let mut obj = CacheSource::default();
                    obj.set_attributes(reader, e, true);
                    self.set_cache_source(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"cacheSource" {
                    let mut obj = CacheSource::default();
                    obj.set_attributes(reader, e, false);
                    self.set_cache_source(obj);
                }
                if e.name().into_inner() == b"cacheFields" {
                    let mut obj = CacheFields::default();
                    obj.set_attributes(reader, e);
                    self.set_cache_fields(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"pivotTableDefinition" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "pivotTableDefinition")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotTableDefinition
        let mut attributes = vec![
            ("xmlns", SHEET_MAIN_NS),
            ("xmlns:r", REL_OFC_NS),
            ("xmlns:mc", MC_NS),
            ("mc:Ignorable", "xr"),
            ("xmlns:xr", SHEET_MS_REVISION_NS),
        ];

        if self.id.has_value() {
            attributes.push(("r:id", self.id.get_value_str()));
        }
        if self.refreshed_by.has_value() {
            attributes.push(("refreshedBy", self.refreshed_by.get_value_str()));
        }
        let refreshed_date_str = self.refreshed_date.get_value_string();
        if self.refreshed_date.has_value() {
            attributes.push(("refreshedDate", refreshed_date_str.as_str()));
        }
        let created_version_str = self.created_version.get_value_string();
        if self.created_version.has_value() {
            attributes.push(("createdVersion", created_version_str.as_str()));
        }
        let refreshed_version_str = self.refreshed_version.get_value_string();
        if self.refreshed_version.has_value() {
            attributes.push(("refreshedVersion", refreshed_version_str.as_str()));
        }
        let min_refreshable_version_str = self.min_refreshable_version.get_value_string();
        if self.min_refreshable_version.has_value() {
            attributes.push((
                "minRefreshableVersion",
                min_refreshable_version_str.as_str(),
            ));
        }
        let record_count_str = self.record_count.get_value_string();
        if self.record_count.has_value() {
            attributes.push(("recordCount", record_count_str.as_str()));
        }

        write_start_tag(writer, "pivotTableDefinition", attributes, false);

        // cacheSource
        self.cache_source.write_to(writer);

        // cacheFields
        self.cache_fields.write_to(writer);

        write_end_tag(writer, "pivotTableDefinition");
    }
}
