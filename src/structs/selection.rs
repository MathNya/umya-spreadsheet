use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    Coordinate,
    EnumValue,
    PaneValues,
    SequenceOfReferences,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Selection {
    pane:                   EnumValue<PaneValues>,
    active_cell:            Option<Coordinate>,
    sequence_of_references: SequenceOfReferences,
}

impl Selection {
    #[inline]
    #[must_use]
    pub fn get_pane(&self) -> &PaneValues {
        self.pane.value()
    }

    #[inline]
    pub fn set_pane(&mut self, value: PaneValues) -> &mut Self {
        self.pane.set_value(value);
        self
    }

    #[inline]
    #[must_use]
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
    #[must_use]
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
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        let mut active_cell_id = 0;
        if let Some(active_cell) = &self.active_cell {
            for range in self.sequence_of_references.get_range_collection() {
                let range_str = range.range();
                if range_str.contains(&active_cell.to_string()) {
                    break;
                }
                active_cell_id += 1;
            }
        }

        if self.pane.has_value() {
            attributes.push(("pane", self.pane.value_string()).into());
        }

        let active_cell_str = match &self.active_cell {
            Some(active_cell) => active_cell.to_string(),
            None => String::new(),
        };
        if !active_cell_str.is_empty() {
            attributes.push(("activeCell", &active_cell_str).into());
        }

        let active_cell_id_str = active_cell_id.to_string();
        if active_cell_id > 0 {
            attributes.push(("activeCellId", &active_cell_id_str).into());
        }

        let sqref = self.sequence_of_references.get_sqref();
        if !sqref.is_empty() {
            attributes.push(("sqref", &sqref).into());
        }

        write_start_tag(writer, "selection", attributes, true);
    }
}
