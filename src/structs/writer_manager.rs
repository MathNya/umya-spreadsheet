use quick_xml::Writer;
use std::io;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;
use writer::xlsx::XlsxError;

const CHART_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
const CORE_PROPS_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
const OLE_OBJECT_TYPE: &str = "application/vnd.openxmlformats-officedocument.oleObject";
const PACKAGE_CHARTS: &str = "xl/charts";
const PACKAGE_DRAWINGS: &str = "xl/drawings";
const PACKAGE_EMBEDDINGS: &str = "xl/embeddings";
const PACKAGE_PRINTER_SETTINGS: &str = "xl/printerSettings";
const PACKAGE_TABLES: &str = "xl/tables";
const SHARED_STRINGS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
const PACKAGE_WORKBOOK: &str = "/xl/workbook.xml";
const STYLES_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";
const VBA_TYPE: &str = "application/vnd.ms-office.vbaProject";
const XPROPS_TYPE: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
const THEME_TYPE: &str = "application/vnd.openxmlformats-officedocument.theme+xml";
const COMMENTS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
const TABLE_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";
const SHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const WORKBOOK_MACRO_TYPE: &str = "application/vnd.ms-excel.sheet.macroEnabled.main+xml";
const WORKBOOK_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

pub struct WriterManager<W: io::Seek + io::Write> {
    files: Vec<String>,
    arv: zip::ZipWriter<W>,
    is_light: bool,
    table_no: i32,
}

impl<W: io::Seek + io::Write> WriterManager<W> {
    pub fn new(arv: zip::ZipWriter<W>) -> Self {
        WriterManager {
            files: Vec::new(),
            arv,
            is_light: false,
            table_no: 0,
        }
    }

    pub fn set_is_light(&mut self, value: bool) -> &mut Self {
        self.is_light = value;
        self
    }

    pub fn get_is_light(&self) -> &bool {
        &self.is_light
    }

    pub fn get_num_tables(&self) -> i32 {
        self.table_no
    }

    pub fn next_table_no(&mut self) -> i32 {
        self.table_no += 1;
        self.table_no
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

    pub(crate) fn add_bin(&mut self, target: &str, data: &[u8]) -> Result<(), XlsxError> {
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
            if file == file_path {
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
            let file_path = format!("{PACKAGE_DRAWINGS}/drawing{}.xml", index);
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
            let file_path = format!("{PACKAGE_DRAWINGS}/vmlDrawing{}.vml", index);
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
            let file_path = format!("{PACKAGE_CHARTS}/chart{}.xml", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_writer(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_ole_object(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{PACKAGE_EMBEDDINGS}/oleObject{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_excel(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!(
                "{PACKAGE_EMBEDDINGS}/Microsoft_Excel_Worksheet{}.xlsx",
                index
            );
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_printer_settings(&mut self, writer: &[u8]) -> Result<i32, XlsxError> {
        let mut index = 0;
        loop {
            index += 1;
            let file_path = format!("{PACKAGE_PRINTER_SETTINGS}/printerSettings{}.bin", index);
            let is_match = self.check_file_exist(&file_path);
            if !is_match {
                self.add_bin(&file_path, writer)?;
                return Ok(index);
            }
        }
    }

    pub(crate) fn add_file_at_table(
        &mut self,
        writer: Writer<Cursor<Vec<u8>>>,
        table_no: i32,
    ) -> Result<i32, XlsxError> {
        let file_path = format!("{PACKAGE_TABLES}/table{}.xml", table_no);
        self.add_writer(&file_path, writer)?;
        return Ok(table_no);
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
            let mut content_type = match file.as_str() {
                f if f.starts_with("/xl/workbook.xml") => match spreadsheet.get_has_macros() {
                    true => WORKBOOK_MACRO_TYPE.to_string(),
                    false => WORKBOOK_TYPE.to_string(),
                },
                f if f.starts_with("/xl/worksheets/sheet") => SHEET_TYPE.to_string(),
                f if f.starts_with("/xl/tables/table") => TABLE_TYPE.to_string(),
                f if f.starts_with("/xl/comments") => COMMENTS_TYPE.to_string(),
                f if f.starts_with("/xl/theme/theme") => THEME_TYPE.to_string(),
                f if f.starts_with("/xl/styles.xml") => STYLES_TYPE.to_string(),
                f if f.starts_with("/xl/sharedStrings.xml") => SHARED_STRINGS_TYPE.to_string(),
                f if f.starts_with("/xl/drawings/drawing") => DRAWING_TYPE.to_string(),
                f if f.starts_with("/xl/charts/chart") => CHART_TYPE.to_string(),
                f if f.starts_with("/xl/embeddings/oleObject") => OLE_OBJECT_TYPE.to_string(),
                f if f.starts_with("/xl/vbaProject.bin") => VBA_TYPE.to_string(),
                f if f.starts_with("/docProps/core.xml") => CORE_PROPS_TYPE.to_string(),
                f if f.starts_with("/docProps/app.xml") => XPROPS_TYPE.to_string(),
                _ => {
                    let mut content_type: String = String::new();
                    for (old_part_name, old_content_type) in spreadsheet.get_backup_context_types()
                    {
                        if old_part_name == &file {
                            content_type = old_content_type.to_string();
                            break;
                        }
                    }
                    content_type
                }
            };

            if !content_type.is_empty() {
                list.push((file, content_type.to_string()));
            }
        }

        list
    }
}
