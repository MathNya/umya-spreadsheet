// xdr:wsDr
use super::ConnectionShape;
use super::GraphicFrame;
use super::OneCellAnchor;
use super::Picture;
use super::Shape;
use super::TwoCellAnchor;
use helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::Chart;
use structs::Image;
use structs::OleObjects;
use traits::AdjustmentCoordinate;
use traits::AdjustmentCoordinateWithSheet;
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
        self.image_collection
            .iter_mut()
            .find(|image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
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
        self.chart_collection
            .iter_mut()
            .find(|chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
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
            if let Some(v) = two_cell_anchor.get_graphic_frame() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_graphic_frame_collection_mut(&mut self) -> Vec<&mut GraphicFrame> {
        let mut result: Vec<&mut GraphicFrame> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_graphic_frame_mut() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_shape_collection(&self) -> Vec<&Shape> {
        let mut result: Vec<&Shape> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_shape() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_shape_collection_mut(&mut self) -> Vec<&mut Shape> {
        let mut result: Vec<&mut Shape> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_shape_mut() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_connection_shape_collection(&self) -> Vec<&ConnectionShape> {
        let mut result: Vec<&ConnectionShape> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_connection_shape() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_connection_shape_collection_mut(&mut self) -> Vec<&mut ConnectionShape> {
        let mut result: Vec<&mut ConnectionShape> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_connection_shape_mut() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_picture_collection(&self) -> Vec<&Picture> {
        let mut result: Vec<&Picture> = Vec::new();
        for two_cell_anchor in &self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_picture() {
                result.push(v);
            }
        }
        result
    }

    pub fn get_one_cell_anchor_all_list(&mut self) -> Vec<&mut OneCellAnchor> {
        let mut result: Vec<&mut OneCellAnchor> = Vec::new();
        for anchor in &mut self.one_cell_anchor_collection {
            result.push(anchor);
        }
        for image in &mut self.image_collection {
            match image.get_one_cell_anchor_mut() {
                Some(anchor) => {
                    result.push(anchor);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_two_cell_anchor_all_list(&mut self) -> Vec<&mut TwoCellAnchor> {
        let mut result: Vec<&mut TwoCellAnchor> = Vec::new();
        for anchor in &mut self.two_cell_anchor_collection {
            result.push(anchor);
        }
        for chart in &mut self.chart_collection {
            let mut anchor = chart.get_two_cell_anchor_mut();
            result.push(anchor);
        }
        for image in &mut self.image_collection {
            match image.get_two_cell_anchor_mut() {
                Some(anchor) => {
                    result.push(anchor);
                }
                None => {}
            }
        }
        result
    }

    pub fn get_picture_collection_mut(&mut self) -> Vec<&mut Picture> {
        let mut result: Vec<&mut Picture> = Vec::new();
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            if let Some(v) = two_cell_anchor.get_picture_mut() {
                result.push(v);
            }
        }
        result
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

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                        let os = ole_objects.get_ole_object_mut();
                        if is_alternate_content && !os.is_empty() {
                            os[ole_index]
                                .get_two_cell_anchor_mut()
                                .set_is_alternate_content(true);
                            os[ole_index].get_two_cell_anchor_mut().set_attributes(
                                reader,
                                e,
                                drawing_relationships,
                            );
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
                }
            },

            Event::End(ref e) => {
                match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        is_alternate_content = false;
                    }
                    b"xdr:wsDr" => return,
                    _ => (),
                }
            },

            Event::Eof => panic!("Error: Could not find {} end element", "xdr:wsDr")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        ole_objects: &OleObjects,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:wsDr
        write_start_tag(
            writer,
            "xdr:wsDr",
            vec![
                ("xmlns:xdr", SHEET_DRAWING_NS),
                ("xmlns:a", DRAWINGML_MAIN_NS),
            ],
            false,
        );

        // xdr:twoCellAnchor
        for chart in &self.chart_collection {
            chart.get_two_cell_anchor().write_to(writer, rel_list, &0);
        }
        for image in &self.image_collection {
            image.write_to(writer, rel_list);
        }
        for two_cell_anchor in &self.two_cell_anchor_collection {
            two_cell_anchor.write_to(writer, rel_list, &0);
        }

        // xdr:oneCellAnchor
        for one_cell_anchor in &self.one_cell_anchor_collection {
            one_cell_anchor.write_to(writer, rel_list);
        }

        // mc:AlternateContent
        let mut ole_id = 1000 + 25;
        for ole_object in ole_objects.get_ole_object() {
            ole_object
                .get_two_cell_anchor()
                .write_to(writer, rel_list, &0);
            ole_id += 1;
        }

        write_end_tag(writer, "xdr:wsDr");
    }
}
impl AdjustmentCoordinate for WorksheetDrawing {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for anchor in &mut self.one_cell_anchor_collection {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for anchor in &mut self.two_cell_anchor_collection {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for chart in &mut self.chart_collection {
            chart.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for image in &mut self.image_collection {
            image.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        &mut self.one_cell_anchor_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for anchor in &mut self.one_cell_anchor_collection {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.two_cell_anchor_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for anchor in &mut self.two_cell_anchor_collection {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.chart_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for chart in &mut self.chart_collection {
            chart.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.image_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for image in &mut self.image_collection {
            image.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
impl AdjustmentCoordinateWithSheet for WorksheetDrawing {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // chart
        for chart in &mut self.chart_collection {
            chart.adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
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
        // chart
        for chart in &mut self.chart_collection {
            chart.adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
