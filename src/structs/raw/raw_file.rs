use std::{
    fs::File,
    io,
    io::Read,
    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    XlsxError,
    reader::driver::{
        join_paths,
        zip_by_name,
    },
    structs::{
        StringValue,
        WriterManager,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct RawFile {
    file_target: StringValue,
    file_data:   Vec<u8>,
    source_file: Option<PathBuf>,
}
impl RawFile {
    #[inline]
    pub(crate) fn file_name(&self) -> &str {
        let v: Vec<&str> = self.file_target().split('/').collect();
        let object_name = v.last().unwrap();
        object_name
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use file_name()")]
    pub(crate) fn get_file_name(&self) -> &str {
        self.file_name()
    }

    #[inline]
    pub(crate) fn make_rel_name(&self) -> String {
        format!("_rels/{}.rels", self.file_name())
    }

    #[inline]
    pub(crate) fn path(&self) -> String {
        let mut v: Vec<&str> = self.file_target().split('/').collect();
        v.pop();
        v.join("/")
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use path()")]
    pub(crate) fn get_path(&self) -> String {
        self.path()
    }

    #[inline]
    pub(crate) fn extension(&self) -> String {
        self.file_name()
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_lowercase())
            .unwrap()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use extension()")]
    pub(crate) fn get_extension(&self) -> String {
        self.extension()
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
    pub(crate) fn file_data(&self) -> &[u8] {
        &self.file_data
    }

    #[inline]
    pub(crate) fn has_file_data(&self) -> bool {
        !self.file_data.is_empty()
    }

    #[inline]
    pub(crate) fn source_file(&self) -> Option<&Path> {
        self.source_file.as_deref()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use file_data()")]
    pub(crate) fn get_file_data(&self) -> &[u8] {
        self.file_data()
    }

    #[inline]
    pub(crate) fn file_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.file_data
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use file_data_mut()")]
    pub(crate) fn get_file_data_mut(&mut self) -> &mut Vec<u8> {
        self.file_data_mut()
    }

    #[inline]
    pub(crate) fn set_file_data(&mut self, value: &[u8]) -> &mut Self {
        self.file_data = value.into();
        self
    }

    #[inline]
    pub(crate) fn set_source_file<P: AsRef<Path>>(&mut self, value: P) -> &mut Self {
        self.source_file = Some(value.as_ref().to_path_buf());
        self
    }

    pub(crate) fn set_attributes<R: Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
    ) {
        let path_str = join_paths(base_path, target);
        let Ok(file) = zip_by_name(arv, &path_str) else {
            // File not found in archive — skip gracefully.
            self.set_file_target(path_str);
            return;
        };
        let mut r = io::BufReader::new(file);
        let mut buf = Vec::new();
        if r.read_to_end(&mut buf).is_err() {
            self.set_file_target(path_str);
            return;
        }

        self.set_file_target(path_str);
        self.file_data = buf;
    }

    pub(crate) fn set_attributes_from_source<R: Read + io::Seek, P: AsRef<Path>>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
        source_file: P,
    ) {
        self.set_attributes(arv, base_path, target);
        self.set_source_file(source_file);
    }

    pub(crate) fn load_file_data_from_source(&mut self) -> Result<(), XlsxError> {
        if !self.file_data.is_empty() || self.file_target().is_empty() {
            return Ok(());
        }
        let Some(source_file) = self.source_file.as_ref() else {
            return Ok(());
        };

        let file = File::open(source_file)?;
        let mut archive = zip::read::ZipArchive::new(file)?;
        let mut source = zip_by_name(&mut archive, self.file_target())?;
        let mut buf = Vec::new();
        source.read_to_end(&mut buf)?;
        self.file_data = buf;
        Ok(())
    }

    pub(crate) fn write_to_target<W: io::Seek + io::Write>(
        &self,
        target: &str,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        if !self.file_data().is_empty() {
            writer_mng.add_bin(target, self.file_data())?;
        } else if let Some(source_file) = self.source_file.as_ref() {
            writer_mng.add_raw_copy(target, source_file, self.file_target())?;
        }
        Ok(())
    }

    pub(crate) fn write_to<W: io::Seek + io::Write>(
        &self,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        self.write_to_target(self.file_target(), writer_mng)
    }
}
