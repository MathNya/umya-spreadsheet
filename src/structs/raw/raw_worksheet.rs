use std::io;
use structs::raw::RawFile;
use structs::raw::RawRelationships;
use structs::WriterManager;
use writer::xlsx::XlsxError;

#[derive(Clone, Debug, Default)]
pub(crate) struct RawWorksheet {
    worksheet_file: RawFile,
    relationships: RawRelationships,
    drawing_relationships: RawRelationships,
    vml_drawing_relationships: RawRelationships,
}
impl RawWorksheet {
    pub(crate) fn get_worksheet_file(&self) -> &RawFile {
        &self.worksheet_file
    }

    pub(crate) fn get_worksheet_file_mut(&mut self) -> &mut RawFile {
        &mut self.worksheet_file
    }

    pub(crate) fn get_relationships(&self) -> &RawRelationships {
        &self.relationships
    }

    pub(crate) fn get_relationships_mut(&mut self) -> &mut RawRelationships {
        &mut self.relationships
    }

    pub(crate) fn get_drawing_relationships(&self) -> &RawRelationships {
        &self.drawing_relationships
    }

    pub(crate) fn get_drawing_relationships_mut(&mut self) -> &mut RawRelationships {
        &mut self.drawing_relationships
    }

    pub(crate) fn get_vml_drawing_relationships(&self) -> &RawRelationships {
        &self.vml_drawing_relationships
    }

    pub(crate) fn get_vml_drawing_relationships_mut(&mut self) -> &mut RawRelationships {
        &mut self.vml_drawing_relationships
    }

    pub(crate) fn read<R: io::Read + io::Seek>(
        &mut self,
        arv: &mut zip::read::ZipArchive<R>,
        target: &str,
    ) {
        self.get_worksheet_file_mut()
            .set_attributes(arv, "xl", target);

        let worksheet_rels_target = self.get_worksheet_file().make_rel_name();
        self.get_relationships_mut()
            .set_attributes(arv, "xl/worksheets", &worksheet_rels_target);

        let relationships = self.get_relationships_mut().clone();
        match relationships.get_drawing_raw_file() {
            Some(v) => {
                let drawing_target = v.make_rel_name();
                self.get_drawing_relationships_mut().set_attributes(
                    arv,
                    "xl/drawings",
                    &drawing_target,
                );
            }
            None => {}
        }
        match relationships.get_vml_drawing_raw_file() {
            Some(v) => {
                let vml_drawing_target = v.make_rel_name();
                self.get_vml_drawing_relationships_mut().set_attributes(
                    arv,
                    "xl/drawings",
                    &vml_drawing_target,
                );
            }
            None => {}
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
        let target = format!("xl/worksheets/_rels/sheet{}.xml.rels", sheet_no);
        self.get_relationships()
            .write_to(writer_mng, Some(&target))?;

        // Add drawing
        self.get_drawing_relationships()
            .write_to(writer_mng, None)?;

        // Add vml drawing
        self.get_vml_drawing_relationships()
            .write_to(writer_mng, None)?;

        Ok(())
    }
}
