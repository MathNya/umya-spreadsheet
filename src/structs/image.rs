use std::{
    fs,
    fs::File,
    io::{
        BufReader,
        Cursor,
        Read,
    },
};

use base64::{
    Engine as _,
    engine::general_purpose::STANDARD,
};
use quick_xml::Writer;

use crate::{
    structs::{
        MediaObject,
        drawing::{
            FillRectangle,
            PresetGeometry,
            Stretch,
            spreadsheet::{
                MarkerType,
                OneCellAnchor,
                Picture,
                TwoCellAnchor,
            },
        },
    },
    traits::AdjustmentCoordinate,
};

#[derive(Clone, Default, Debug)]
pub struct Image {
    two_cell_anchor: Option<Box<TwoCellAnchor>>,
    one_cell_anchor: Option<Box<OneCellAnchor>>,
}
/// ## Example
/// ```rust
/// extern crate umya_spreadsheet;
/// let mut book = umya_spreadsheet::new_file();
///
/// // Add Image
/// let mut marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
/// marker.set_coordinate("B3");
/// let mut image = umya_spreadsheet::structs::Image::default();
/// image.new_image("./images/sample1.png", marker);
/// book.get_sheet_by_name_mut("Sheet1")
///     .unwrap()
///     .add_image(image);
///
/// // Get Image by Worksheet.
/// let worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// let image = worksheet.get_image("B3");
/// let image = worksheet.get_image_by_column_and_row(&2, 1);
/// let image = worksheet.get_image_mut("B3");
/// let image = worksheet.get_image_by_column_and_row_mut(&2, 1);
///
/// // Use this if there are multiple Images in a given cell.
/// let images = worksheet.get_images("B3");
/// let images = worksheet.get_images_by_column_and_row(&2, 1);
/// let images = worksheet.get_images_mut("B3");
/// let images = worksheet.get_images_by_column_and_row_mut(&2, 1);
///
/// // Download Image
/// book.get_sheet_by_name("Sheet1")
///     .unwrap()
///     .get_image_collection()
///     .get(0)
///     .unwrap()
///     .download_image("./tests/result_files/bbb.png");
///
/// // Change Image
/// book.get_sheet_by_name_mut("Sheet1")
///     .unwrap()
///     .get_image_collection_mut()
///     .get_mut(0)
///     .unwrap()
///     .change_image("./images/sample1.png");
/// ```
impl Image {
    #[inline]
    #[must_use]
    pub fn two_cell_anchor(&self) -> Option<&TwoCellAnchor> {
        self.two_cell_anchor.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use two_cell_anchor()")]
    pub fn get_two_cell_anchor(&self) -> Option<&TwoCellAnchor> {
        self.two_cell_anchor()
    }

    #[inline]
    pub fn two_cell_anchor_mut(&mut self) -> Option<&mut TwoCellAnchor> {
        self.two_cell_anchor.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use two_cell_anchor_mut()")]
    pub fn get_two_cell_anchor_mut(&mut self) -> Option<&mut TwoCellAnchor> {
        self.two_cell_anchor_mut()
    }

    #[inline]
    pub fn set_two_cell_anchor(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_two_cell_anchor(&mut self) -> &mut Self {
        self.two_cell_anchor = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn one_cell_anchor(&self) -> Option<&OneCellAnchor> {
        self.one_cell_anchor.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use one_cell_anchor()")]
    pub fn get_one_cell_anchor(&self) -> Option<&OneCellAnchor> {
        self.one_cell_anchor()
    }

    #[inline]
    pub fn one_cell_anchor_mut(&mut self) -> Option<&mut OneCellAnchor> {
        self.one_cell_anchor.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use one_cell_anchor_mut()")]
    pub fn get_one_cell_anchor_mut(&mut self) -> Option<&mut OneCellAnchor> {
        self.one_cell_anchor_mut()
    }

    #[inline]
    pub fn set_one_cell_anchor(&mut self, value: OneCellAnchor) -> &mut Self {
        self.one_cell_anchor = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_one_cell_anchor(&mut self) -> &mut Self {
        self.one_cell_anchor = None;
        self
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn new_image(&mut self, path: &str, marker: MarkerType) {
        let path = std::path::Path::new(path);

        let size = imagesize::size(path).unwrap();
        let image_name = path.file_name().unwrap().to_str().unwrap();
        let mut buf = Vec::new();

        let file = File::open(path).unwrap();
        BufReader::new(file).read_to_end(&mut buf).unwrap();

        self.new_image_with_dimensions(size.height as u32, size.width as u32, image_name, buf, marker);
    }

    pub fn new_image_with_dimensions<B: Into<Vec<u8>>>(
        &mut self,
        height: u32,
        width: u32,
        image_name: &str,
        bytes: B,
        marker: MarkerType,
    ) {
        let mut picture = Picture::default();
        // filename and filedata.
        picture
            .blip_fill_mut()
            .blip_mut()
            .set_cstate("print")
            .image_mut()
            .set_image_name(image_name)
            .set_image_data(bytes.into());

        // name
        picture
            .non_visual_picture_properties_mut()
            .non_visual_drawing_properties_mut()
            .set_name(image_name);

        // prefer_relative_resize
        picture
            .non_visual_picture_properties_mut()
            .non_visual_picture_drawing_properties_mut()
            .set_prefer_relative_resize(false);

        // stretch
        let fill_rectangle = FillRectangle::default();
        let mut stretch = Stretch::default();
        stretch.set_fill_rectangle(fill_rectangle);
        picture.blip_fill_mut().set_stretch(stretch);

        // geometry
        picture
            .shape_properties_mut()
            .geometry_mut()
            .set_geometry(PresetGeometry::GEOMETRY_RECT);

        let mut one_cell_anchor = OneCellAnchor::default();
        one_cell_anchor.set_from_marker(marker);
        one_cell_anchor
            .extent_mut()
            .set_cy(i64::from(height) * 9525);
        one_cell_anchor
            .extent_mut()
            .set_cx(i64::from(width) * 9525);
        one_cell_anchor.set_picture(picture);
        self.set_one_cell_anchor(one_cell_anchor);
    }

    #[inline]
    pub fn change_image(&mut self, path: &str) {
        let marker = self.get_from_marker_type().clone();
        self.remove_two_cell_anchor();
        self.remove_one_cell_anchor();
        self.new_image(path, marker);
    }

    #[inline]
    pub fn download_image(&self, path: &str) {
        fs::write(path, self.get_image_data()).unwrap();
    }

    #[inline]
    #[must_use]
    pub fn has_image(&self) -> bool {
        !self.get_media_object().is_empty()
    }

    #[inline]
    #[must_use]
    pub fn get_image_name(&self) -> &str {
        match self.get_media_object().first() {
            Some(v) => v.image_name(),
            None => "",
        }
    }

    #[inline]
    #[must_use]
    pub fn get_image_data(&self) -> &[u8] {
        match self.get_media_object().first() {
            Some(v) => v.image_data(),
            None => &[0u8; 0],
        }
    }

    #[inline]
    #[must_use]
    pub fn get_image_data_base64(&self) -> String {
        STANDARD.encode(self.get_image_data())
    }

    #[inline]
    #[must_use]
    pub fn get_coordinate(&self) -> String {
        self.get_from_marker_type().coordinate()
    }

    #[inline]
    #[must_use]
    pub fn get_col(&self) -> u32 {
        self.get_from_marker_type().col()
    }

    #[inline]
    #[must_use]
    pub fn get_row(&self) -> u32 {
        self.get_from_marker_type().row()
    }

    #[inline]
    #[must_use]
    pub fn get_from_marker_type(&self) -> &MarkerType {
        if let Some(anchor) = self.two_cell_anchor() {
            return anchor.from_marker();
        }
        if let Some(anchor) = self.one_cell_anchor() {
            return anchor.from_marker();
        }
        panic!("Not Found MediaObject");
    }

    #[inline]
    #[must_use]
    pub fn get_to_marker_type(&self) -> Option<&MarkerType> {
        self.two_cell_anchor()
            .as_ref()
            .map(|anchor| anchor.to_marker())
    }

    pub(crate) fn get_media_object(&self) -> Vec<&MediaObject> {
        let mut result: Vec<&MediaObject> = Vec::new();
        if let Some(anchor) = self.two_cell_anchor() {
            if let Some(v) = anchor.picture() {
                result.push(v.blip_fill().blip().image());
            }
            if let Some(v) = anchor.shape() {
                if let Some(bf) = v.shape_properties().blip_fill() {
                    result.push(bf.blip().image());
                }
            }
            if let Some(v) = anchor.connection_shape() {
                if let Some(bf) = v.shape_properties().blip_fill() {
                    result.push(bf.blip().image());
                }
            }
            if let Some(v) = anchor.group_shape() {
                for pic in v.picture_collection() {
                    result.push(pic.blip_fill().blip().image());
                }
                for shp in v.shape_collection() {
                    if let Some(bf) = shp.shape_properties().blip_fill() {
                        result.push(bf.blip().image());
                    }
                }
            }
        }
        if let Some(anchor) = self.one_cell_anchor() {
            if let Some(v) = anchor.picture() {
                result.push(v.blip_fill().blip().image());
            }
            if let Some(v) = anchor.shape() {
                if let Some(bf) = v.shape_properties().blip_fill() {
                    result.push(bf.blip().image());
                }
            }
            if let Some(v) = anchor.group_shape() {
                for pic in v.picture_collection() {
                    result.push(pic.blip_fill().blip().image());
                }
                for shp in v.shape_collection() {
                    if let Some(bf) = shp.shape_properties().blip_fill() {
                        result.push(bf.blip().image());
                    }
                }
            }
        }
        result
    }

    #[inline]
    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        if let Some(anchor) = self.two_cell_anchor() {
            anchor.write_to(writer, rel_list, 0);
        }
        if let Some(anchor) = self.one_cell_anchor() {
            anchor.write_to(writer, rel_list);
        }
    }
}
impl AdjustmentCoordinate for Image {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        // one_cell_anchor
        if let Some(anchor) = self.one_cell_anchor.as_mut() {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // two_cell_anchor
        if let Some(anchor) = self.two_cell_anchor.as_mut() {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        // one_cell_anchor
        if let Some(anchor) = self.one_cell_anchor.as_mut() {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // two_cell_anchor
        if let Some(anchor) = self.two_cell_anchor.as_mut() {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        if let Some(anchor) = self.one_cell_anchor.as_ref() {
            return anchor.is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        if let Some(anchor) = self.two_cell_anchor.as_ref() {
            return anchor.is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        false
    }
}
