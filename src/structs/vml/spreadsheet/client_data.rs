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

    pub fn get_move_with_cells(&self) -> &Option<MoveWithCells> {
        &self.move_with_cells
    }

    pub fn get_move_with_cells_mut(&mut self) -> &mut Option<MoveWithCells> {
        &mut self.move_with_cells
    }

    pub fn set_move_with_cells(&mut self, value: MoveWithCells) -> &mut Self {
        self.move_with_cells = Some(value);
        self
    }

    pub fn get_resize_with_cells(&self) -> &Option<ResizeWithCells> {
        &self.resize_with_cells
    }

    pub fn get_resize_with_cells_mut(&mut self) -> &mut Option<ResizeWithCells> {
        &mut self.resize_with_cells
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

    pub fn get_auto_fill(&self) -> &Option<AutoFill> {
        &self.auto_fill
    }

    pub fn get_auto_fill_mut(&mut self) -> &mut Option<AutoFill> {
        &mut self.auto_fill
    }

    pub fn set_auto_fill(&mut self, value: AutoFill) -> &mut Self {
        self.auto_fill = Some(value);
        self
    }

    pub fn get_comment_row_target(&self) -> &Option<CommentRowTarget> {
        &self.comment_row_target
    }

    pub fn get_comment_row_target_mut(&mut self) -> &mut Option<CommentRowTarget> {
        &mut self.comment_row_target
    }

    pub fn set_comment_row_target(&mut self, value: CommentRowTarget) -> &mut Self {
        self.comment_row_target = Some(value);
        self
    }

    pub fn get_comment_column_target(&self) -> &Option<CommentColumnTarget> {
        &self.comment_column_target
    }

    pub fn get_comment_column_target_mut(&mut self) -> &mut Option<CommentColumnTarget> {
        &mut self.comment_column_target
    }

    pub fn set_comment_column_target(&mut self, value: CommentColumnTarget) -> &mut Self {
        self.comment_column_target = Some(value);
        self
    }

    pub fn get_visible(&self) -> &Option<Visible> {
        &self.visible
    }

    pub fn get_visible_mut(&mut self) -> &mut Option<Visible> {
        &mut self.visible
    }

    pub fn set_visible(&mut self, value: Visible) -> &mut Self {
        self.visible = Some(value);
        self
    }

    pub fn get_clipboard_format(&self) -> &Option<ClipboardFormat> {
        &self.clipboard_format
    }

    pub fn get_clipboard_format_mut(&mut self) -> &mut Option<ClipboardFormat> {
        &mut self.clipboard_format
    }

    pub fn set_clipboard_format(&mut self, value: ClipboardFormat) -> &mut Self {
        self.clipboard_format = Some(value);
        self
    }

    pub fn get_auto_size_picture(&self) -> &Option<AutoSizePicture> {
        &self.auto_size_picture
    }

    pub fn get_auto_size_picture_mut(&mut self) -> &mut Option<AutoSizePicture> {
        &mut self.auto_size_picture
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
        match get_attribute(e, b"ObjectType") {
            Some(v) => {
                self.object_type.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"x:ClientData" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:ClientData"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
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
        match &self.move_with_cells {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:SizeWithCells
        match &self.resize_with_cells {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:Anchor
        let _ = &self.anchor.write_to(writer);

        // x:AutoFill
        match &self.auto_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:Row
        match &self.comment_row_target {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:Column
        match &self.comment_column_target {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:Visible
        match &self.visible {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:CF
        match &self.clipboard_format {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // x:AutoPict
        match &self.auto_size_picture {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "x:ClientData");
    }
}
