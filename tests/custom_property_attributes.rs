use std::io::Cursor;

use umya_spreadsheet::{
    custom_properties::CustomDocumentProperty,
    reader,
    writer,
};

#[test]
fn round_trip_custom_property_attributes() {
    let name = "R&D <team> &amp;";
    let target = "one & two";
    let mut book = umya_spreadsheet::new_file();
    let mut property = CustomDocumentProperty::default();
    property
        .set_name(name)
        .set_link_target(target)
        .set_value_string("A&B");
    book.properties_mut()
        .custom_properties_mut()
        .add_custom_document_property_list(property);

    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let restored = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let property = &restored
        .properties()
        .custom_properties()
        .custom_document_property_list()[0];
    assert_eq!(property.name(), name);
    assert_eq!(property.link_target(), target);
    assert_eq!(property.value(), "A&B");
}
