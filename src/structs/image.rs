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
/// ## Add Image
/// ```rust
/// extern crate umya_spreadsheet;
/// let mut book = umya_spreadsheet::new_file();
///
/// let mut marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
/// marker.set_coordinate("B3");
/// let mut image = umya_spreadsheet::structs::Image::default();
/// image.new_image("./images/sample1.png", marker);
/// book.get_sheet_by_name_mut("Sheet1")
///     .unwrap()
///     .add_image(image);
/// ```
///
/// ## Download Image
/// ```rust
/// book.get_sheet_by_name("Sheet1")
/// .unwrap()
/// .get_image_collection()
/// .get(0)
/// .unwrap()
/// .download_image("./tests/result_files/bbb.png");
/// ```
///
/// ## Change Image
/// ```rust
/// book.get_sheet_by_name_mut("Sheet1")
/// .unwrap()
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

    pub fn new_image<S: Into<String>>(&mut self, path: S, marker: MarkerType) {
        let path_str = path.into();
        let path_obj = std::path::Path::new(path_str.as_str());
        let image_name = path_obj.file_name().unwrap().to_str().unwrap();

        let img = image::open(path_obj).unwrap();
        let (width, height) = img.dimensions();

        let mut file = File::open(path_str.as_str()).unwrap();
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

        picture
            .get_non_visual_picture_properties_mut()
            .get_non_visual_picture_drawing_properties_mut()
            .get_picture_locks_mut()
            .set_no_change_aspect(true);

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

    pub fn change_image<S: Into<String>>(&mut self, path: S) {
        let marker = match self.get_two_cell_anchor() {
            Some(v) => v.get_from_marker(),
            None => match self.get_one_cell_anchor() {
                Some(v) => v.get_from_marker(),
                None => {
                    panic!("Not Found MediaObject");
                }
            },
        }
        .clone();
        self.two_cell_anchor = None;
        self.one_cell_anchor = None;
        self.new_image(path, marker);
    }

    pub fn download_image<S: Into<String>>(&self, path: S) {
        fs::write(path.into(), self.get_media_object().get_image_data()).unwrap();
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
