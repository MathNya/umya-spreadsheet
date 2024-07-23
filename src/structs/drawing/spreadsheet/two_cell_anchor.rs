// xdr:twoCellAnchor
use super::super::super::EnumValue;
use super::ConnectionShape;
use super::EditAsValues;
use super::GraphicFrame;
use super::GroupShape;
use super::MarkerType;
use super::Picture;
use super::Shape;
use helper::const_str::MC_NS;
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::BooleanValue;
use traits::AdjustmentCoordinate;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct TwoCellAnchor {
    edit_as: EnumValue<EditAsValues>,
    from_marker: MarkerType,
    to_marker: MarkerType,
    group_shape: Option<GroupShape>,
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

    pub fn get_group_shape(&self) -> Option<&GroupShape> {
        self.group_shape.as_ref()
    }

    pub fn get_group_shape_mut(&mut self) -> Option<&mut GroupShape> {
        self.group_shape.as_mut()
    }

    pub fn set_group_shape(&mut self, value: GroupShape) -> &mut Self {
        self.group_shape = Some(value);
        self
    }

    pub fn get_graphic_frame(&self) -> Option<&GraphicFrame> {
        self.graphic_frame.as_ref()
    }

    pub fn get_graphic_frame_mut(&mut self) -> Option<&mut GraphicFrame> {
        self.graphic_frame.as_mut()
    }

    pub fn set_graphic_frame(&mut self, value: GraphicFrame) -> &mut Self {
        self.graphic_frame = Some(value);
        self
    }

    pub fn get_shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    pub fn get_shape_mut(&mut self) -> Option<&mut Shape> {
        self.shape.as_mut()
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = Some(value);
        self
    }

    pub fn get_connection_shape(&self) -> Option<&ConnectionShape> {
        self.connection_shape.as_ref()
    }

    pub fn get_connection_shape_mut(&mut self) -> Option<&mut ConnectionShape> {
        self.connection_shape.as_mut()
    }

    pub fn set_connection_shape(&mut self, value: ConnectionShape) -> &mut Self {
        self.connection_shape = Some(value);
        self
    }

    pub fn get_picture(&self) -> Option<&Picture> {
        self.picture.as_ref()
    }

    pub fn get_picture_mut(&mut self) -> Option<&mut Picture> {
        self.picture.as_mut()
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

    pub(crate) fn is_support(&self) -> bool {
        self.graphic_frame.as_ref().map_or(true, |v| {
            v.get_graphic()
                .get_graphic_data()
                .get_chart_space()
                .get_chart()
                .get_plot_area()
                .is_support()
        })
    }

    pub(crate) fn is_chart(&self) -> bool {
        self.graphic_frame.is_some()
    }

    pub(crate) fn is_image(&self) -> bool {
        self.picture.is_some() || self.group_shape.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, edit_as, "editAs");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"xdr:from" => {
                    self.from_marker.set_attributes(reader, e);
                }
                b"xdr:to" => {
                    self.to_marker.set_attributes(reader, e);
                }
                b"xdr:grpSp" => {
                    let mut obj = GroupShape::default();
                    obj.set_attributes(reader, e, drawing_relationships);
                    self.set_group_shape(obj);
                }
                b"xdr:graphicFrame" => {
                    let mut obj = GraphicFrame::default();
                    obj.set_attributes(reader, e, drawing_relationships);
                    self.set_graphic_frame(obj);
                }
                b"xdr:sp" => {
                    let mut obj = Shape::default();
                    obj.set_attributes(reader, e, drawing_relationships);
                    self.set_shape(obj);
                }
                b"xdr:cxnSp" => {
                    let mut obj = ConnectionShape::default();
                    obj.set_attributes(reader, e, drawing_relationships);
                    self.set_connection_shape(obj);
                }
                b"xdr:pic" => {
                    let mut obj = Picture::default();
                    obj.set_attributes(reader, e, drawing_relationships);
                    self.set_picture(obj);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:twoCellAnchor" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:twoCellAnchor")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
        ole_id: &usize,
    ) {
        if *self.get_is_alternate_content() {
            // mc:AlternateContent
            write_start_tag(
                writer,
                "mc:AlternateContent",
                vec![("xmlns:mc", MC_NS)],
                false,
            );

            // mc:Choice
            write_start_tag(
                writer,
                "mc:Choice",
                vec![("xmlns:a14", DRAWING_MAIN_NS), ("Requires", "a14")],
                false,
            );
        }

        // xdr:twoCellAnchor
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.edit_as.has_value() {
            attributes.push(("editAs", self.edit_as.get_value_string()));
        }
        write_start_tag(writer, "xdr:twoCellAnchor", attributes, false);

        // xdr:from
        self.from_marker.write_to_from(writer);

        // xdr:to
        self.to_marker.write_to_to(writer);

        // xdr:grpSp
        if let Some(v) = &self.group_shape {
            v.write_to(writer, rel_list);
        }

        // xdr:graphicFrame
        if let Some(v) = &self.graphic_frame {
            v.write_to(writer, rel_list);
        }

        // xdr:sp
        if let Some(v) = &self.shape {
            v.write_to(writer, rel_list, ole_id);
        }

        // xdr:cxnSp
        if let Some(v) = &self.connection_shape {
            v.write_to(writer, rel_list)
        }

        // xdr:pic
        if let Some(v) = &self.picture {
            v.write_to(writer, rel_list);
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:twoCellAnchor");

        if *self.get_is_alternate_content() {
            write_end_tag(writer, "mc:Choice");

            // mc:Fallback
            write_start_tag(writer, "mc:Fallback", vec![], true);

            write_end_tag(writer, "mc:AlternateContent");
        }
    }
}
impl AdjustmentCoordinate for TwoCellAnchor {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.from_marker.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.to_marker.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.from_marker.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.to_marker.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn is_remove_coordinate(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.from_marker.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        ) || self.to_marker.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
impl AdjustmentCoordinateWithSheet for TwoCellAnchor {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        match &mut self.graphic_frame {
            Some(v) => {
                v.adjustment_insert_coordinate_with_sheet(
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }
            None => {}
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        match &mut self.graphic_frame {
            Some(v) => {
                v.adjustment_remove_coordinate_with_sheet(
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }
            None => {}
        }
    }
}
