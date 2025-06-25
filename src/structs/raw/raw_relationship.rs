use std::{
    io,
    io::Cursor,
};

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    structs::{
        StringValue,
        WriterManager,
        XlsxError,
        raw::RawFile,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct RawRelationship {
    id:          StringValue,
    r_type:      StringValue,
    target:      StringValue,
    raw_file:    RawFile,
    target_mode: StringValue,
}

impl RawRelationship {
    #[inline]
    pub(crate) fn id(&self) -> &str {
        self.id.value_str()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use id()")]
    pub(crate) fn get_id(&self) -> &str {
        self.id()
    }

    #[inline]
    pub(crate) fn set_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn target(&self) -> &str {
        self.target.value_str()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use target()")]
    pub(crate) fn get_target(&self) -> &str {
        self.target()
    }

    #[inline]
    pub(crate) fn set_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.target.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn get_type(&self) -> &str {
        self.r_type.value_str()
    }

    #[inline]
    pub(crate) fn set_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.r_type.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn raw_file(&self) -> &RawFile {
        &self.raw_file
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use raw_file()")]
    pub(crate) fn get_raw_file(&self) -> &RawFile {
        self.raw_file()
    }

    #[inline]
    pub(crate) fn raw_file_mut(&mut self) -> &mut RawFile {
        &mut self.raw_file
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use raw_file_mut()")]
    pub(crate) fn get_raw_file_mut(&mut self) -> &mut RawFile {
        self.raw_file_mut()
    }

    #[inline]
    pub(crate) fn set_raw_file(&mut self, value: RawFile) -> &mut Self {
        self.raw_file = value;
        self
    }

    #[inline]
    pub(crate) fn target_mode(&self) -> &str {
        self.target_mode.value_str()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use target_mode()")]
    pub(crate) fn get_target_mode(&self) -> &str {
        self.target_mode()
    }

    #[inline]
    pub(crate) fn set_target_mode<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.target_mode.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: io::BufRead, A: io::Read + io::Seek>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        base_path: &str,
    ) {
        self.set_id(get_attribute(e, b"Id").unwrap());
        self.set_type(get_attribute(e, b"Type").unwrap());
        self.set_target(get_attribute(e, b"Target").unwrap());
        if let Some(v) = get_attribute(e, b"TargetMode") {
            self.set_target_mode(v);
        }
        if self.target_mode() != "External" {
            let target = self.target().to_string();
            self.raw_file_mut().set_attributes(arv, base_path, &target);
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("Id", self.id()).into());
        attributes.push(("Type", self.get_type()).into());
        attributes.push(("Target", self.target()).into());
        if self.target_mode() != "" {
            attributes.push(("TargetMode", self.target_mode()).into());
        }
        write_start_tag(writer, "Relationship", attributes, true);
    }

    #[inline]
    pub(crate) fn write_to_bin<W: io::Seek + io::Write>(
        &self,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        self.raw_file().write_to(writer_mng)
    }
}
