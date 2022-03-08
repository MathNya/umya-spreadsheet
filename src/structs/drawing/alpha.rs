// a:alpha
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Alpha {
    val: String,
}
impl Alpha {
    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) {
        self.val = value.into();
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:alpha
        write_start_tag(writer, "a:alpha", vec![("val", &self.val)], true);
    }
}
