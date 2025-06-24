// a:gd
use std::io::Cursor;

use quick_xml::Writer;

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct ShapeGuide {
    name: Box<str>,
    fmla: Box<str>,
}
impl ShapeGuide {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, value: S) {
        self.name = value.into().into_boxed_str();
    }

    #[inline]
    #[must_use]
    pub fn fmla(&self) -> &str {
        &self.fmla
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use fmla()")]
    pub fn get_fmla(&self) -> &str {
        self.fmla()
    }

    #[inline]
    pub fn set_fmla<S: Into<String>>(&mut self, value: S) {
        self.fmla = value.into().into_boxed_str();
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:gd",
            vec![("name", &self.name).into(), ("fmla", &self.fmla).into()],
            true,
        );
    }
}
