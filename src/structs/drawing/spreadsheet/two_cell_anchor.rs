// xdr:twoCellAnchor
use super::super::super::EnumValue;
use super::ConnectionShape;
use super::EditAsValues;
use super::GraphicFrame;
use super::MarkerType;
use super::Picture;
use super::Shape;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::BooleanValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct TwoCellAnchor {
    edit_as: EnumValue<EditAsValues>,
    from_marker: MarkerType,
    to_marker: MarkerType,
    graphic_frame: Option<GraphicFrame>,
    shape: Option<Shape>,
    connection_shape: Option<ConnectionShape>,
    picture: Option<Picture>,
    is_alternate_content: BooleanValue,
}
impl TwoCellAnchor {
    pub fn get_edit_as(&self) -> &EditAsValues {
        self.edit_as.get_value()
    }

    pub fn set_edit_as(&mut self, value: EditAsValues) -> &mut Self {
        self.edit_as.set_value(value);
        self
    }

    pub fn get_from_marker(&self) -> &MarkerType {
        &self.from_marker
    }

    pub fn get_from_marker_mut(&mut self) -> &mut MarkerType {
        &mut self.from_marker
    }

    pub fn set_from_marker(&mut self, value: MarkerType) -> &mut Self {
        self.from_marker = value;
        self
    }

    pub fn get_to_marker(&self) -> &MarkerType {
        &self.to_marker
    }

    pub fn get_to_marker_mut(&mut self) -> &mut MarkerType {
        &mut self.to_marker
    }

    pub fn set_to_marker(&mut self, value: MarkerType) -> &mut Self {
        self.to_marker = value;
        self
    }

    pub fn get_graphic_frame(&self) -> &Option<GraphicFrame> {
        &self.graphic_frame
    }

    pub fn get_graphic_frame_mut(&mut self) -> &mut Option<GraphicFrame> {
        &mut self.graphic_frame
    }

    pub fn set_graphic_frame(&mut self, value: GraphicFrame) -> &mut Self {
        self.graphic_frame = Some(value);
        self
    }

    pub fn get_shape(&self) -> &Option<Shape> {
        &self.shape
    }

    pub fn get_shape_mut(&mut self) -> &mut Option<Shape> {
        &mut self.shape
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = Some(value);
        self
    }

    pub fn get_connection_shape(&self) -> &Option<ConnectionShape> {
        &self.connection_shape
    }

    pub fn get_connection_shape_mut(&mut self) -> &mut Option<ConnectionShape> {
        &mut self.connection_shape
    }

    pub fn set_connection_shape(&mut self, value: ConnectionShape) -> &mut Self {
        self.connection_shape = Some(value);
        self
    }

    pub fn get_picture(&self) -> &Option<Picture> {
        &self.picture
    }

    pub fn get_picture_mut(&mut self) -> &mut Option<Picture> {
        &mut self.picture
    }

    pub fn set_picture(&mut self, value: Picture) -> &mut Self {
        self.picture = Some(value);
        self
    }

    pub fn get_is_alternate_content(&self) -> &bool {
        self.is_alternate_content.get_value()
    }

    pub fn set_is_alternate_content(&mut self, value: bool) -> &mut Self {
        self.is_alternate_content.set_value(value);
        self
    }

    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &u32) {
        self.from_marker._adjustment_insert_row(num_rows);
        self.to_marker._adjustment_insert_row(num_rows);
    }

    pub(crate) fn _adjustment_insert_column(&mut self, num_cols: &u32) {
        self.from_marker._adjustment_insert_column(num_cols);
        self.to_marker._adjustment_insert_column(num_cols);
    }

    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &u32) {
        self.from_marker._adjustment_remove_row(num_rows);
        self.to_marker._adjustment_remove_row(num_rows);
    }

    pub(crate) fn _adjustment_remove_column(&mut self, num_cols: &u32) {
        self.from_marker._adjustment_remove_column(num_cols);
        self.to_marker._adjustment_remove_column(num_cols);
    }

    pub(crate) fn is_support(&self) -> bool {
        match &self.graphic_frame {
            Some(v) => {
                return v
                    .get_graphic()
                    .get_graphic_data()
                    .get_chart_space()
                    .get_chart()
                    .get_plot_area()
                    .is_support();
            }
            None => {}
        }
        true
    }

    pub(crate) fn is_chart(&self) -> bool {
        match &self.graphic_frame {
            Some(_) => {
                return true;
            }
            None => {}
        }
        false
    }

    pub(crate) fn is_image(&self) -> bool {
        match &self.picture {
            Some(_) => {
                return true;
            }
            None => {}
        }
        false
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        match get_attribute(e, b"editAs") {
            Some(v) => {
                self.edit_as.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"xdr:from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"xdr:to" => {
                        self.to_marker.set_attributes(reader, e);
                    }
                    b"xdr:graphicFrame" => {
                        let mut obj = GraphicFrame::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_graphic_frame(obj);
                    }
                    b"xdr:sp" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(reader, e);
                        self.set_shape(obj);
                    }
                    b"xdr:cxnSp" => {
                        let mut obj = ConnectionShape::default();
                        obj.set_attributes(reader, e);
                        self.set_connection_shape(obj);
                    }
                    b"xdr:pic" => {
                        let mut obj = Picture::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_picture(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:twoCellAnchor" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:twoCellAnchor"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: &mut i32,
        ole_id: &usize,
    ) {
        if self.get_is_alternate_content() == &true {
            // mc:AlternateContent
            write_start_tag(
                writer,
                "mc:AlternateContent",
                vec![(
                    "xmlns:mc",
                    "http://schemas.openxmlformats.org/markup-compatibility/2006",
                )],
                false,
            );

            // mc:Choice
            write_start_tag(
                writer,
                "mc:Choice",
                vec![
                    (
                        "xmlns:a14",
                        "http://schemas.microsoft.com/office/drawing/2010/main",
                    ),
                    ("Requires", "a14"),
                ],
                false,
            );
        }

        // xdr:twoCellAnchor
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if &self.edit_as.has_value() == &true {
            attributes.push(("editAs", self.edit_as.get_value_string()));
        }
        write_start_tag(writer, "xdr:twoCellAnchor", attributes, false);

        // xdr:from
        let _ = &self.from_marker.write_to_from(writer);

        // xdr:to
        let _ = &self.to_marker.write_to_to(writer);

        // xdr:graphicFrame
        match &self.graphic_frame {
            Some(v) => {
                v.write_to(writer, r_id);
                *r_id += 1i32;
            }
            None => {}
        }

        // xdr:sp
        match &self.shape {
            Some(v) => v.write_to(writer, ole_id),
            None => {}
        }

        // xdr:cxnSp
        match &self.connection_shape {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // xdr:pic
        match &self.picture {
            Some(v) => {
                v.write_to(writer, r_id);
                *r_id += 1i32;
            }
            None => {}
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:twoCellAnchor");

        if self.get_is_alternate_content() == &true {
            write_end_tag(writer, "mc:Choice");

            // mc:Fallback
            write_start_tag(writer, "mc:Fallback", vec![], true);

            write_end_tag(writer, "mc:AlternateContent");
        }
    }
}
