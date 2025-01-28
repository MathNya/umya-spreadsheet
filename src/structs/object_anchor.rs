use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    BooleanValue,
    FromMarker,
    ToMarker,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ObjectAnchor {
    move_with_cells: BooleanValue,
    from_marker:     FromMarker,
    to_marker:       ToMarker,
}

impl ObjectAnchor {
    #[inline]
    #[must_use]
    pub fn get_move_with_cells(&self) -> bool {
        self.move_with_cells.value()
    }

    #[inline]
    pub fn set_move_with_cells(&mut self, value: bool) -> &mut Self {
        self.move_with_cells.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_from_marker(&self) -> &FromMarker {
        &self.from_marker
    }

    #[inline]
    pub fn get_from_marker_mut(&mut self) -> &mut FromMarker {
        &mut self.from_marker
    }

    #[inline]
    pub fn set_from_marker(&mut self, value: FromMarker) -> &mut Self {
        self.from_marker = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_to_marker(&self) -> &ToMarker {
        &self.to_marker
    }

    #[inline]
    pub fn get_to_marker_mut(&mut self) -> &mut ToMarker {
        &mut self.to_marker
    }

    #[inline]
    pub fn set_to_marker(&mut self, value: ToMarker) -> &mut Self {
        self.to_marker = value;
        self
    }

    #[inline]
    pub(crate) fn adjustment_insert_row(&mut self, num_rows: usize) {
        self.from_marker.adjustment_insert_row(num_rows);
        self.to_marker.adjustment_insert_row(num_rows);
    }

    #[inline]
    pub(crate) fn adjustment_insert_column(&mut self, num_cols: usize) {
        self.from_marker.adjustment_insert_column(num_cols);
        self.to_marker.adjustment_insert_column(num_cols);
    }

    #[inline]
    pub(crate) fn adjustment_remove_row(&mut self, num_rows: usize) {
        self.from_marker.adjustment_remove_row(num_rows);
        self.to_marker.adjustment_remove_row(num_rows);
    }

    #[inline]
    pub(crate) fn adjustment_remove_column(&mut self, num_cols: usize) {
        self.from_marker.adjustment_remove_column(num_cols);
        self.to_marker.adjustment_remove_column(num_cols);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, move_with_cells, "moveWithCells");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"from" | b"xdr:from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"to" | b"xdr:to" => {
                        self.to_marker.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"anchor" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "anchor")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // anchor
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.move_with_cells.has_value() {
            attributes.push(("moveWithCells", self.move_with_cells.value_string()).into());
        }
        write_start_tag(writer, "anchor", attributes, false);

        // xdr:from
        self.from_marker.write_to(writer);

        // xdr:to
        self.to_marker.write_to(writer);

        write_end_tag(writer, "anchor");
    }
}
