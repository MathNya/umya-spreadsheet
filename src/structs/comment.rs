use super::vml::spreadsheet::Anchor;
use super::Coordinate;
use super::RichText;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use reader::driver::*;
use structs::vml::Shape;

#[derive(Clone, Default, Debug)]
pub struct Comment {
    coordinate: Coordinate,
    author: String,
    text: RichText,
    shape: Shape,
}
impl Comment {
    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.coordinate
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn set_author<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.author = value.into();
        self
    }

    pub fn get_text(&self) -> &RichText {
        &self.text
    }

    pub fn get_text_mut(&mut self) -> &mut RichText {
        &mut self.text
    }

    pub fn set_text(&mut self, value: RichText) -> &mut Self {
        self.text = value;
        self
    }

    pub fn get_anchor(&self) -> &Anchor {
        self.shape.get_client_data().get_anchor()
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        self.shape.get_client_data_mut().get_anchor_mut()
    }

    pub fn set_anchor(&mut self, value: Anchor) -> &mut Self {
        self.shape.get_client_data_mut().set_anchor(value);
        self
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = value;
        self
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        let org_col_num = *self.coordinate.get_col_num();
        let org_row_num = *self.coordinate.get_row_num();
        self.coordinate.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        if &org_col_num != self.coordinate.get_col_num() {
            self.get_anchor_mut()
                .adjustment_insert_column(offset_col_num);
            match self
                .get_shape_mut()
                .get_client_data_mut()
                .get_comment_column_target_mut()
            {
                Some(v) => {
                    v.adjustment_insert_column(offset_col_num);
                }
                None => {}
            }
        }
        if &org_row_num != self.coordinate.get_row_num() {
            self.get_anchor_mut().adjustment_insert_row(offset_row_num);
            match self
                .get_shape_mut()
                .get_client_data_mut()
                .get_comment_row_target_mut()
            {
                Some(v) => {
                    v.adjustment_insert_row(offset_row_num);
                }
                None => {}
            }
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        let org_col_num = *self.coordinate.get_col_num();
        let org_row_num = *self.coordinate.get_row_num();
        self.coordinate.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
        if &org_col_num != self.coordinate.get_col_num() {
            self.get_anchor_mut()
                .adjustment_remove_column(offset_col_num);
            match self
                .get_shape_mut()
                .get_client_data_mut()
                .get_comment_column_target_mut()
            {
                Some(v) => {
                    v.adjustment_remove_column(offset_col_num);
                }
                None => {}
            }
        }
        if &org_row_num != self.coordinate.get_row_num() {
            self.get_anchor_mut().adjustment_remove_row(offset_row_num);
            match self
                .get_shape_mut()
                .get_client_data_mut()
                .get_comment_row_target_mut()
            {
                Some(v) => {
                    v.adjustment_remove_row(offset_row_num);
                }
                None => {}
            }
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        authors: &Vec<String>,
    ) {
        let coordinate = get_attribute(e, b"ref").unwrap();
        self.get_coordinate_mut().set_coordinate(coordinate);

        let author_id = get_attribute(e, b"authorId")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let author = authors.get(author_id).unwrap();
        self.set_author(author);

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"text" => {
                        self.get_text_mut().set_attributes_text(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"comment" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "comment"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }
}
