use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use crate::structs::XlsxError;
use crate::writer::driver::*;
use quick_xml::Writer;
use std::io;
use std::io::Cursor;
pub struct WriterManager<'a, W: io::Seek + io::Write> {
    files: Vec<String>,
    arv: &'a mut zip::ZipWriter<W>,
    is_light: bool,
    table_no: i32,
}

impl<'a, W: io::Seek + io::Write> WriterManager<'a, W> {
    #[inline]
    pub fn new(arv: &'a mut zip::ZipWriter<W>) -> Self {
        WriterManager {
            files: Vec::new(),
            arv,
            is_light: false,
            table_no: 0,
        }
    }

    #[inline]
    pub fn set_is_light(&mut self, value: bool) -> &mut Self {
        self.is_light = value;
        self
    }

    #[inline]
    pub fn get_is_light(&self) -> &bool {
        &self.is_light
    }

    #[inline]
    pub fn get_num_tables(&self) -> i32 {
        self.table_no
    }

    #[inline]
    pub fn next_table_no(&mut self) -> i32 {
        self.table_no += 1;
        self.table_no
    }

    #[inline]
    pub(crate) fn add_writer(
        &mut self,
        target: &str,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<(), XlsxError> {
        if !self.check_file_exist(target) {
            make_file_from_writer(target, self.arv, writer, None, &self.is_light)?;
            self.files.push(target.to_string());
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn add_bin(&mut self, target: &str, data: &[u8]) -> Result<(), XlsxError> {
        if !self.check_file_exist(target) {
            make_file_from_bin(target, &mut self.arv, data, None, &self.is_light)?;
            self.files.push(target.to_string());
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn get_arv_mut(&mut self) -> &mut zip::ZipWriter<W> {
        &mut self.arv
    }

    #[inline]
    pub(crate) fn file_list_sort(&mut self) -> &mut Self {
        self.files.sort();
        self
    }

    #[inline]
    pub(crate) fn check_file_exist(&mut self, file_path: &str) -> bool {
        self.file_list_sort();
        self.files.iter().any(|file| file == file_path)
    }

    pub(crate) fn add_file_at_drawing(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
    ) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{}/drawing{}.xml", PKG_DRAWINGS, index);
            if !self.check_file_exist(&file_path) {
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
            let file_path = format!("{}/vmlDrawing{}.vml", PKG_DRAWINGS, index);
            if !self.check_file_exist(&file_path) {
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
            if !self.check_file_exist(&file_path) {
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
            let file_path = format!("{}/chart{}.xml", PKG_CHARTS, index);
            if !self.check_file_exist(&file_path) {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_ole_object(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{}/oleObject{}.bin", PKG_EMBEDDINGS, index);
            if !self.check_file_exist(&file_path) {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_excel(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{}/Microsoft_Excel_Worksheet{}.xlsx", PKG_EMBEDDINGS, index);
            if !self.check_file_exist(&file_path) {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_printer_settings(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{}/printerSettings{}.bin", PKG_PRNTR_SETTINGS, index);
            if !self.check_file_exist(&file_path) {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    #[inline]
    pub(crate) fn add_file_at_table(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
        table_no: i32,
    ) -> Result<i32, XlsxError> {
        let file_path = format!("{}/table{}.xml", PKG_TABLES, table_no);
        self.add_writer(&file_path, writer)?;
        return Ok(table_no);
    }

    #[inline]
    pub(crate) fn has_extension(&self, extension: &str) -> bool {
        let extension = format!(".{}", extension);
        self.files.iter().any(|file| file.ends_with(&extension))
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
                    true => WORKBOOK_MACRO_TYPE,
                    false => WORKBOOK_TYPE,
                };
            }

            // Override sheet
            if file.starts_with("/xl/worksheets/sheet") {
                content_type = SHEET_TYPE;
            }

            // Override table
            if file.starts_with("/xl/tables/table") {
                content_type = TABLE_TYPE;
            }

            // Override comments
            if file.starts_with("/xl/comments") {
                content_type = COMMENTS_TYPE;
            }

            // Override theme
            if file.starts_with("/xl/theme/theme") {
                content_type = THEME_TYPE;
            }

            // Override styles
            if file.starts_with("/xl/styles.xml") {
                content_type = STYLES_TYPE;
            }

            // Override sharedStrings
            if file.starts_with("/xl/sharedStrings.xml") {
                content_type = SHARED_STRINGS_TYPE;
            }

            // Override drawing
            if file.starts_with("/xl/drawings/drawing") {
                content_type = DRAWING_TYPE;
            }

            // Override chart
            if file.starts_with("/xl/charts/chart") {
                content_type = CHART_TYPE;
            }

            // Override embeddings
            if file.starts_with("/xl/embeddings/oleObject") {
                content_type = OLE_OBJECT_TYPE;
            }

            // Override xl/vbaProject.bin
            if file.starts_with("/xl/vbaProject.bin") {
                content_type = VBA_TYPE;
            }

            // Override docProps/core
            if file.starts_with("/docProps/core.xml") {
                content_type = CORE_PROPS_TYPE;
            }

            // Override docProps/app
            if file.starts_with("/docProps/app.xml") {
                content_type = XPROPS_TYPE;
            }

            // Override docProps/custom
            if file.starts_with("/docProps/custom.xml") {
                content_type = CUSTOM_PROPS_TYPE;
            }

            // Override Unsupported
            if content_type.is_empty() {
                for (old_part_name, old_content_type) in spreadsheet.get_backup_context_types() {
                    if &**old_part_name == &file {
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
