// threadedComment
use crate::office2019::threaded_comment_text::ThreadedCommentText;
use crate::reader::driver::get_attribute;
use crate::writer::driver::{write_end_tag, write_start_tag, write_text_node};
use crate::{
    set_string_from_xml, xml_read_loop, AdjustmentCoordinate, AdjustmentValue, Coordinate,
    DateTimeValue, StringValue,
};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ThreadedComment {
    coordinate: Coordinate,
    d_t: DateTimeValue,
    threaded_comment_text: ThreadedCommentText,
    id: StringValue,
}

impl ThreadedComment {
    #[inline]
    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    #[inline]
    pub fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.coordinate
    }

    #[inline]
    pub fn get_d_t(&self) -> &str {
        self.d_t.get_value_str()
    }

    #[inline]
    pub fn set_d_t<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.d_t.set_value(value);
        self
    }

    #[inline]
    pub fn get_id(&self) -> &str {
        &self.id.get_value_str()
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
        self.get_coordinate_mut().set_coordinate(coordinate);

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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let coordinate = self.coordinate.to_string();
        attributes.push(("ref", &coordinate));

        if self.d_t.has_value() {
            attributes.push(("dT", self.d_t.get_value_str()));
        }
        attributes.push(("personId", "{00000000-0000-0000-0000-000000000000}"));
        if self.id.has_value() {
            attributes.push(("id", self.id.get_value_str()));
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
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
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
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
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
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.coordinate.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
