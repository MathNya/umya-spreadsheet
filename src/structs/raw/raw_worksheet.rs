use std::io;

use crate::{
    helper::const_str::{
        PKG_DRAWINGS_RELS,
        PKG_PIVOT_CACHE_RELS,
        PKG_PIVOT_TABLE_RELS,
        PKG_SHEET,
        PKG_SHEET_RELS,
        PKG_VML_DRAWING_RELS,
    },
    structs::{
        WriterManager,
        XlsxError,
        raw::{
            RawFile,
            RawRelationships,
        },
    },
};

#[derive(Clone, Debug, Default)]
pub(crate) struct RawWorksheet {
    worksheet_file:     RawFile,
    relationships_list: Vec<RawRelationships>,
}
impl RawWorksheet {
    #[inline]
    pub(crate) fn worksheet_file(&self) -> &RawFile {
        &self.worksheet_file
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use worksheet_file()")]
    pub(crate) fn get_worksheet_file(&self) -> &RawFile {
        self.worksheet_file()
    }

    #[inline]
    pub(crate) fn worksheet_file_mut(&mut self) -> &mut RawFile {
        &mut self.worksheet_file
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use worksheet_file_mut()")]
    pub(crate) fn get_worksheet_file_mut(&mut self) -> &mut RawFile {
        self.worksheet_file_mut()
    }

    #[inline]
    pub(crate) fn relationships_list(&self) -> &[RawRelationships] {
        &self.relationships_list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use relationships_list()")]
    pub(crate) fn get_relationships_list(&self) -> &[RawRelationships] {
        self.relationships_list()
    }

    #[inline]
    pub(crate) fn relationships_list_mut(&mut self) -> &mut Vec<RawRelationships> {
        &mut self.relationships_list
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use relationships_list_mut()")]
    pub(crate) fn get_relationships_list_mut(&mut self) -> &mut Vec<RawRelationships> {
        self.relationships_list_mut()
    }

    #[inline]
    pub(crate) fn set_relationships(&mut self, value: RawRelationships) -> &mut Self {
        self.relationships_list.push(value);
        self
    }

    #[inline]
    pub(crate) fn worksheet_relationships(&self) -> Option<&RawRelationships> {
        self.relationships_list()
            .iter()
            .find(|&relationships| relationships.file_target().starts_with(PKG_SHEET_RELS))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use worksheet_relationships()")]
    pub(crate) fn get_worksheet_relationships(&self) -> Option<&RawRelationships> {
        self.worksheet_relationships()
    }

    pub(crate) fn drawing_relationships(&self) -> Option<&RawRelationships> {
        self.relationships_list()
            .iter()
            .find(|&relationships| relationships.file_target().starts_with(PKG_DRAWINGS_RELS))
    }

    #[deprecated(since = "3.0.0", note = "Use drawing_relationships()")]
    pub(crate) fn get_drawing_relationships(&self) -> Option<&RawRelationships> {
        self.drawing_relationships()
    }

    pub(crate) fn vml_drawing_relationships(&self) -> Option<&RawRelationships> {
        self.relationships_list().iter().find(|&relationships| {
            relationships
                .file_target()
                .starts_with(PKG_VML_DRAWING_RELS)
        })
    }

    #[deprecated(since = "3.0.0", note = "Use vml_drawing_relationships()")]
    pub(crate) fn get_vml_drawing_relationships(&self) -> Option<&RawRelationships> {
        self.vml_drawing_relationships()
    }

    pub(crate) fn pivot_table_relationships(&self) -> Option<&RawRelationships> {
        self.relationships_list().iter().find(|&relationships| {
            relationships
                .file_target()
                .starts_with(PKG_PIVOT_TABLE_RELS)
        })
    }

    pub(crate) fn pivot_cache_relationships(&self) -> Option<&RawRelationships> {
        self.relationships_list().iter().find(|&relationships| {
            relationships
                .file_target()
                .starts_with(PKG_PIVOT_CACHE_RELS)
        })
    }

    pub(crate) fn read<R: io::Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        target: &str,
    ) {
        self.worksheet_file_mut().set_attributes(arv, "xl", target);

        let base_path = self.worksheet_file().path();
        let target = self.worksheet_file().make_rel_name();
        self.read_rawrelationships(arv, &base_path, &target);
    }

    pub(crate) fn read_rawrelationships<R: io::Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        base_path: &str,
        target: &str,
    ) {
        let mut obj = RawRelationships::default();
        if obj.set_attributes(arv, base_path, target) {
            for relationship in obj.relationship_list() {
                let rels_base_path = relationship.raw_file().path();
                let rels_target = relationship.raw_file().make_rel_name();
                self.read_rawrelationships(arv, &rels_base_path, &rels_target);
            }
            self.set_relationships(obj);
        }
    }

    pub(crate) fn write<W: io::Seek + io::Write>(
        &self,
        sheet_no: i32,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        // Add worksheet
        let target = format!("{PKG_SHEET}{sheet_no}.xml");
        writer_mng.add_bin(&target, self.worksheet_file().file_data())?;

        // Add worksheet rels
        for relationships in self.relationships_list() {
            relationships.write_to(writer_mng, None)?;
        }

        Ok(())
    }
}
