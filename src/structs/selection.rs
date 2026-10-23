use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    ColumnReference,
    Coordinate,
    EnumValue,
    PaneValues,
    Range,
    RowReference,
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
    pub fn pane(&self) -> &PaneValues {
        self.pane.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pane()")]
    pub fn get_pane(&self) -> &PaneValues {
        self.pane()
    }

    #[inline]
    pub fn set_pane(&mut self, value: PaneValues) -> &mut Self {
        self.pane.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn active_cell(&self) -> Option<&Coordinate> {
        self.active_cell.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use active_cell()")]
    pub fn get_active_cell(&self) -> Option<&Coordinate> {
        self.active_cell()
    }

    #[inline]
    pub fn active_cell_mut(&mut self) -> Option<&mut Coordinate> {
        self.active_cell.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use active_cell_mut()")]
    pub fn get_active_cell_mut(&mut self) -> Option<&mut Coordinate> {
        self.active_cell_mut()
    }

    #[inline]
    pub fn set_active_cell(&mut self, value: Coordinate) -> &mut Self {
        self.active_cell = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sequence_of_references()")]
    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        self.sequence_of_references()
    }

    #[inline]
    pub fn sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sequence_of_references_mut()")]
    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        self.sequence_of_references_mut()
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

        let active_cell_id = self.active_cell.as_ref().map_or(0, |active_cell| {
            self.sequence_of_references
                .range_collection()
                .iter()
                .position(|range| range_contains(range, active_cell))
                .unwrap_or(0)
        });

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

fn range_contains(range: &Range, cell: &Coordinate) -> bool {
    let start_col = range.coordinate_start_col().map(ColumnReference::num);
    let start_row = range.coordinate_start_row().map(RowReference::num);
    let end_col = range.coordinate_end_col().map(ColumnReference::num).or(start_col);
    let end_row = range.coordinate_end_row().map(RowReference::num).or(start_row);

    let col = cell.col_num();
    let row = cell.row_num();
    start_col.is_none_or(|v| v <= col)
        && end_col.is_none_or(|v| col <= v)
        && start_row.is_none_or(|v| v <= row)
        && end_row.is_none_or(|v| row <= v)
}
