// a:grpSpLocks
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::BooleanValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct GroupShapeLocks {
    no_change_aspect: BooleanValue,
    no_grouping:      BooleanValue,
    no_move:          BooleanValue,
    no_resize:        BooleanValue,
    no_rotation:      BooleanValue,
    no_selection:     BooleanValue,
    no_ungrouping:    BooleanValue,
}

impl GroupShapeLocks {
    #[must_use]
    pub fn get_no_change_aspect(&self) -> bool {
        self.no_change_aspect.get_value()
    }

    pub fn set_no_change_aspect(&mut self, value: bool) {
        self.no_change_aspect.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, no_change_aspect, "noChangeAspect");
        set_string_from_xml!(self, e, no_grouping, "noGrp");
        set_string_from_xml!(self, e, no_move, "noMove");
        set_string_from_xml!(self, e, no_resize, "noResize");
        set_string_from_xml!(self, e, no_rotation, "noRot");
        set_string_from_xml!(self, e, no_selection, "noSelect");
        set_string_from_xml!(self, e, no_ungrouping, "noUngrp");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:grpSpLocks
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        let no_change_aspect_str = self.no_change_aspect.get_value_string();
        if self.no_change_aspect.has_value() {
            attributes.push(("noChangeAspect", no_change_aspect_str).into());
        }

        let no_grouping_str = self.no_grouping.get_value_string();
        if self.no_grouping.has_value() {
            attributes.push(("noGrp", no_grouping_str).into());
        }

        let no_move_str = self.no_move.get_value_string();
        if self.no_move.has_value() {
            attributes.push(("noMove", no_move_str).into());
        }

        let no_resize_str = self.no_resize.get_value_string();
        if self.no_resize.has_value() {
            attributes.push(("noResize", no_resize_str).into());
        }

        let no_rotation_str = self.no_rotation.get_value_string();
        if self.no_rotation.has_value() {
            attributes.push(("noRot", no_rotation_str).into());
        }

        let no_selection_str = self.no_selection.get_value_string();
        if self.no_selection.has_value() {
            attributes.push(("noSelect", no_selection_str).into());
        }

        let no_ungrouping_str = self.no_ungrouping.get_value_string();
        if self.no_ungrouping.has_value() {
            attributes.push(("noUngrp", no_ungrouping_str).into());
        }

        write_start_tag(writer, "a:grpSpLocks", attributes, true);
    }
}
