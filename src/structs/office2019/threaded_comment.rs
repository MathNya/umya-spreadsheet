// threadedComment
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    Coordinate,
    DateTimeValue,
    StringValue,
    office2019::threaded_comment_text::ThreadedCommentText,
    reader::driver::get_attribute,
    set_string_from_xml,
    traits::AdjustmentCoordinate,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct ThreadedComment {
    coordinate:            Coordinate,
    d_t:                   DateTimeValue,
    threaded_comment_text: ThreadedCommentText,
    id:                    StringValue,
}

impl ThreadedComment {
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
    pub fn d_t(&self) -> &str {
        self.d_t.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use d_t()")]
    pub fn get_d_t(&self) -> &str {
        self.d_t()
    }

    #[inline]
    pub fn set_d_t<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.d_t.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use id()")]
    pub fn get_id(&self) -> &str {
        self.id()
    }

    #[inline]
    pub(crate) fn set_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.id.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let coordinate = get_attribute(e, b"ref").unwrap();
        self.coordinate_mut().set_coordinate(coordinate);

        set_string_from_xml!(self, e, d_t, "dT");
        set_string_from_xml!(self, e, id, "id");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"text" {
                    self.threaded_comment_text.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"threadedComment" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "threadedComment")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // threadedComment
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let coordinate = self.coordinate.to_string();
        attributes.push(("ref", &coordinate).into());

        if self.d_t.has_value() {
            attributes.push(("dT", self.d_t.value_str()).into());
        }
        attributes.push(("personId", "{00000000-0000-0000-0000-000000000000}").into());
        if self.id.has_value() {
            attributes.push(("id", self.id.value_str()).into());
        }
        write_start_tag(writer, "threadedComment", attributes, false);

        // text
        self.threaded_comment_text.write_to(writer);

        write_end_tag(writer, "threadedComment");
    }
}
impl AdjustmentCoordinate for ThreadedComment {
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
