use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    Coordinate,
    DoubleValue,
    EnumValue,
    PaneStateValues,
    PaneValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Pane {
    horizontal_split: DoubleValue,
    vertical_split:   DoubleValue,
    top_left_cell:    Coordinate,
    active_pane:      EnumValue<PaneValues>,
    state:            EnumValue<PaneStateValues>,
}

impl Pane {
    #[must_use]
    pub fn get_horizontal_split(&self) -> f64 {
        self.horizontal_split.get_value()
    }

    pub fn set_horizontal_split(&mut self, value: f64) -> &mut Self {
        self.horizontal_split.set_value(value);
        self
    }

    #[must_use]
    pub fn get_vertical_split(&self) -> f64 {
        self.vertical_split.get_value()
    }

    pub fn set_vertical_split(&mut self, value: f64) -> &mut Self {
        self.vertical_split.set_value(value);
        self
    }

    #[must_use]
    pub fn get_top_left_cell(&self) -> &Coordinate {
        &self.top_left_cell
    }

    pub fn get_top_left_cell_mut(&mut self) -> &mut Coordinate {
        &mut self.top_left_cell
    }

    pub fn set_top_left_cell(&mut self, value: Coordinate) -> &mut Self {
        self.top_left_cell = value;
        self
    }

    #[must_use]
    pub fn get_active_pane(&self) -> &PaneValues {
        self.active_pane.get_value()
    }

    pub fn set_active_pane(&mut self, value: PaneValues) -> &mut Self {
        self.active_pane.set_value(value);
        self
    }

    #[must_use]
    pub fn get_state(&self) -> &PaneStateValues {
        self.state.get_value()
    }

    pub fn set_state(&mut self, value: PaneStateValues) -> &mut Self {
        self.state.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, horizontal_split, "xSplit");
        set_string_from_xml!(self, e, vertical_split, "ySplit");
        set_string_from_xml!(self, e, active_pane, "activePane");
        set_string_from_xml!(self, e, state, "state");

        if let Some(v) = get_attribute(e, b"topLeftCell") {
            self.top_left_cell.set_coordinate(v);
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pane
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let coordinate = self.top_left_cell.to_string();
        let horizontal_split = self.horizontal_split.get_value_string();
        if self.horizontal_split.has_value() {
            attributes.push(("xSplit", &horizontal_split).into());
        }
        let vertical_split = self.vertical_split.get_value_string();
        if self.vertical_split.has_value() {
            attributes.push(("ySplit", &vertical_split).into());
        }
        attributes.push(("topLeftCell", coordinate.as_str()).into());
        attributes.push(("activePane", self.active_pane.get_value_string()).into());
        attributes.push(("state", self.state.get_value_string()).into());
        write_start_tag(writer, "pane", attributes, true);
    }
}
