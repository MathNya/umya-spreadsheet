//! Streaming xlsx writer.
//! Streaming writer provide us a way to incrementally write `WorkSheet` data
//! one at a time. This is useful in situation where a file has a large number
//! of Worksheet, that cannot all fit into memory.
//!
//! Limitiation: a `WorkSheet`, once flushed, cannot be modified or read back
//! into memory.
//!
//! # Example
//! ```no_run
//! use std::io::Cursor;
//!
//! use umya_spreadsheet::{
//!     new_file,
//!     writer::streaming_writer::StreamingWriter,
//! };
//!
//! let mut book = new_file();
//! let _ = book.new_sheet("Sheet2");
//! let zip_writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
//! let mut sw = StreamingWriter::new(zip_writer, book);
//! let sheet = sw.take_sheet("Sheet1").unwrap();
//! sw.flush_sheet(sheet).unwrap();
//! let sheet = sw.take_sheet("Sheet2").unwrap();
//! sw.flush_sheet(sheet).unwrap();
//! let _writer = sw.finish().unwrap();
//! ```
#[allow(unused_imports)]
use std::{
    fs,
    fs::File,
    io,
    io::Read,
    path::Path,
    sync::{
        Arc,
        RwLock,
    },
};

use crate::{
    Stylesheet,
    structs::{
        // SharedStringTable,
        // Stylesheet,
        Workbook,
        Worksheet,
        WriterManager,
        XlsxError,
    },
    writer::xlsx::{
        chart,
        comment,
        content_types,
        doc_props_app,
        doc_props_core,
        doc_props_custom,
        drawing,
        drawing_rels,
        embeddings,
        jsa_project_bin,
        media,
        person,
        pivot_cache,
        pivot_table,
        printer_settings,
        rels,
        shared_strings,
        styles,
        table,
        theme,
        threaded_comment,
        vba_project_bin,
        vml_drawing,
        vml_drawing_rels,
        workbook,
        workbook_rels,
        worksheet,
        worksheet_rels,
    },
};

// StreamingWriter manage the stream writting process. The Streaming Writer is
// essentially all the functions defined in writer/xlsx.rs, but re-wrap as a
// struct method here, because we need a stateful, long-lived struct to keep
// track of the sheets we dumped / not dumped to our zip archive.
//
pub struct StreamingWriter<W: io::Write + io::Seek> {
    writer_manager: WriterManager<W>,
    // NOTE: workbook here is only to reference
    work_book:      Workbook,

    available:  Vec<Worksheet>,
    stylesheet: Stylesheet,
    has_macros: bool,
    sheet_no:   i32,
}

impl<W: io::Write + io::Seek> StreamingWriter<W> {
    pub fn new(zip_writer: zip::ZipWriter<W>, mut work_book: Workbook) -> Self {
        let stylesheet = work_book.stylesheet().clone();
        let has_macros = work_book.has_macros();
        let writer_manager = WriterManager::new(zip_writer);
        let available = work_book.take_all_sheets();
        Self {
            writer_manager,
            work_book,
            available,
            stylesheet,
            has_macros,
            sheet_no: 0,
        }
    }

    // finish the final part in writer::xlsx, the part after looping through all
    // sheet.
    // return owner ship of our writer to caller
    // also consume our write manager
    pub fn finish(mut self) -> Result<W, XlsxError> {
        // from xlsx::write_zip_to_writer
        // Add docProps
        doc_props_app::write(&self.work_book, &mut self.writer_manager)?;
        doc_props_core::write(&self.work_book, &mut self.writer_manager)?;
        doc_props_custom::write(&self.work_book, &mut self.writer_manager)?;
        vba_project_bin::write(&self.work_book, &mut self.writer_manager)?;
        jsa_project_bin::write(&self.work_book, &mut self.writer_manager)?;
        rels::write(&self.work_book, &mut self.writer_manager)?;
        theme::write(self.work_book.theme(), &mut self.writer_manager)?;
        person::write(&self.work_book, &mut self.writer_manager)?;

        self.writer_manager.file_list_sort();
        shared_strings::write(
            &self.work_book.shared_string_table(),
            &mut self.writer_manager,
        )?;
        styles::write(&self.stylesheet, &mut self.writer_manager)?;
        workbook::write(&self.work_book, &mut self.writer_manager)?;

        let has_shared_string_table = self
            .work_book
            .shared_string_table()
            .read()
            .unwrap()
            .has_value();
        workbook_rels::write(
            &self.work_book,
            has_shared_string_table,
            &mut self.writer_manager,
        )?;
        content_types::write(&self.work_book, &mut self.writer_manager)?;

        Ok(self.writer_manager.finish()?)
    }

    pub fn take_sheet(&mut self, name: &str) -> Option<Worksheet> {
        let pos = self.available.iter().position(|ws| ws.name() == name)?;
        Some(self.available.remove(pos))
    }

    // flush_sheet write a Worksheet into our zip writer. The object is consumed and
    // no longer holds in memory, forever lost in the void.
    #[allow(clippy::needless_pass_by_value)]
    pub fn flush_sheet(&mut self, worksheet: Worksheet) -> Result<(), XlsxError> {
        self.sheet_no += 1;
        // TODO: allow caller to specify worksheet number
        let worksheet_no = self.sheet_no;

        worksheet::write(
            worksheet_no,
            &worksheet,
            &self.work_book.shared_string_table(),
            &mut self.stylesheet,
            self.has_macros,
            &mut self.writer_manager,
        )?;

        // Add charts
        let chart_no_list: Result<Vec<String>, XlsxError> = worksheet
            .worksheet_drawing()
            .chart_collection()
            .iter()
            .map(|chart| {
                chart::write(
                    chart.chart_space(),
                    &self.work_book,
                    &mut self.writer_manager,
                )
            })
            .collect();

        let chart_no_list = chart_no_list?;

        // Add drawing and its relationships
        let (drawing_no, rel_list) = drawing::write(&worksheet, &mut self.writer_manager)?;
        drawing_rels::write(
            &worksheet,
            &drawing_no,
            &chart_no_list,
            &rel_list,
            &mut self.writer_manager,
        )?;

        // Add vml drawing and its relationships
        let (vml_drawing_no, rel_list) = vml_drawing::write(&worksheet, &mut self.writer_manager)?;
        vml_drawing_rels::write(
            &worksheet,
            &vml_drawing_no,
            &rel_list,
            &mut self.writer_manager,
        )?;

        // Add comments
        let comment_no = comment::write(&worksheet, &mut self.writer_manager)?;

        // Add threaded_comment
        let threaded_comment_no = threaded_comment::write(&worksheet, &mut self.writer_manager)?;

        // Add ole_object and excel
        let (ole_object_no_list, excel_no_list) =
            embeddings::write(&worksheet, &mut self.writer_manager)?;

        // Add Media
        media::write(&worksheet, &mut self.writer_manager)?;

        // Add printer settings
        let printer_settings_no = worksheet
            .page_setup()
            .object_data()
            .map_or_else(String::new, |_| {
                printer_settings::write(&worksheet, &mut self.writer_manager).unwrap_or_default()
            });

        // Add tables
        let table_no_list = table::write(&worksheet, &mut self.writer_manager)?;

        // Add pivot tables and caches
        let pivot_table_no_list = pivot_table::write(&worksheet, &mut self.writer_manager)?;
        let pivot_cache_no_list = pivot_cache::write(&worksheet, &mut self.writer_manager)?;

        // Add worksheet relationships
        worksheet_rels::write(
            &worksheet,
            &worksheet_no.to_string(),
            &drawing_no,
            &vml_drawing_no,
            &comment_no,
            &threaded_comment_no,
            &ole_object_no_list,
            &excel_no_list,
            &printer_settings_no,
            &table_no_list,
            &pivot_table_no_list,
            &pivot_cache_no_list,
            &mut self.writer_manager,
        )?;

        let mut stub = Worksheet::default();
        stub.set_name(worksheet.name());
        if worksheet.has_state() {
            stub.set_state_str(worksheet.state_str());
        }
        let defined = worksheet.defined_names();
        if !defined.is_empty() {
            stub.set_defined_names(defined.to_vec());
        }
        self.work_book.add_sheet(stub)?;

        Ok(())
    }
}
