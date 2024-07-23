use super::Anchor;
use super::AutoFill;
use super::AutoSizePicture;
use super::ClipboardFormat;
use super::CommentColumnTarget;
use super::CommentRowTarget;
use super::MoveWithCells;
use super::ObjectValues;
use super::ResizeWithCells;
use super::Visible;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::EnumValue;
use traits::AdjustmentCoordinate;
use traits::AdjustmentValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ClientData {
    object_type: EnumValue<ObjectValues>,
    move_with_cells: Option<MoveWithCells>,
    resize_with_cells: Option<ResizeWithCells>,
    anchor: Anchor,
    auto_fill: Option<AutoFill>,
    comment_row_target: Option<CommentRowTarget>,
    comment_column_target: Option<CommentColumnTarget>,
    visible: Option<Visible>,
    clipboard_format: Option<ClipboardFormat>,
    auto_size_picture: Option<AutoSizePicture>,
}

impl ClientData {
    pub fn get_object_type(&self) -> &ObjectValues {
        self.object_type.get_value()
    }

    pub fn set_object_type(&mut self, value: ObjectValues) -> &mut Self {
        self.object_type.set_value(value);
        self
    }

    pub fn get_move_with_cells(&self) -> Option<&MoveWithCells> {
        self.move_with_cells.as_ref()
    }

    pub fn get_move_with_cells_mut(&mut self) -> Option<&mut MoveWithCells> {
        self.move_with_cells.as_mut()
    }

    pub fn set_move_with_cells(&mut self, value: MoveWithCells) -> &mut Self {
        self.move_with_cells = Some(value);
        self
    }

    pub fn get_resize_with_cells(&self) -> Option<&ResizeWithCells> {
        self.resize_with_cells.as_ref()
    }

    pub fn get_resize_with_cells_mut(&mut self) -> Option<&mut ResizeWithCells> {
        self.resize_with_cells.as_mut()
    }

    pub fn set_resize_with_cells(&mut self, value: ResizeWithCells) -> &mut Self {
        self.resize_with_cells = Some(value);
        self
    }

    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value: Anchor) -> &mut Self {
        self.anchor = value;
        self
    }

    pub fn get_auto_fill(&self) -> Option<&AutoFill> {
        self.auto_fill.as_ref()
    }

    pub fn get_auto_fill_mut(&mut self) -> Option<&mut AutoFill> {
        self.auto_fill.as_mut()
    }

    pub fn set_auto_fill(&mut self, value: AutoFill) -> &mut Self {
        self.auto_fill = Some(value);
        self
    }

    pub fn get_comment_row_target(&self) -> Option<&CommentRowTarget> {
        self.comment_row_target.as_ref()
    }

    pub fn get_comment_row_target_mut(&mut self) -> Option<&mut CommentRowTarget> {
        self.comment_row_target.as_mut()
    }

    pub fn set_comment_row_target(&mut self, value: CommentRowTarget) -> &mut Self {
        self.comment_row_target = Some(value);
        self
    }

    pub fn get_comment_column_target(&self) -> Option<&CommentColumnTarget> {
        self.comment_column_target.as_ref()
    }

    pub fn get_comment_column_target_mut(&mut self) -> Option<&mut CommentColumnTarget> {
        self.comment_column_target.as_mut()
    }

    pub fn set_comment_column_target(&mut self, value: CommentColumnTarget) -> &mut Self {
        self.comment_column_target = Some(value);
        self
    }

    pub fn get_visible(&self) -> Option<&Visible> {
        self.visible.as_ref()
    }

    pub fn get_visible_mut(&mut self) -> Option<&mut Visible> {
        self.visible.as_mut()
    }

    pub fn set_visible(&mut self, value: Visible) -> &mut Self {
        self.visible = Some(value);
        self
    }

    pub fn get_clipboard_format(&self) -> Option<&ClipboardFormat> {
        self.clipboard_format.as_ref()
    }

    pub fn get_clipboard_format_mut(&mut self) -> Option<&mut ClipboardFormat> {
        self.clipboard_format.as_mut()
    }

    pub fn set_clipboard_format(&mut self, value: ClipboardFormat) -> &mut Self {
        self.clipboard_format = Some(value);
        self
    }

    pub fn get_auto_size_picture(&self) -> Option<&AutoSizePicture> {
        self.auto_size_picture.as_ref()
    }

    pub fn get_auto_size_picture_mut(&mut self) -> Option<&mut AutoSizePicture> {
        self.auto_size_picture.as_mut()
    }

    pub fn set_auto_size_picture(&mut self, value: AutoSizePicture) -> &mut Self {
        self.auto_size_picture = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, object_type, "ObjectType");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"x:MoveWithCells" => {
                        let mut obj = MoveWithCells::default();
                        obj.set_attributes(reader, e, true);
                        self.set_move_with_cells(obj);
                    }
                    b"x:SizeWithCells" => {
                        let mut obj = ResizeWithCells::default();
                        obj.set_attributes(reader, e, true);
                        self.set_resize_with_cells(obj);
                    }
                    b"x:AutoFill" => {
                        let mut obj = AutoFill::default();
                        obj.set_attributes(reader, e, true);
                        self.set_auto_fill(obj);
                    }
                    b"x:Visible" => {
                        let mut obj = Visible::default();
                        obj.set_attributes(reader, e, true);
                        self.set_visible(obj);
                    }
                    b"x:AutoPict" => {
                        let mut obj = AutoSizePicture::default();
                        obj.set_attributes(reader, e, true);
                        self.set_auto_size_picture(obj);
                    }
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"x:Anchor" => {
                        let mut obj = Anchor::default();
                        obj.set_attributes(reader, e);
                        self.set_anchor(obj);
                    }
                    b"x:MoveWithCells" => {
                        let mut obj = MoveWithCells::default();
                        obj.set_attributes(reader, e, false);
                        self.set_move_with_cells(obj);
                    }
                    b"x:SizeWithCells" => {
                        let mut obj = ResizeWithCells::default();
                        obj.set_attributes(reader, e, false);
                        self.set_resize_with_cells(obj);
                    }
                    b"x:AutoFill" => {
                        let mut obj = AutoFill::default();
                        obj.set_attributes(reader, e, false);
                        self.set_auto_fill(obj);
                    }
                    b"x:Row" => {
                        let mut obj = CommentRowTarget::default();
                        obj.set_attributes(reader, e);
                        self.set_comment_row_target(obj);
                    }
                    b"x:Column" => {
                        let mut obj = CommentColumnTarget::default();
                        obj.set_attributes(reader, e);
                        self.set_comment_column_target(obj);
                    }
                    b"x:CF" => {
                        let mut obj = ClipboardFormat::default();
                        obj.set_attributes(reader, e);
                        self.set_clipboard_format(obj);
                    }
                    b"x:Visible" => {
                        let mut obj = Visible::default();
                        obj.set_attributes(reader, e, false);
                        self.set_visible(obj);
                    }
                    b"x:AutoPict" => {
                        let mut obj = AutoSizePicture::default();
                        obj.set_attributes(reader, e, false);
                        self.set_auto_size_picture(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"x:ClientData" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:ClientData")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:ClientData
        write_start_tag(
            writer,
            "x:ClientData",
            vec![("ObjectType", self.object_type.get_value_string())],
            false,
        );

        // x:MoveWithCells
        if let Some(v) = &self.move_with_cells {
            v.write_to(writer);
        }

        // x:SizeWithCells
        if let Some(v) = &self.resize_with_cells {
            v.write_to(writer);
        }

        // x:Anchor
        self.anchor.write_to(writer);

        // x:AutoFill
        if let Some(v) = &self.auto_fill {
            v.write_to(writer);
        }

        // x:Row
        if let Some(v) = &self.comment_row_target {
            v.write_to(writer);
        }

        // x:Column
        if let Some(v) = &self.comment_column_target {
            v.write_to(writer);
        }

        // x:Visible
        if let Some(v) = &self.visible {
            v.write_to(writer);
        }

        // x:CF
        if let Some(v) = &self.clipboard_format {
            v.write_to(writer);
        }

        // x:AutoPict
        if let Some(v) = &self.auto_size_picture {
            v.write_to(writer);
        }

        write_end_tag(writer, "x:ClientData");
    }
}
impl AdjustmentCoordinate for ClientData {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.anchor.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        match &mut self.comment_column_target {
            Some(v) => {
                v.adjustment_insert_value(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.comment_row_target {
            Some(v) => {
                v.adjustment_insert_value(root_row_num, offset_row_num);
            }
            None => {}
        }
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.anchor.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        match &mut self.comment_column_target {
            Some(v) => {
                v.adjustment_remove_value(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.comment_row_target {
            Some(v) => {
                v.adjustment_remove_value(root_row_num, offset_row_num);
            }
            None => {}
        }
    }
}
