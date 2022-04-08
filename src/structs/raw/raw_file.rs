use reader::driver::*;
use std::io;
use std::io::Read;
use structs::StringValue;
use structs::WriterManager;
use writer::xlsx::XlsxError;

#[derive(Clone, Default, Debug)]
pub(crate) struct RawFile {
    file_target: StringValue,
    file_data: Vec<u8>,
}
impl RawFile {
    pub(crate) fn get_file_name(&self) -> String {
        let v: Vec<&str> = self.get_file_target().split('/').collect();
        let object_name = v.last().unwrap().clone();
        object_name.to_string()
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
        let file_name = self.get_file_name();
        let v: Vec<&str> = file_name.split('.').collect();
        let extension = v.last().unwrap().clone();

        extension.to_lowercase()
    }

    pub(crate) fn get_file_target(&self) -> &str {
        self.file_target.get_value()
    }

    pub(crate) fn set_file_target<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.file_target.set_value(value);
        self
    }

    pub(crate) fn get_file_data(&self) -> &Vec<u8> {
        &self.file_data
    }

    pub(crate) fn _get_file_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.file_data
    }

    pub(crate) fn set_file_data(&mut self, value: Vec<u8>) -> &mut Self {
        self.file_data = value;
        self
    }

    pub(crate) fn set_attributes<R: io::Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
    ) {
        let path_str = normalize_path_to_str(&format!("{}/{}", base_path, target));
        let mut r = io::BufReader::new(arv.by_name(&path_str).unwrap());
        let mut buf = Vec::new();
        r.read_to_end(&mut buf).unwrap();

        self.set_file_target(path_str);
        self.set_file_data(buf);
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
