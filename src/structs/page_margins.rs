use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    structs::DoubleValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PageMargins {
    left:   DoubleValue,
    right:  DoubleValue,
    top:    DoubleValue,
    bottom: DoubleValue,
    header: DoubleValue,
    footer: DoubleValue,
}
impl PageMargins {
    #[inline]
    #[must_use]
    pub fn left(&self) -> f64 {
        self.left.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use left()")]
    pub fn get_left(&self) -> f64 {
        self.left()
    }

    #[inline]
    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn right(&self) -> f64 {
        self.right.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use right()")]
    pub fn get_right(&self) -> f64 {
        self.right()
    }

    #[inline]
    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn top(&self) -> f64 {
        self.top.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use top()")]
    pub fn get_top(&self) -> f64 {
        self.top()
    }

    #[inline]
    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn bottom(&self) -> f64 {
        self.bottom.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bottom()")]
    pub fn get_bottom(&self) -> f64 {
        self.bottom()
    }

    #[inline]
    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn header(&self) -> f64 {
        self.header.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use header()")]
    pub fn get_header(&self) -> f64 {
        self.header()
    }

    #[inline]
    pub fn set_header(&mut self, value: f64) -> &mut Self {
        self.header.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn footer(&self) -> f64 {
        self.footer.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use footer()")]
    pub fn get_footer(&self) -> f64 {
        self.footer()
    }

    #[inline]
    pub fn set_footer(&mut self, value: f64) -> &mut Self {
        self.footer.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.left
            .set_value_string(get_attribute(e, b"left").unwrap());
        self.right
            .set_value_string(get_attribute(e, b"right").unwrap());
        self.top.set_value_string(get_attribute(e, b"top").unwrap());
        self.bottom
            .set_value_string(get_attribute(e, b"bottom").unwrap());
        self.header
            .set_value_string(get_attribute(e, b"header").unwrap());
        self.footer
            .set_value_string(get_attribute(e, b"footer").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pageMargins
        // If all margins are zero (common for minimal template sheets),
        // use Excel-compatible defaults to avoid "Line 2, column 0" errors.
        let is_all_zero = self.left.value() == 0.0
            && self.right.value() == 0.0
            && self.top.value() == 0.0
            && self.bottom.value() == 0.0
            && self.header.value() == 0.0
            && self.footer.value() == 0.0;

        let (left, right, top, bottom, header, footer) = if is_all_zero {
            (
                "0.7".to_string(),  "0.7".to_string(),
                "0.75".to_string(), "0.75".to_string(),
                "0.3".to_string(),  "0.3".to_string(),
            )
        } else {
            (
                self.left.value_string(),   self.right.value_string(),
                self.top.value_string(),    self.bottom.value_string(),
                self.header.value_string(), self.footer.value_string(),
            )
        };

        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("left", &left).into());
        attributes.push(("right", &right).into());
        attributes.push(("top", &top).into());
        attributes.push(("bottom", &bottom).into());
        attributes.push(("header", &header).into());
        attributes.push(("footer", &footer).into());
        write_start_tag(writer, "pageMargins", attributes, true);
    }
}
