// a:gd
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ShapeGuide {
    name: Box<str>,
    fmla: Box<str>,
}
impl ShapeGuide {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) {
        self.name = value.into().into_boxed_str();
    }

    pub fn get_fmla(&self) -> &str {
        &self.fmla
    }

    pub fn set_fmla<S: Into<String>>(&mut self, value: S) {
        self.fmla = value.into().into_boxed_str();
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:gd",
            vec![("name", &self.name), ("fmla", &self.fmla)],
            true,
        );
    }
}
