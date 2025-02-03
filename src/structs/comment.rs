use quick_xml::{
    Reader,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Coordinate,
    RichText,
    vml::{
        Fill as VmlFill,
        Path,
        Shadow,
        TextBox,
        office::InsetMarginValues,
        spreadsheet::{
            Anchor,
            CommentColumnTarget,
            CommentRowTarget,
            MoveWithCells,
            ResizeWithCells,
        },
    },
};
use crate::{
    helper::coordinate::CellCoordinates,
    reader::driver::get_attribute,
    structs::vml::Shape,
    traits::AdjustmentCoordinate,
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct Comment {
    coordinate: Coordinate,
    author:     Box<str>,
    text:       RichText,
    shape:      Shape,
}

impl Comment {
    #[inline]
    #[must_use]
    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate()")]
    pub fn get_coordinate(&self) -> &Coordinate {
        self.coordinate()
    }

    #[inline]
    pub fn coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.coordinate
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_mut()")]
    pub fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        self.coordinate_mut()
    }

    #[inline]
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use author()")]
    pub fn get_author(&self) -> &str {
        self.author()
    }

    #[inline]
    pub fn set_author<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.author = value.into().into_boxed_str();
        self
    }

    #[inline]
    #[must_use]
    pub fn text(&self) -> &RichText {
        &self.text
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use text()")]
    pub fn get_text(&self) -> &RichText {
        self.text()
    }

    #[inline]
    pub fn text_mut(&mut self) -> &mut RichText {
        &mut self.text
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use text_mut()")]
    pub fn get_text_mut(&mut self) -> &mut RichText {
        self.text_mut()
    }

    #[inline]
    pub fn set_text(&mut self, value: RichText) -> &mut Self {
        self.text = value;
        self
    }

    #[inline]
    pub fn set_text_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text.set_text(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn anchor(&self) -> &Anchor {
        self.shape.get_client_data().get_anchor()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use anchor()")]
    pub fn get_anchor(&self) -> &Anchor {
        self.anchor()
    }

    #[inline]
    pub fn anchor_mut(&mut self) -> &mut Anchor {
        self.shape.get_client_data_mut().get_anchor_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use anchor_mut()")]
    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        self.anchor_mut()
    }

    #[inline]
    pub fn set_anchor(&mut self, value: Anchor) -> &mut Self {
        self.shape.get_client_data_mut().set_anchor(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape()")]
    pub fn get_shape(&self) -> &Shape {
        self.shape()
    }

    #[inline]
    pub fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shape_mut()")]
    pub fn get_shape_mut(&mut self) -> &mut Shape {
        self.shape_mut()
    }

    #[inline]
    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = value;
        self
    }

    #[inline]
    pub fn new_comment<T>(&mut self, coordinate: T) -> &mut Self
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.coordinate_mut().set_col_num(col).set_row_num(row);

        self.shape_mut()
            .set_type("#_x0000_t202")
            .set_style(
                "position:absolute;margin-left:275.25pt;margin-top:61.5pt;width:207.75pt;height:\
                 145.5pt;z-index:1;visibility:hidden;mso-wrap-style:tight",
            )
            .set_fill_color("infoBackground [80]")
            .set_inset_mode(InsetMarginValues::Auto);

        let mut fill = VmlFill::default();
        fill.set_color_2("infoBackground [80]");
        self.shape_mut().set_fill(fill);

        let mut shadow = Shadow::default();
        shadow.set_color("none [81]").set_obscured(true);
        self.shape_mut().set_shadow(shadow);

        let mut path = Path::default();
        path.set_connection_point_type(super::vml::office::ConnectValues::None);
        self.shape_mut().set_path(path);

        let mut textbox = TextBox::default();
        textbox
            .set_style("mso-direction-alt:auto")
            .set_innder("<div style='text-align:left'></div>");
        self.shape_mut().set_text_box(textbox);

        let movewithcells = MoveWithCells::default();
        self.shape_mut()
            .get_client_data_mut()
            .set_move_with_cells(movewithcells);

        let resizewithcells = ResizeWithCells::default();
        self.shape_mut()
            .get_client_data_mut()
            .set_resize_with_cells(resizewithcells);

        self.shape_mut()
            .get_client_data_mut()
            .get_anchor_mut()
            .set_left_column(col)
            .set_left_offset(15)
            .set_top_row(if row > 1 { row - 1 } else { 1 })
            .set_top_offset(8)
            .set_right_column(col + 1)
            .set_right_offset(71)
            .set_bottom_row(row + 3)
            .set_bottom_offset(15);

        let mut comment_col = CommentColumnTarget::default();
        comment_col.set_value(col - 1);
        let mut comment_row = CommentRowTarget::default();
        comment_row.set_value(row - 1);
        self.shape_mut()
            .get_client_data_mut()
            .set_comment_column_target(comment_col)
            .set_comment_row_target(comment_row);

        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        authors: &[String],
    ) {
        let coordinate = get_attribute(e, b"ref").unwrap();
        self.coordinate_mut().set_coordinate(coordinate);

        let author_id = get_attribute(e, b"authorId")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let author = authors.get(author_id).unwrap();
        self.set_author(author);

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"text" {
                    self.text_mut().set_attributes_text(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"comment" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "comment")
        );
    }
}
impl AdjustmentCoordinate for Comment {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.coordinate.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.shape.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.coordinate.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        self.shape.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        self.coordinate.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
