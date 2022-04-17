use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Fill {
    color_2: StringValue,
}
impl Fill {
    pub fn get_color_2(&self) -> &str {
        self.color_2.get_value()
    }

    pub fn set_color_2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color_2.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"color2") {
            Some(v) => {
                self.color_2.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:fill
        write_start_tag(
            writer,
            "v:fill",
            vec![("color2", self.color_2.get_value())],
            true,
        );
    }
}
