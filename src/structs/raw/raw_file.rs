use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::structs::WriterManager;
use crate::XlsxError;
use std::io;
use std::io::Read;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub(crate) struct RawFile {
    file_target: StringValue,
    file_data: ThinVec<u8>,
}
impl RawFile {
    #[inline]
    pub(crate) fn get_file_name(&self) -> &str {
        let v: Vec<&str> = self.get_file_target().split('/').collect();
        let object_name = v.last().unwrap();
        *object_name
    }

    #[inline]
    pub(crate) fn make_rel_name(&self) -> String {
        format!("_rels/{}.rels", self.get_file_name())
    }

    #[inline]
    pub(crate) fn get_path(&self) -> String {
        let mut v: Vec<&str> = self.get_file_target().split('/').collect();
        v.pop();
        v.join("/")
    }

    #[inline]
    pub(crate) fn get_extension(&self) -> String {
        self.get_file_name()
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_lowercase())
            .unwrap()
    }

    #[inline]
    pub(crate) fn get_file_target(&self) -> &str {
        self.file_target.get_value_str()
    }

    #[inline]
    pub(crate) fn set_file_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.file_target.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn get_file_data(&self) -> &[u8] {
        &self.file_data
    }

    #[inline]
    pub(crate) fn _get_file_data_mut(&mut self) -> &mut ThinVec<u8> {
        &mut self.file_data
    }

    #[inline]
    pub(crate) fn set_file_data(&mut self, value: &[u8]) -> &mut Self {
        self.file_data = value.into();
        self
    }

    pub(crate) fn set_attributes<R: io::Read + io::Seek>(
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
