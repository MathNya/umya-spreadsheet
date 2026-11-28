use std::io::Cursor;

use umya_spreadsheet::{
    Image,
    Workbook,
    reader,
    structs::drawing::spreadsheet::MarkerType,
    writer,
};
use zip::ZipArchive;

fn picture(name: &str, data: &[u8], coordinate: &str) -> Image {
    let mut marker = MarkerType::default();
    marker.set_coordinate(coordinate);
    let mut image = Image::default();
    image.new_image_with_dimensions(80, 128, name, data, marker);
    image
}

fn write_book(book: &Workbook) -> Vec<u8> {
    let mut output = Vec::new();
    writer::xlsx::write_writer(book, &mut output).unwrap();
    output
}

fn write_and_read(book: &Workbook) -> Workbook {
    reader::xlsx::read_reader(Cursor::new(write_book(book)), true).unwrap()
}

#[test]
fn image_part_names_with_spaces_round_trip_without_collisions() {
    let images: [(&str, &[u8]); 3] = [
        ("my image.png", include_bytes!("../images/sample1.png")),
        ("my%20image.png", include_bytes!("../images/sample2.png")),
        ("image1.png", include_bytes!("../images/sample3.png")),
    ];

    let mut book = umya_spreadsheet::new_file();
    for (index, (name, data)) in images.iter().enumerate() {
        let image = picture(name, data, &format!("B{}", index + 2));
        assert_eq!(image.image_name(), *name);
        book.sheet_mut(0).unwrap().add_image(image);
    }

    for _ in 0..2 {
        let bytes = write_book(&book);
        let archive = ZipArchive::new(Cursor::new(&bytes)).unwrap();
        let parts: Vec<_> = archive
            .file_names()
            .map(Result::unwrap)
            .filter(|name| name.starts_with("xl/media/"))
            .collect();
        assert_eq!(parts.len(), images.len());
        assert!(!parts.iter().any(|name| name.contains(' ')), "{parts:?}");

        book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
        let pictures = book.sheet(0).unwrap().image_collection();
        assert_eq!(pictures.len(), images.len());
        for (picture, (_, data)) in pictures.iter().zip(images) {
            assert_eq!(picture.image_data(), data);
        }
    }
}

#[test]
fn add_image_after_reopening_a_generated_part_name() {
    let old = include_bytes!("../images/sample1.png");
    let new = include_bytes!("../images/sample2.png");
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .add_image(picture("my image.png", old, "B2"));

    let mut reopened = write_and_read(&book);
    let sheet = reopened.sheet_mut(0).unwrap();
    let part_name = sheet.image_collection()[0].image_name().to_owned();
    assert!(!part_name.contains(' '));
    sheet.add_image(picture(&part_name, new, "B6"));

    let reopened = write_and_read(&reopened);
    let images = reopened.sheet(0).unwrap().image_collection();
    assert_eq!(images.len(), 2);
    assert_eq!(images[0].image_data(), old.as_slice());
    assert!(
        images[1].image_data() == new.as_slice(),
        "second picture uses the wrong part"
    );
}

#[test]
fn image_part_names_do_not_take_a_part_on_a_lazy_sheet() {
    let old = include_bytes!("../images/sample1.png");
    let new = include_bytes!("../images/sample2.png");
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .add_image(picture("image1.png", old, "B2"));
    book.new_sheet("Edited").unwrap();

    let mut lazy = reader::xlsx::read_reader(Cursor::new(write_book(&book)), false).unwrap();
    lazy.sheet_by_name_mut("Edited")
        .unwrap()
        .add_image(picture("my image.png", new, "B2"));

    let reopened = write_and_read(&lazy);
    let first = &reopened.sheet(0).unwrap().image_collection()[0];
    let second = &reopened.sheet(1).unwrap().image_collection()[0];
    assert_eq!(first.image_name(), "image1.png");
    assert_ne!(second.image_name(), first.image_name());
    assert!(first.image_data() == old.as_slice());
    assert!(second.image_data() == new.as_slice());
}
