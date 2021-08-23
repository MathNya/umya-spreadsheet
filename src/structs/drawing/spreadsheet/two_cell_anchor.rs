// xdr:twoCellAnchor
use super::EditAsValues;
use super::super::super::EnumValue;
use super::FromMarker;
use super::ToMarker;
use super::GraphicFrame;
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
    edit_as: EnumValue<EditAsValues>,
    from_marker: FromMarker,
    to_marker: ToMarker,
    graphic_frame: Option<GraphicFrame>,
    shape: Option<Shape>,
    connection_shape: Option<ConnectionShape>,
    picture: Option<Picture>,
}
impl TwoCellAnchor {
    pub fn get_edit_as(&self)-> &EditAsValues {
        &self.edit_as.get_value()
    }

    pub fn set_edit_as(&mut self, value:EditAsValues)-> &mut TwoCellAnchor {
        self.edit_as.set_value(value);
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

    pub fn get_graphic_frame(&self)-> &Option<GraphicFrame> {
        &self.graphic_frame
    }

    pub fn get_graphic_frame_mut(&mut self)-> &mut Option<GraphicFrame> {
        &mut self.graphic_frame
    }

    pub fn set_graphic_frame(&mut self, value:GraphicFrame)-> &mut TwoCellAnchor {
        self.graphic_frame = Some(value);
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

    pub(crate) fn is_support(&self) -> bool {
        match &self.graphic_frame {
            Some(v) => {
                return v.get_graphic().get_graphic_data().get_chart_space().get_chart().get_plot_area().is_support();
            },
            None => {}
        }
        true
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart,
        dir: &TempDir,
        target: &str,
    ) {
        match get_attribute(e, b"editAs") {
            Some(v) => {self.edit_as.set_value_string(v);},
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
                            let mut obj = GraphicFrame::default();
                            obj.set_attributes(reader, e, dir, target);
                            self.set_graphic_frame(obj);
                        },
                        b"xdr:sp" => {
                            let mut obj = Shape::default();
                            obj.set_attributes(reader, e);
                            self.set_shape(obj);
                        },
                        b"xdr:cxnSp" => {
                            let mut obj = ConnectionShape::default();
                            obj.set_attributes(reader, e);
                            self.set_connection_shape(obj);
                        }
                        b"xdr:pic" => {
                            let mut obj = Picture::default();
                            obj.set_attributes(reader, e, dir, target);
                            self.set_picture(obj);
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
        if &self.edit_as.has_value() == &true {
            attributes.push(("editAs", &self.edit_as.get_value_string()));
        }
        write_start_tag(writer, "xdr:twoCellAnchor", attributes, false);

        // xdr:from
        &self.from_marker.write_to(writer);

        // xdr:to
        &self.to_marker.write_to(writer);

        // xdr:graphicFrame
        match &self.graphic_frame {
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
