use super::Coordinate;
use super::DoubleValue;
use super::EnumValue;
use super::PaneStateValues;
use super::PaneValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Pane {
    horizontal_split: DoubleValue,
    vertical_split: DoubleValue,
    top_left_cell: Coordinate,
    active_pane: EnumValue<PaneValues>,
    state: EnumValue<PaneStateValues>,
}
impl Pane {
    pub fn get_horizontal_split(&self) -> &f64 {
        self.horizontal_split.get_value()
    }

    pub fn set_horizontal_split(&mut self, value: f64) -> &mut Self {
        self.horizontal_split.set_value(value);
        self
    }

    pub fn get_vertical_split(&self) -> &f64 {
        self.vertical_split.get_value()
    }

    pub fn set_vertical_split(&mut self, value: f64) -> &mut Self {
        self.vertical_split.set_value(value);
        self
    }

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

    pub fn get_active_pane(&self) -> &PaneValues {
        self.active_pane.get_value()
    }

    pub fn set_active_pane(&mut self, value: PaneValues) -> &mut Self {
        self.active_pane.set_value(value);
        self
    }

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
        match get_attribute(e, b"xSplit") {
            Some(v) => {
                self.horizontal_split.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"ySplit") {
            Some(v) => {
                self.vertical_split.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"topLeftCell") {
            Some(v) => {
                self.top_left_cell.set_coordinate(v);
            }
            None => {}
        }

        match get_attribute(e, b"activePane") {
            Some(v) => {
                self.active_pane.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"state") {
            Some(v) => {
                self.state.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pane
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let coordinate = self.top_left_cell.get_coordinate();
        let horizontal_split = self.horizontal_split.get_value_string();
        if self.horizontal_split.has_value() {
            attributes.push(("xSplit", &horizontal_split));
        }
        let vertical_split = self.vertical_split.get_value_string();
        if self.vertical_split.has_value() {
            attributes.push(("ySplit", &vertical_split));
        }
        attributes.push(("topLeftCell", coordinate.as_str()));
        attributes.push(("activePane", self.active_pane.get_value_string()));
        attributes.push(("state", self.state.get_value_string()));
        write_start_tag(writer, "pane", attributes, true);
    }
}
