use std::{
    io,
    io::Read,
};

use crate::{
    XlsxError,
    reader::driver::join_paths,
    structs::{
        StringValue,
        WriterManager,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct RawFile {
    file_target: StringValue,
    file_data:   Vec<u8>,
}
impl RawFile {
    pub(crate) fn get_file_name(&self) -> String {
        let v: Vec<&str> = self.get_file_target().split('/').collect();
        let object_name = v.last().unwrap();
        (*object_name).to_string()
    }

    pub(crate) fn make_rel_name(&self) -> String {
        format!("_rels/{}.rels", self.get_file_name())
    }

    pub(crate) fn get_path(&self) -> String {
        let mut v: Vec<&str> = self.get_file_target().split('/').collect();
        v.pop();
        v.join("/")
    }

    pub(crate) fn get_extension(&self) -> String {
        self.get_file_name()
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_lowercase())
            .unwrap()
    }

    pub(crate) fn get_file_target(&self) -> &str {
        self.file_target.get_value_str()
    }

    pub(crate) fn set_file_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.file_target.set_value(value);
        self
    }

    pub(crate) fn get_file_data(&self) -> &[u8] {
        &self.file_data
    }

    pub(crate) fn get_file_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.file_data
    }

    pub(crate) fn set_file_data(&mut self, value: &[u8]) -> &mut Self {
        self.file_data = value.into();
        self
    }

    pub(crate) fn set_attributes<R: Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
    ) {
        let path_str = join_paths(base_path, target);
        let mut r = io::BufReader::new(arv.by_name(&path_str).unwrap());
        let mut buf = Vec::new();
        r.read_to_end(&mut buf).unwrap();

        self.set_file_target(path_str);
        self.set_file_data(&buf);
    }

    pub(crate) fn write_to<W: io::Seek + io::Write>(
        &self,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        if !self.get_file_data().is_empty() {
            writer_mng.add_bin(self.get_file_target(), self.get_file_data())?;
        }
        Ok(())
    }
}
