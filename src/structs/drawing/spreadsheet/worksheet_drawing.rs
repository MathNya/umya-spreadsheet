// xdr:wsDr
use super::ConnectionShape;
use super::GraphicFrame;
use super::OneCellAnchor;
use super::Picture;
use super::Shape;
use super::TwoCellAnchor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::Chart;
use structs::Image;
use structs::OleObjects;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct WorksheetDrawing {
    image_collection: Vec<Image>,
    chart_collection: Vec<Chart>,
    one_cell_anchor_collection: Vec<OneCellAnchor>,
    two_cell_anchor_collection: Vec<TwoCellAnchor>,
}
impl WorksheetDrawing {
    pub fn get_image_collection(&self) -> &Vec<Image> {
        &self.image_collection
    }

    pub fn get_image_collection_mut(&mut self) -> &mut Vec<Image> {
        &mut self.image_collection
    }

    pub fn add_image(&mut self, value: Image) -> &mut Self {
        self.image_collection.push(value);
        self
    }

    pub fn get_image(&self, col: &u32, row: &u32) -> Option<&Image> {
        self.image_collection
            .iter()
            .find(|&image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
    }

    pub fn get_image_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Image> {
        for image in &mut self.image_collection {
            if image.get_col() == &(col - 1) && image.get_row() == &(row - 1) {
                return Some(image);
            }
        }
        None
    }

    pub fn get_images(&self, col: &u32, row: &u32) -> Vec<&Image> {
        let mut result: Vec<&Image> = Vec::new();
        for image in &self.image_collection {
            if image.get_col() == &(col - 1) && image.get_row() == &(row - 1) {
                result.push(image);
            }
        }
        result
    }

    pub fn get_images_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Image> {
        let mut result: Vec<&mut Image> = Vec::new();
        for image in &mut self.image_collection {
            if image.get_col() == &(col - 1) && image.get_row() == &(row - 1) {
                result.push(image);
            }
        }
        result
    }

    pub fn get_chart_collection(&self) -> &Vec<Chart> {
        &self.chart_collection
    }

    pub fn get_chart_collection_mut(&mut self) -> &mut Vec<Chart> {
        &mut self.chart_collection
    }

    pub fn add_chart_collection(&mut self, value: Chart) -> &mut Self {
        self.chart_collection.push(value);
        self
    }

    pub fn get_chart(&self, col: &u32, row: &u32) -> Option<&Chart> {
        self.chart_collection
            .iter()
            .find(|&chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
    }

    pub fn get_chart_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Chart> {
        for chart in &mut self.chart_collection {
            if chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1) {
                return Some(chart);
            }
        }
        None
    }

    pub fn get_charts(&self, col: &u32, row: &u32) -> Vec<&Chart> {
        let mut result: Vec<&Chart> = Vec::new();
        for chart in &self.chart_collection {
            if chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1) {
                result.push(chart);
            }
        }
        result
    }

    pub fn get_charts_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Chart> {
        let mut result: Vec<&mut Chart> = Vec::new();
        for chart in &mut self.chart_collection {
            if chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1) {
                result.push(chart);
            }
        }
        result
    }

    pub fn get_one_cell_anchor_collection(&self) -> &Vec<OneCellAnchor> {
        &self.one_cell_anchor_collection
    }

    pub fn get_one_cell_anchor_collection_mut(&mut self) -> &mut Vec<OneCellAnchor> {
        &mut self.one_cell_anchor_collection
    }

    pub fn add_one_cell_anchor_collection(&mut self, value: OneCellAnchor) -> &mut Self {
        self.one_cell_anchor_collection.push(value);
        self
    }

    pub fn get_two_cell_anchor_collection(&self) -> &Vec<TwoCellAnchor> {
        &self.two_cell_anchor_collection
    }

    pub fn get_two_cell_anchor_collection_mut(&mut self) -> &mut Vec<TwoCellAnchor> {
        &mut self.two_cell_anchor_collection
    }

    pub fn add_two_cell_anchor_collection(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor_collection.push(value);
        self
    }

    pub fn has_drawing_object(&self) -> bool {
        !self.chart_collection.is_empty()
            || !self.image_collection.is_empty()
            || !self.one_cell_anchor_collection.is_empty()
            || !self.two_cell_anchor_collection.is_empty()
    }

    pub fn get_graphic_frame_collection(&self) -> Vec<&GraphicFrame> {
        let mut result: Vec<&GraphicFrame> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_graphic_frame() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_graphic_frame_collection_mut(&mut self) -> Vec<&mut GraphicFrame> {
        let mut result: Vec<&mut GraphicFrame> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_graphic_frame_mut() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_shape_collection(&self) -> Vec<&Shape> {
        let mut result: Vec<&Shape> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_shape() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_shape_collection_mut(&mut self) -> Vec<&mut Shape> {
        let mut result: Vec<&mut Shape> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_shape_mut() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_connection_shape_collection(&self) -> Vec<&ConnectionShape> {
        let mut result: Vec<&ConnectionShape> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_connection_shape() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_connection_shape_collection_mut(&mut self) -> Vec<&mut ConnectionShape> {
        let mut result: Vec<&mut ConnectionShape> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_connection_shape_mut() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_picture_collection(&self) -> Vec<&Picture> {
        let mut result: Vec<&Picture> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_picture() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_picture_collection_mut(&mut self) -> Vec<&mut Picture> {
        let mut result: Vec<&mut Picture> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_picture_mut() {
                Some(v) => {
                    result.push(v);
                }
                None => {}
            }
        }
        result
    }
    /// (This method is crate only.)
    /// Adjustment Insert Coordinate
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // chart
        for graphic_frame in self.get_graphic_frame_collection_mut() {
            for formula in graphic_frame
                .get_graphic_mut()
                .get_graphic_data_mut()
                .get_chart_space_mut()
                .get_chart_mut()
                .get_formula_mut()
            {
                formula.get_address_mut().adjustment_insert_coordinate(
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }
        }
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // chart
        for graphic_frame in self.get_graphic_frame_collection_mut() {
            for formula in graphic_frame
                .get_graphic_mut()
                .get_graphic_data_mut()
                .get_chart_space_mut()
                .get_chart_mut()
                .get_formula_mut()
            {
                formula.get_address_mut().adjustment_remove_coordinate(
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
        ole_objects: &mut OleObjects,
    ) {
        let mut ole_index = 0;
        let mut is_alternate_content = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        is_alternate_content = true;
                    }
                    b"xdr:oneCellAnchor" => {
                        if is_alternate_content {
                            continue;
                        }
                        let mut obj = OneCellAnchor::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        if obj.is_image() {
                            let mut image = Image::default();
                            image.set_one_cell_anchor(obj);
                            self.add_image(image);
                        } else {
                            self.add_one_cell_anchor_collection(obj);
                        }
                    }
                    b"xdr:twoCellAnchor" => {
                        if is_alternate_content {
                            ole_objects.get_ole_object_mut()[ole_index]
                                .get_two_cell_anchor_mut()
                                .set_is_alternate_content(true);
                            ole_objects.get_ole_object_mut()[ole_index]
                                .get_two_cell_anchor_mut()
                                .set_attributes(reader, e, drawing_relationships);
                            ole_index += 1;
                            continue;
                        }
                        let mut obj = TwoCellAnchor::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        if obj.is_support() {
                            if obj.is_chart() {
                                let mut chart = Chart::default();
                                chart.set_two_cell_anchor(obj);
                                self.add_chart_collection(chart);
                            } else if obj.is_image() {
                                let mut image = Image::default();
                                image.set_two_cell_anchor(obj);
                                self.add_image(image);
                            } else {
                                self.add_two_cell_anchor_collection(obj);
                            }
                        }
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        is_alternate_content = false;
                    }
                    b"xdr:wsDr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:wsDr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, ole_objects: &OleObjects) {
        // xdr:wsDr
        write_start_tag(
            writer,
            "xdr:wsDr",
            vec![
                (
                    "xmlns:xdr",
                    "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
                ),
                (
                    "xmlns:a",
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                ),
            ],
            false,
        );

        // xdr:twoCellAnchor
        let mut r_id = 1;
        for chart in &self.chart_collection {
            chart.get_two_cell_anchor().write_to(writer, &mut r_id, &0);
        }
        for image in &self.image_collection {
            image.write_to(writer, &mut r_id);
        }
        for two_cell_anchor in &self.two_cell_anchor_collection {
            two_cell_anchor.write_to(writer, &mut r_id, &0);
        }

        // xdr:oneCellAnchor
        for one_cell_anchor in &self.one_cell_anchor_collection {
            one_cell_anchor.write_to(writer, &mut r_id);
        }

        // mc:AlternateContent
        let mut r_id = 1;
        let mut ole_id = 1000 + 25;
        for ole_object in ole_objects.get_ole_object() {
            ole_object
                .get_two_cell_anchor()
                .write_to(writer, &mut r_id, &ole_id);
            ole_id += 1;
        }

        write_end_tag(writer, "xdr:wsDr");
    }
}
