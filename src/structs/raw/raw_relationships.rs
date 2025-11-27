use std::{
    io,
    io::Read,
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use crate::{
    helper::const_str::REL_NS,
    reader::driver::{
        join_paths,
        xml_read_loop,
    },
    structs::{
        StringValue,
        WriterManager,
        XlsxError,
        raw::RawRelationship,
    },
    writer::driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};

#[derive(Clone, Debug, Default)]
pub(crate) struct RawRelationships {
    file_target:       StringValue,
    relationship_list: Vec<RawRelationship>,
}

impl RawRelationships {
    #[inline]
    pub(crate) fn file_name(&self) -> String {
        let v: Vec<&str> = self.file_target().split('/').collect();
        let object_name = v.last().unwrap();
        (*object_name).to_string()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use file_name()")]
    pub(crate) fn get_file_name(&self) -> String {
        self.file_name()
    }

    #[inline]
    pub(crate) fn file_target(&self) -> &str {
        self.file_target.value_str()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use file_target()")]
    pub(crate) fn get_file_target(&self) -> &str {
        self.file_target()
    }

    #[inline]
    pub(crate) fn set_file_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.file_target.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn relationship_list(&self) -> &[RawRelationship] {
        &self.relationship_list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use relationship_list()")]
    pub(crate) fn get_relationship_list(&self) -> &[RawRelationship] {
        self.relationship_list()
    }

    #[inline]
    pub(crate) fn relationship_list_mut(&mut self) -> &mut Vec<RawRelationship> {
        &mut self.relationship_list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use relationship_list_mut()")]
    pub(crate) fn get_relationship_list_mut(&mut self) -> &mut Vec<RawRelationship> {
        self.relationship_list_mut()
    }

    pub(crate) fn relationship_by_rid(&self, r_id: &str) -> &RawRelationship {
        self.relationship_list()
            .iter()
            .find(|relationship| relationship.id() == r_id)
            .unwrap_or_else(|| panic!("Not found relationship with ID: {r_id}."))
    }

    #[deprecated(since = "3.0.0", note = "Use relationship_by_rid()")]
    pub(crate) fn get_relationship_by_rid(&self, r_id: &str) -> &RawRelationship {
        self.relationship_by_rid(r_id)
    }

    #[inline]
    pub(crate) fn add_relationship_list(&mut self, value: RawRelationship) -> &mut Self {
        self.relationship_list.push(value);
        self
    }

    pub(crate) fn set_attributes<R: Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
    ) -> bool {
        let data = {
            let path_str = join_paths(base_path, target);
            let Ok(file_path) = arv.by_name(&path_str) else {
                return false;
            };
            self.set_file_target(path_str);
            let mut r = io::BufReader::new(file_path);
            let mut buf = Vec::new();
            r.read_to_end(&mut buf).unwrap();
            io::Cursor::new(buf)
        };
        let mut reader = Reader::from_reader(data);
        reader.config_mut().trim_text(true);

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"Relationship" {
                    let mut obj = RawRelationship::default();
                    obj.set_attributes(&mut reader, e, arv, base_path);
                    self.add_relationship_list(obj);
                }
            },
            Event::Eof => break
        );

        true
    }

    pub(crate) fn write_to<W: io::Seek + io::Write>(
        &self,
        writer_mng: &mut WriterManager<W>,
        ather_target: Option<&str>,
    ) -> Result<(), XlsxError> {
        if self.relationship_list().is_empty() {
            return Ok(());
        }

        let mut writer = Writer::new(io::Cursor::new(Vec::new()));
        // XML header
        writer
            .write_event(Event::Decl(BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                Some("yes"),
            )))
            .unwrap();
        write_new_line(&mut writer);

        // relationships
        write_start_tag(
            &mut writer,
            "Relationships",
            vec![("xmlns", REL_NS).into()],
            false,
        );

        for relationship in self.relationship_list() {
            relationship.write_to(&mut writer);
        }

        write_end_tag(&mut writer, "Relationships");

        let target = match ather_target {
            Some(v) => v,
            None => self.file_target(),
        };
        writer_mng.add_writer(target, writer)?;

        for relationship in self.relationship_list() {
            relationship.write_to_bin(writer_mng)?;
        }

        Ok(())
    }
}
