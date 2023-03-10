use quick_xml::Writer;
use std::io;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;
use writer::xlsx::XlsxError;

pub struct WriterManager<W: io::Seek + io::Write> {
    files: Vec<String>,
    arv: zip::ZipWriter<W>,
    is_light: bool,
}
impl<W: io::Seek + io::Write> WriterManager<W> {
    pub fn new(arv: zip::ZipWriter<W>) -> Self {
        WriterManager {
            files: Vec::new(),
            arv,
            is_light: false,
        }
    }

    pub fn set_is_light(&mut self, value: bool) -> &mut Self {
        self.is_light = value;
        self
    }

    pub fn get_is_light(&self) -> &bool {
        &self.is_light
    }

    pub(crate) fn add_writer(
        &mut self,
        target: &str,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<(), XlsxError> {
        let is_match = self.check_file_exist(target);
        if !is_match {
            make_file_from_writer(target, &mut self.arv, writer, None, &self.is_light)?;
            self.files.push(target.to_string());
        }
        Ok(())
    }

    pub(crate) fn add_bin(&mut self, target: &str, data: &Vec<u8>) -> Result<(), XlsxError> {
        let is_match = self.check_file_exist(target);
        if !is_match {
            make_file_from_bin(target, &mut self.arv, data, None, &self.is_light)?;
            self.files.push(target.to_string());
        }
        Ok(())
    }

    pub(crate) fn get_arv_mut(&mut self) -> &mut zip::ZipWriter<W> {
        &mut self.arv
    }

    pub(crate) fn file_list_sort(&mut self) -> &mut Self {
        self.files.sort();
        self
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

    pub(crate) fn add_file_at_drawing(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/drawings/drawing{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_vml_drawing(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/drawings/vmlDrawing{}.vml", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_comment(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/comments{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_chart(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/charts/chart{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_ole_object(&mut self, writer: &Vec<u8>) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/embeddings/oleObject{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_excel(&mut self, writer: &Vec<u8>) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/embeddings/Microsoft_Excel_Worksheet{}.xlsx", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }
    pub(crate) fn add_file_at_printer_settings(
        &mut self,
        writer: &Vec<u8>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("xl/printerSettings/printerSettings{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
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

    pub(crate) fn make_context_type_override(
        &mut self,
        spreadsheet: &Spreadsheet,
    ) -> Vec<(String, String)> {
        self.file_list_sort();
        let mut list: Vec<(String, String)> = Vec::new();
        for file in &self.files {
            let file = format!("/{}", file);
            let mut content_type = "";
            // Override workbook
            if file.starts_with("/xl/workbook.xml") {
                content_type = match spreadsheet.get_has_macros() {
                    true => "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
                    false => {
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
                    }
                };
            }

            // Override sheet
            if file.starts_with("/xl/worksheets/sheet") {
                content_type =
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
            }

            // Override comments
            if file.starts_with("/xl/comments") {
                content_type =
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
            }

            // Override theme
            if file.starts_with("/xl/theme/theme") {
                content_type = "application/vnd.openxmlformats-officedocument.theme+xml";
            }

            // Override styles
            if file.starts_with("/xl/styles.xml") {
                content_type =
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";
            }

            // Override sharedStrings
            if file.starts_with("/xl/sharedStrings.xml") {
                content_type =
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
            }

            // Override drawing
            if file.starts_with("/xl/drawings/drawing") {
                content_type = "application/vnd.openxmlformats-officedocument.drawing+xml";
            }

            // Override chart
            if file.starts_with("/xl/charts/chart") {
                content_type = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
            }

            // Override embeddings
            if file.starts_with("/xl/embeddings/oleObject") {
                content_type = "application/vnd.openxmlformats-officedocument.oleObject";
            }

            // Override xl/vbaProject.bin
            if file.starts_with("/xl/vbaProject.bin") {
                content_type = "application/vnd.ms-office.vbaProject";
            }

            // Override docProps/core
            if file.starts_with("/docProps/core.xml") {
                content_type = "application/vnd.openxmlformats-package.core-properties+xml";
            }

            // Override docProps/app
            if file.starts_with("/docProps/app.xml") {
                content_type =
                    "application/vnd.openxmlformats-officedocument.extended-properties+xml";
            }

            // Override Unsupported
            if content_type.is_empty() {
                for (old_part_name, old_content_type) in spreadsheet.get_backup_context_types() {
                    if old_part_name == &file {
                        content_type = old_content_type;
                    }
                }
            }

            if !content_type.is_empty() {
                list.push((file, content_type.to_string()));
            }
        }
        list
    }
}
