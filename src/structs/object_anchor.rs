use super::BooleanValue;
use super::FromMarker;
use super::ToMarker;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ObjectAnchor {
    move_with_cells: BooleanValue,
    from_marker: FromMarker,
    to_marker: ToMarker,
}
impl ObjectAnchor {
    pub fn get_move_with_cells(&self) -> &bool {
        self.move_with_cells.get_value()
    }

    pub fn set_move_with_cells(&mut self, value: bool) -> &mut Self {
        self.move_with_cells.set_value(value);
        self
    }

    pub fn get_from_marker(&self) -> &FromMarker {
        &self.from_marker
    }

    pub fn get_from_marker_mut(&mut self) -> &mut FromMarker {
        &mut self.from_marker
    }

    pub fn set_from_marker(&mut self, value: FromMarker) -> &mut Self {
        self.from_marker = value;
        self
    }

    pub fn get_to_marker(&self) -> &ToMarker {
        &self.to_marker
    }

    pub fn get_to_marker_mut(&mut self) -> &mut ToMarker {
        &mut self.to_marker
    }

    pub fn set_to_marker(&mut self, value: ToMarker) -> &mut Self {
        self.to_marker = value;
        self
    }

    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &usize) {
        self.from_marker._adjustment_insert_row(num_rows);
        self.to_marker._adjustment_insert_row(num_rows);
    }

    pub(crate) fn _adjustment_insert_column(&mut self, num_cols: &usize) {
        self.from_marker._adjustment_insert_column(num_cols);
        self.to_marker._adjustment_insert_column(num_cols);
    }

    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &usize) {
        self.from_marker._adjustment_remove_row(num_rows);
        self.to_marker._adjustment_remove_row(num_rows);
    }

    pub(crate) fn _adjustment_remove_column(&mut self, num_cols: &usize) {
        self.from_marker._adjustment_remove_column(num_cols);
        self.to_marker._adjustment_remove_column(num_cols);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"moveWithCells") {
            Some(v) => {
                self.move_with_cells.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"xdr:from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"to" => {
                        self.to_marker.set_attributes(reader, e);
                    }
                    b"xdr:to" => {
                        self.to_marker.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"anchor" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "anchor"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // anchor
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if &self.move_with_cells.has_value() == &true {
            attributes.push(("moveWithCells", self.move_with_cells.get_value_string()));
        }
        write_start_tag(writer, "anchor", attributes, false);

        // xdr:from
        let _ = &self.from_marker.write_to(writer);

        // xdr:to
        let _ = &self.to_marker.write_to(writer);

        write_end_tag(writer, "anchor");
    }
}
