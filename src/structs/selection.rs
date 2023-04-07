use super::Coordinate;
use super::EnumValue;
use super::PaneValues;
use super::SequenceOfReferences;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Selection {
    pane: EnumValue<PaneValues>,
    active_cell: Option<Coordinate>,
    sequence_of_references: SequenceOfReferences,
}
impl Selection {
    pub fn get_pane(&self) -> &PaneValues {
        self.pane.get_value()
    }

    pub fn set_pane(&mut self, value: PaneValues) -> &mut Self {
        self.pane.set_value(value);
        self
    }

    pub fn get_active_cell(&self) -> &Option<Coordinate> {
        &self.active_cell
    }

    pub fn get_active_cell_mut(&mut self) -> &mut Option<Coordinate> {
        &mut self.active_cell
    }

    pub fn set_active_cell(&mut self, value: Coordinate) -> &mut Self {
        self.active_cell = Some(value);
        self
    }

    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) -> &mut Self {
        self.sequence_of_references = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"pane") {
            Some(v) => {
                self.pane.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"activeCell") {
            Some(v) => {
                let mut obj = Coordinate::default();
                obj.set_coordinate(v);
                self.set_active_cell(obj);
            }
            None => {}
        }

        match get_attribute(e, b"sqref") {
            Some(v) => {
                self.sequence_of_references.set_sqref(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let mut active_cell_id = 0;
        match &self.active_cell {
            Some(active_cell) => {
                for range in self.sequence_of_references.get_range_collection() {
                    let range_str = range.get_range();
                    match range_str.find(active_cell.get_coordinate().as_str()) {
                        Some(_) => {
                            break;
                        }
                        None => {}
                    }
                    active_cell_id += 1;
                }
            }
            None => {}
        }

        if self.pane.has_value() {
            attributes.push(("pane", self.pane.get_value_string()));
        }

        let active_cell_str = match &self.active_cell {
            Some(active_cell) => active_cell.get_coordinate(),
            None => String::from(""),
        };
        if active_cell_str != "" {
            attributes.push(("activeCell", active_cell_str.as_str()));
        }

        let active_cell_id_str = active_cell_id.to_string();
        if active_cell_id > 0 {
            attributes.push(("activeCellId", active_cell_id_str.as_str()));
        }

        let sqref = self.sequence_of_references.get_sqref();
        if sqref != "" {
            attributes.push(("sqref", sqref.as_str()));
        }

        write_start_tag(writer, "selection", attributes, true);
    }
}
