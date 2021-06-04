// xdr:blipFill
use super::FromMarker;
use super::ToMarker;
use super::super::charts::Chart;
use super::Shape;
use super::ConnectionShape;
use super::Picture;
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;
use tempdir::TempDir;

#[derive(Default, Debug)]
pub struct TwoCellAnchor {
    edit_as: Option<String>,
    from_marker: FromMarker,
    to_marker: ToMarker,
    chart: Option<Chart>,
    shape: Option<Shape>,
    connection_shape: Option<ConnectionShape>,
    picture: Option<Picture>,
}
impl TwoCellAnchor {
    pub fn get_edit_as(&self)-> &Option<String> {
        &self.edit_as
    }

    pub fn set_edit_as<S: Into<String>>(&mut self, value:S)-> &mut TwoCellAnchor {
        self.edit_as = Some(value.into());
        self
    }

    pub fn get_from_marker(&self)-> &FromMarker {
        &self.from_marker
    }

    pub fn get_from_marker_mut(&mut self)-> &mut FromMarker {
        &mut self.from_marker
    }

    pub fn set_from_marker(&mut self, value:FromMarker)-> &mut TwoCellAnchor {
        self.from_marker = value;
        self
    }

    pub fn get_to_marker(&self)-> &ToMarker {
        &self.to_marker
    }

    pub fn get_to_marker_mut(&mut self)-> &mut ToMarker {
        &mut self.to_marker
    }

    pub fn set_to_marker(&mut self, value:ToMarker)-> &mut TwoCellAnchor {
        self.to_marker = value;
        self
    }

    pub fn get_chart(&self)-> &Option<Chart> {
        &self.chart
    }

    pub fn get_chart_mut(&mut self)-> &mut Option<Chart> {
        &mut self.chart
    }

    pub fn set_chart(&mut self, value:Chart)-> &mut TwoCellAnchor {
        self.chart = Some(value);
        self
    }

    pub fn get_shape(&self)-> &Option<Shape> {
        &self.shape
    }

    pub fn get_shape_mut(&mut self)-> &mut Option<Shape> {
        &mut self.shape
    }

    pub fn set_shape(&mut self, value:Shape)-> &mut TwoCellAnchor {
        self.shape = Some(value);
        self
    }

    pub fn get_connection_shape(&self)-> &Option<ConnectionShape> {
        &self.connection_shape
    }

    pub fn get_connection_shape_mut(&mut self)-> &mut Option<ConnectionShape> {
        &mut self.connection_shape
    }

    pub fn set_connection_shape(&mut self, value:ConnectionShape)-> &mut TwoCellAnchor {
        self.connection_shape = Some(value);
        self
    }

    pub fn get_picture(&self)-> &Option<Picture> {
        &self.picture
    }

    pub fn get_picture_mut(&mut self)-> &mut Option<Picture> {
        &mut self.picture
    }

    pub fn set_picture(&mut self, value:Picture)-> &mut TwoCellAnchor {
        self.picture = Some(value);
        self
    }

    pub(crate) fn adjustment_insert_row(&mut self, num_rows:&usize) {
        &mut self.from_marker.adjustment_insert_row(num_rows);
        &mut self.to_marker.adjustment_insert_row(num_rows);
    }

    pub(crate) fn adjustment_insert_colmun(&mut self, num_cols:&usize) {
        &mut self.from_marker.adjustment_insert_colmun(num_cols);
        &mut self.to_marker.adjustment_insert_colmun(num_cols);
    }

    pub(crate) fn adjustment_remove_row(&mut self, num_rows:&usize) {
        &mut self.from_marker.adjustment_remove_row(num_rows);
        &mut self.to_marker.adjustment_remove_row(num_rows);
    }

    pub(crate) fn adjustment_remove_colmun(&mut self, num_cols:&usize) {
        &mut self.from_marker.adjustment_remove_colmun(num_cols);
        &mut self.to_marker.adjustment_remove_colmun(num_cols);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        dir: &TempDir,
        target: &str,
    ) {
        match get_attribute(e, b"editAs") {
            Some(v) => {self.set_edit_as(v);},
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"xdr:from" => {
                            self.from_marker.set_attributes(reader, e);
                        },
                        b"xdr:to" => {
                            self.to_marker.set_attributes(reader, e);
                        },
                        b"xdr:graphicFrame" => {
                            let mut chart = Chart::default();
                            chart.set_attributes(reader, e, dir, target);
                            self.set_chart(chart);
                        },
                        b"xdr:sp" => {
                            let mut shape = Shape::default();
                            shape.set_attributes(reader, e);
                            self.set_shape(shape);
                        },
                        b"xdr:cxnSp" => {
                            let mut connection_shape = ConnectionShape::default();
                            connection_shape.set_attributes(reader, e);
                            self.set_connection_shape(connection_shape);
                        }
                        b"xdr:pic" => {
                            let mut picture = Picture::default();
                            picture.set_attributes(reader, e, dir, target);
                            self.set_picture(picture);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:twoCellAnchor" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:twoCellAnchor"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut i32) {
        // xdr:twoCellAnchor
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.edit_as {
            Some(v) => {
                attributes.push(("editAs", v));
            },
            None => {}
        }
        write_start_tag(writer, "xdr:twoCellAnchor", attributes, false);

        // xdr:from
        &self.from_marker.write_to(writer);

        // xdr:to
        &self.to_marker.write_to(writer);

        // xdr:graphicFrame
        match &self.chart {
            Some(v) => {
                v.write_to(writer, r_id);
                *r_id += 1i32;
            },
            None => {},
        }

        // xdr:sp
        match &self.shape {
            Some(v) => v.write_to(writer),
            None => {},
        }

        // xdr:cxnSp
        match &self.connection_shape {
            Some(v) => v.write_to(writer),
            None => {},
        }

        // xdr:pic
        match &self.picture {
            Some(v) => {
                v.write_to(writer, r_id);
                *r_id += 1i32;
            },
            None => {},
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:twoCellAnchor");
    }
}
