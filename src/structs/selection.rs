use super::Coordinate;
use super::EnumValue;
use super::PaneValues;
use super::SequenceOfReferences;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Selection {
    pane: EnumValue<PaneValues>,
    active_cell: Option<Coordinate>,
    sequence_of_references: SequenceOfReferences,
}

impl Selection {
    #[inline]
    pub fn get_pane(&self) -> &PaneValues {
        self.pane.get_value()
    }

    #[inline]
    pub fn set_pane(&mut self, value: PaneValues) -> &mut Self {
        self.pane.set_value(value);
        self
    }

    #[inline]
    pub fn get_active_cell(&self) -> Option<&Coordinate> {
        self.active_cell.as_ref()
    }

    #[inline]
    pub fn get_active_cell_mut(&mut self) -> Option<&mut Coordinate> {
        self.active_cell.as_mut()
    }

    #[inline]
    pub fn set_active_cell(&mut self, value: Coordinate) -> &mut Self {
        self.active_cell = Some(value);
        self
    }

    #[inline]
    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    #[inline]
    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    #[inline]
    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) -> &mut Self {
        self.sequence_of_references = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, pane, "pane");

        if let Some(v) = get_attribute(e, b"activeCell") {
            let mut obj = Coordinate::default();
            obj.set_coordinate(v);
            self.set_active_cell(obj);
        }

        if let Some(v) = get_attribute(e, b"sqref") {
            self.sequence_of_references.set_sqref(v);
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let mut active_cell_id = 0;
        if let Some(active_cell) = &self.active_cell {
            for range in self.sequence_of_references.get_range_collection() {
                let range_str = range.get_range();
                if range_str.contains(active_cell.to_string().as_str()) {
                    break;
                }
                active_cell_id += 1;
            }
        }

        if self.pane.has_value() {
            attributes.push(("pane", self.pane.get_value_string()));
        }

        let active_cell_str = match &self.active_cell {
            Some(active_cell) => active_cell.to_string(),
            None => String::from(""),
        };
        if !active_cell_str.is_empty() {
            attributes.push(("activeCell", active_cell_str.as_str()));
        }

        let active_cell_id_str = active_cell_id.to_string();
        if active_cell_id > 0 {
            attributes.push(("activeCellId", active_cell_id_str.as_str()));
        }

        let sqref = self.sequence_of_references.get_sqref();
        if !sqref.is_empty() {
            attributes.push(("sqref", sqref.as_str()));
        }

        write_start_tag(writer, "selection", attributes, true);
    }
}
