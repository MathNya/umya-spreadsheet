use std::io;
use structs::raw::RawFile;
use structs::raw::RawRelationships;
use structs::WriterManager;
use writer::xlsx::XlsxError;

#[derive(Clone, Debug, Default)]
pub(crate) struct RawWorksheet {
    worksheet_file: RawFile,
    relationships_list: Vec<RawRelationships>,
}
impl RawWorksheet {
    pub(crate) fn get_worksheet_file(&self) -> &RawFile {
        &self.worksheet_file
    }

    pub(crate) fn get_worksheet_file_mut(&mut self) -> &mut RawFile {
        &mut self.worksheet_file
    }

    pub(crate) fn get_relationships_list(&self) -> &Vec<RawRelationships> {
        &self.relationships_list
    }

    pub(crate) fn _get_relationships_list_mut(&mut self) -> &mut Vec<RawRelationships> {
        &mut self.relationships_list
    }

    pub(crate) fn set_relationships(&mut self, value: RawRelationships) -> &mut Self {
        self.relationships_list.push(value);
        self
    }

    pub(crate) fn get_worksheet_relationships(&self) -> Option<&RawRelationships> {
        self.get_relationships_list().iter().find(|&relationships| {
            relationships
                .get_file_target()
                .starts_with("xl/worksheets/_rels/sheet")
        })
    }

    pub(crate) fn get_drawing_relationships(&self) -> Option<&RawRelationships> {
        self.get_relationships_list().iter().find(|&relationships| {
            relationships
                .get_file_target()
                .starts_with("xl/drawings/_rels/drawing")
        })
    }

    pub(crate) fn get_vml_drawing_relationships(&self) -> Option<&RawRelationships> {
        self.get_relationships_list().iter().find(|&relationships| {
            relationships
                .get_file_target()
                .starts_with("xl/drawings/_rels/vmlDrawing")
        })
    }

    pub(crate) fn read<R: io::Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        target: &str,
    ) {
        self.get_worksheet_file_mut()
            .set_attributes(arv, "xl", target);

        let base_path = self.get_worksheet_file().get_path();
        let target = self.get_worksheet_file().make_rel_name();
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
            for relationship in obj.get_relationship_list() {
                let rels_base_path = relationship.get_raw_file().get_path();
                let rels_target = relationship.get_raw_file().make_rel_name();
                self.read_rawrelationships(arv, &rels_base_path, &rels_target);
            }
            self.set_relationships(obj);
        }
    }

    pub(crate) fn write<W: io::Seek + io::Write>(
        &self,
        sheet_no: &i32,
        writer_mng: &mut WriterManager<W>,
    ) -> Result<(), XlsxError> {
        // Add worksheet
        let target = format!("xl/worksheets/sheet{}.xml", sheet_no);
        writer_mng.add_bin(&target, self.get_worksheet_file().get_file_data())?;

        // Add worksheet rels
        for relationships in self.get_relationships_list() {
            relationships.write_to(writer_mng, None)?;
        }

        Ok(())
    }
}
