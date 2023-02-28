use image::GenericImageView;
use quick_xml::Writer;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use structs::drawing::spreadsheet::MarkerType;
use structs::drawing::spreadsheet::OneCellAnchor;
use structs::drawing::spreadsheet::Picture;
use structs::drawing::spreadsheet::TwoCellAnchor;
use structs::drawing::FillRectangle;
use structs::drawing::PresetGeometry;
use structs::drawing::Stretch;
use structs::MediaObject;

#[derive(Clone, Default, Debug)]
pub struct Image {
    two_cell_anchor: Option<TwoCellAnchor>,
    one_cell_anchor: Option<OneCellAnchor>,
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
/// book.get_sheet_by_name_mut("Sheet1").unwrap()
///     .add_image(image);
///
/// // Get Image by Worksheet.
/// let worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// let image = worksheet.get_image("B3");
/// let image = worksheet.get_image_by_column_and_row(&2, &1);
/// let image = worksheet.get_image_mut("B3");
/// let image = worksheet.get_image_by_column_and_row_mut(&2, &1);
///
/// // Use this if there are multiple Images in a given cell.
/// let images = worksheet.get_images("B3");
/// let images = worksheet.get_images_by_column_and_row(&2, &1);
/// let images = worksheet.get_images_mut("B3");
/// let images = worksheet.get_images_by_column_and_row_mut(&2, &1);
///
/// // Download Image
/// book.get_sheet_by_name("Sheet1").unwrap()
/// .get_image_collection()
/// .get(0)
/// .unwrap()
/// .download_image("./tests/result_files/bbb.png");
///
/// // Change Image
/// book.get_sheet_by_name_mut("Sheet1").unwrap()
/// .get_image_collection_mut()
/// .get_mut(0)
/// .unwrap()
/// .change_image("./images/sample1.png");
/// ```
impl Image {
    pub fn get_two_cell_anchor(&self) -> &Option<TwoCellAnchor> {
        &self.two_cell_anchor
    }

    pub fn get_two_cell_anchor_mut(&mut self) -> &mut Option<TwoCellAnchor> {
        &mut self.two_cell_anchor
    }

    pub fn set_two_cell_anchor(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor = Some(value);
        self
    }

    pub fn get_one_cell_anchor(&self) -> &Option<OneCellAnchor> {
        &self.one_cell_anchor
    }

    pub fn get_one_cell_anchor_mut(&mut self) -> &mut Option<OneCellAnchor> {
        &mut self.one_cell_anchor
    }

    pub fn set_one_cell_anchor(&mut self, value: OneCellAnchor) -> &mut Self {
        self.one_cell_anchor = Some(value);
        self
    }

    pub fn new_image(&mut self, path: &str, marker: MarkerType) {
        let path_str = path;
        let path_obj = std::path::Path::new(path_str);
        let image_name = path_obj.file_name().unwrap().to_str().unwrap();

        let img = image::open(path_obj).unwrap();
        let (width, height) = img.dimensions();

        let mut file = File::open(path_str).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();

        let mut picture = Picture::default();
        // filename and filedata.
        picture
            .get_blip_fill_mut()
            .get_blip_mut()
            .set_cstate("print")
            .get_image_mut()
            .set_image_name(image_name)
            .set_image_data(buf);

        // name
        picture
            .get_non_visual_picture_properties_mut()
            .get_non_visual_drawing_properties_mut()
            .set_name(image_name);

        // prefer_relative_resize
        picture
            .get_non_visual_picture_properties_mut()
            .get_non_visual_picture_drawing_properties_mut()
            .set_prefer_relative_resize(false);

        // stretch
        let fill_rectangle = FillRectangle::default();
        let mut stretch = Stretch::default();
        stretch.set_fill_rectangle(fill_rectangle);
        picture.get_blip_fill_mut().set_stretch(stretch);

        // geometry
        picture
            .get_shape_properties_mut()
            .get_geometry_mut()
            .set_geometry(PresetGeometry::GEOMETRY_RECT);

        let mut one_cell_anchor = OneCellAnchor::default();
        one_cell_anchor.set_from_marker(marker);
        one_cell_anchor
            .get_extent_mut()
            .set_cy(height as i64 * 9525);
        one_cell_anchor.get_extent_mut().set_cx(width as i64 * 9525);
        one_cell_anchor.set_picture(picture);
        self.one_cell_anchor = Some(one_cell_anchor);
    }

    pub fn change_image(&mut self, path: &str) {
        let marker = self.get_from_marker_type().clone();
        self.two_cell_anchor = None;
        self.one_cell_anchor = None;
        self.new_image(path, marker);
    }

    pub fn download_image(&self, path: &str) {
        fs::write(path, self.get_media_object().get_image_data()).unwrap();
    }

    pub fn get_coordinate(&self) -> String {
        self.get_from_marker_type().get_coordinate()
    }

    pub(crate) fn get_col(&self) -> &u32 {
        self.get_from_marker_type().get_col()
    }

    pub(crate) fn get_row(&self) -> &u32 {
        self.get_from_marker_type().get_row()
    }

    pub fn get_from_marker_type(&self) -> &MarkerType {
        match &self.two_cell_anchor {
            Some(anchor) => {
                return anchor.get_from_marker();
            }
            None => {}
        }
        match &self.one_cell_anchor {
            Some(anchor) => {
                return anchor.get_from_marker();
            }
            None => {}
        }
        panic!("Not Found MediaObject");
    }

    pub fn get_to_marker_type(&self) -> Option<&MarkerType> {
        match &self.two_cell_anchor {
            Some(anchor) => {
                return Some(anchor.get_to_marker());
            }
            None => None,
        }
    }

    pub(crate) fn get_media_object(&self) -> &MediaObject {
        match &self.two_cell_anchor {
            Some(anchor) => match anchor.get_picture() {
                Some(v) => {
                    return v.get_blip_fill().get_blip().get_image();
                }
                None => {}
            },
            None => {}
        }
        match &self.one_cell_anchor {
            Some(anchor) => match anchor.get_picture() {
                Some(v) => {
                    return v.get_blip_fill().get_blip().get_image();
                }
                None => {}
            },
            None => {}
        }
        panic!("Not Found MediaObject");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut i32) {
        match &self.two_cell_anchor {
            Some(anchor) => {
                anchor.write_to(writer, r_id, &0);
            }
            None => {}
        }
        match &self.one_cell_anchor {
            Some(anchor) => {
                anchor.write_to(writer, r_id);
            }
            None => {}
        }
    }
}
