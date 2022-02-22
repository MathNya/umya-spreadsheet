use quick_xml::Writer;
use std::io;
use std::io::Cursor;
use writer::driver::*;

pub struct WriterManager<W: io::Seek + io::Write> {
    files: Vec<String>,
    arv: zip::ZipWriter<W>,
}
impl<W: io::Seek + io::Write> WriterManager<W> {
    pub fn new(arv: zip::ZipWriter<W>) -> Self {
        WriterManager {
            files: Vec::new(),
            arv: arv,
        }
    }

    pub(crate) fn add_writer(
        &mut self,
        target: &str,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> &mut Self {
        let is_match = self.check_file_exist(target);
        if is_match == false {
            make_file_from_writer(target, &mut self.arv, writer, None).unwrap();
            self.files.push(target.to_string());
        }
        self
    }

    pub(crate) fn add_bin(&mut self, target: &str, data: &Vec<u8>) -> &mut Self {
        let is_match = self.check_file_exist(target);
        if is_match == false {
            make_file_from_bin(target, &mut self.arv, data, None).unwrap();
            self.files.push(target.to_string());
        }
        self
    }

    pub(crate) fn get_arv_mut(&mut self) -> &mut zip::ZipWriter<W> {
        &mut self.arv
    }

    pub(crate) fn file_list_sort(&mut self) -> &mut Self {
        self.files.sort();
        self
    }

    pub(crate) fn _get_file_list(&self) -> &Vec<String> {
        &self.files
    }

    pub(crate) fn check_file_exist(&mut self, file_path: &str) -> bool {
        self.file_list_sort();
        for file in &self.files {
            if file == &file_path {
                return true;
            }
        }
        false
    }

    pub(crate) fn add_file_at_drawing(&mut self, writer: Writer<Cursor<Vec<u8>>>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/drawings/drawing{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_writer(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn add_file_at_vml_drawing(&mut self, writer: Writer<Cursor<Vec<u8>>>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/drawings/vmlDrawing{}.vml", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_writer(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn add_file_at_comment(&mut self, writer: Writer<Cursor<Vec<u8>>>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/comments{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_writer(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn add_file_at_chart(&mut self, writer: Writer<Cursor<Vec<u8>>>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/charts/chart{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_writer(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn add_file_at_ole_object(&mut self, writer: &Vec<u8>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/embeddings/oleObject{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_bin(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn add_file_at_excel(&mut self, writer: &Vec<u8>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/embeddings/Microsoft_Excel_Worksheet{}.xlsx", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_bin(&file_path, writer);
                return index;
            }
        }
    }
    pub(crate) fn add_file_at_printer_settings(&mut self, writer: &Vec<u8>) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/printerSettings/printerSettings{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if is_match == false {
                self.add_bin(&file_path, writer);
                return index;
            }
        }
    }

    pub(crate) fn has_extension(&self, extension: &str) -> bool {
        let extension = format!(".{}", extension);
        for file in &self.files {
            if file.ends_with(&extension) {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_find(&self, find_str: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for file in &self.files {
            if file.starts_with(find_str) {
                result.push(format!("/{}", file));
            }
        }
        result
    }
}
