use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io;
use std::io::Cursor;
use structs::raw::RawFile;
use structs::StringValue;
use structs::WriterManager;
use writer::driver::*;
use writer::xlsx::XlsxError;

#[derive(Clone, Debug, Default)]
pub(crate) struct RawRelationship {
    id: StringValue,
    r_type: StringValue,
    target: StringValue,
    raw_file: RawFile,
    target_mode: StringValue,
}
impl RawRelationship {
    pub(crate) fn get_id(&self) -> &str {
        self.id.get_value()
    }

    pub(crate) fn set_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.id.set_value(value);
        self
    }

    pub(crate) fn get_target(&self) -> &str {
        self.target.get_value()
    }

    pub(crate) fn set_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.target.set_value(value);
        self
    }

    pub(crate) fn get_type(&self) -> &str {
        self.r_type.get_value()
    }

    pub(crate) fn set_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.r_type.set_value(value);
        self
    }

    pub(crate) fn get_raw_file(&self) -> &RawFile {
        &self.raw_file
    }

    pub(crate) fn get_raw_file_mut(&mut self) -> &mut RawFile {
        &mut self.raw_file
    }

    pub(crate) fn _set_raw_file(&mut self, value: RawFile) -> &mut Self {
        self.raw_file = value;
        self
    }

    pub(crate) fn get_target_mode(&self) -> &str {
        self.target_mode.get_value()
    }

    pub(crate) fn set_target_mode<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.target_mode.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: io::Read + io::Seek>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        base_path: &str,
    ) {
        self.set_id(get_attribute(e, b"Id").unwrap());
        self.set_type(get_attribute(e, b"Type").unwrap());
        self.set_target(get_attribute(e, b"Target").unwrap());
        match get_attribute(e, b"TargetMode") {
            Some(v) => {
                self.set_target_mode(v);
            }
            None => {}
        }
        if self.get_target_mode() != "External" {
            let target = self.get_target().to_string();
            self.get_raw_file_mut()
                .set_attributes(arv, base_path, &target);
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("Id", self.get_id()));
        attributes.push(("Type", self.get_type()));
        attributes.push(("Target", self.get_target()));
        if self.get_target_mode() != "" {
            attributes.push(("TargetMode", self.get_target_mode()));
        }
        write_start_tag(writer, "Relationship", attributes, true);
    }

    pub(crate) fn write_to_bin<W: io::Seek + io::Write>(
        &self,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        self.get_raw_file().write_to(writer_mng)
    }
}
