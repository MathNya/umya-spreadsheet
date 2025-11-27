// a:clrScheme
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    super::StringValue,
    Color2Type,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ColorScheme {
    name:      StringValue,
    accent1:   Color2Type,
    accent2:   Color2Type,
    accent3:   Color2Type,
    accent4:   Color2Type,
    accent5:   Color2Type,
    accent6:   Color2Type,
    dk1:       Color2Type,
    dk2:       Color2Type,
    fol_hlink: Color2Type,
    hlink:     Color2Type,
    lt1:       Color2Type,
    lt2:       Color2Type,
}

impl ColorScheme {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn accent1(&self) -> &Color2Type {
        &self.accent1
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent1()")]
    pub fn get_accent1(&self) -> &Color2Type {
        self.accent1()
    }

    #[inline]
    pub fn accent1_mut(&mut self) -> &mut Color2Type {
        &mut self.accent1
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent1_mut()")]
    pub fn get_accent1_mut(&mut self) -> &mut Color2Type {
        self.accent1_mut()
    }

    #[inline]
    pub fn set_accent1(&mut self, value: Color2Type) -> &mut Self {
        self.accent1 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn accent2(&self) -> &Color2Type {
        &self.accent2
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent2()")]
    pub fn get_accent2(&self) -> &Color2Type {
        self.accent2()
    }

    #[inline]
    pub fn accent2_mut(&mut self) -> &mut Color2Type {
        &mut self.accent2
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent2_mut()")]
    pub fn get_accent2_mut(&mut self) -> &mut Color2Type {
        self.accent2_mut()
    }

    #[inline]
    pub fn set_accent2(&mut self, value: Color2Type) -> &mut Self {
        self.accent2 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn accent3(&self) -> &Color2Type {
        &self.accent3
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent3()")]
    pub fn get_accent3(&self) -> &Color2Type {
        self.accent3()
    }

    #[inline]
    pub fn accent3_mut(&mut self) -> &mut Color2Type {
        &mut self.accent3
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent3_mut()")]
    pub fn get_accent3_mut(&mut self) -> &mut Color2Type {
        self.accent3_mut()
    }

    #[inline]
    pub fn set_accent3(&mut self, value: Color2Type) -> &mut Self {
        self.accent3 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn accent4(&self) -> &Color2Type {
        &self.accent4
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent4()")]
    pub fn get_accent4(&self) -> &Color2Type {
        self.accent4()
    }

    #[inline]
    pub fn accent4_mut(&mut self) -> &mut Color2Type {
        &mut self.accent4
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent4_mut()")]
    pub fn get_accent4_mut(&mut self) -> &mut Color2Type {
        self.accent4_mut()
    }

    #[inline]
    pub fn set_accent4(&mut self, value: Color2Type) -> &mut Self {
        self.accent4 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn accent5(&self) -> &Color2Type {
        &self.accent5
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent5()")]
    pub fn get_accent5(&self) -> &Color2Type {
        self.accent5()
    }

    #[inline]
    pub fn accent5_mut(&mut self) -> &mut Color2Type {
        &mut self.accent5
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent5_mut()")]
    pub fn get_accent5_mut(&mut self) -> &mut Color2Type {
        self.accent5_mut()
    }

    #[inline]
    pub fn set_accent5(&mut self, value: Color2Type) -> &mut Self {
        self.accent5 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn accent6(&self) -> &Color2Type {
        &self.accent6
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use accent6()")]
    pub fn get_accent6(&self) -> &Color2Type {
        self.accent6()
    }

    #[inline]
    pub fn accent6_mut(&mut self) -> &mut Color2Type {
        &mut self.accent6
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use accent6_mut()")]
    pub fn get_accent6_mut(&mut self) -> &mut Color2Type {
        self.accent6_mut()
    }

    #[inline]
    pub fn set_accent6(&mut self, value: Color2Type) -> &mut Self {
        self.accent6 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn dk1(&self) -> &Color2Type {
        &self.dk1
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use dk1()")]
    pub fn get_dk1(&self) -> &Color2Type {
        self.dk1()
    }

    #[inline]
    pub fn dk1_mut(&mut self) -> &mut Color2Type {
        &mut self.dk1
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use dk1_mut()")]
    pub fn get_dk1_mut(&mut self) -> &mut Color2Type {
        self.dk1_mut()
    }

    #[inline]
    pub fn set_dk1(&mut self, value: Color2Type) -> &mut Self {
        self.dk1 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn dk2(&self) -> &Color2Type {
        &self.dk2
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use dk2()")]
    pub fn get_dk2(&self) -> &Color2Type {
        self.dk2()
    }

    #[inline]
    pub fn dk2_mut(&mut self) -> &mut Color2Type {
        &mut self.dk2
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use dk2_mut()")]
    pub fn get_dk2_mut(&mut self) -> &mut Color2Type {
        self.dk2_mut()
    }

    #[inline]
    pub fn set_dk2(&mut self, value: Color2Type) -> &mut Self {
        self.dk2 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn fol_hlink(&self) -> &Color2Type {
        &self.fol_hlink
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use fol_hlink()")]
    pub fn get_fol_hlink(&self) -> &Color2Type {
        self.fol_hlink()
    }

    #[inline]
    pub fn fol_hlink_mut(&mut self) -> &mut Color2Type {
        &mut self.fol_hlink
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use fol_hlink_mut()")]
    pub fn get_fol_hlink_mut(&mut self) -> &mut Color2Type {
        self.fol_hlink_mut()
    }

    #[inline]
    pub fn set_fol_hlink(&mut self, value: Color2Type) -> &mut Self {
        self.fol_hlink = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn hlink(&self) -> &Color2Type {
        &self.hlink
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use hlink()")]
    pub fn get_hlink(&self) -> &Color2Type {
        self.hlink()
    }

    #[inline]
    pub fn hlink_mut(&mut self) -> &mut Color2Type {
        &mut self.hlink
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hlink_mut()")]
    pub fn get_hlink_mut(&mut self) -> &mut Color2Type {
        self.hlink_mut()
    }

    #[inline]
    pub fn set_hlink(&mut self, value: Color2Type) -> &mut Self {
        self.hlink = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn lt1(&self) -> &Color2Type {
        &self.lt1
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use lt1()")]
    pub fn get_lt1(&self) -> &Color2Type {
        self.lt1()
    }

    #[inline]
    pub fn lt1_mut(&mut self) -> &mut Color2Type {
        &mut self.lt1
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use lt1_mut()")]
    pub fn get_lt1_mut(&mut self) -> &mut Color2Type {
        self.lt1_mut()
    }

    #[inline]
    pub fn set_lt1(&mut self, value: Color2Type) -> &mut Self {
        self.lt1 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn lt2(&self) -> &Color2Type {
        &self.lt2
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use lt2()")]
    pub fn get_lt2(&self) -> &Color2Type {
        self.lt2()
    }

    #[inline]
    pub fn lt2_mut(&mut self) -> &mut Color2Type {
        &mut self.lt2
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use lt2_mut()")]
    pub fn get_lt2_mut(&mut self) -> &mut Color2Type {
        self.lt2_mut()
    }

    #[inline]
    pub fn set_lt2(&mut self, value: Color2Type) -> &mut Self {
        self.lt2 = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn color_map(&self) -> Vec<String> {
        vec![
            self.lt1.val(),
            self.dk1.val(),
            self.lt2.val(),
            self.dk2.val(),
            self.accent1.val(),
            self.accent2.val(),
            self.accent3.val(),
            self.accent4.val(),
            self.accent5.val(),
            self.accent6.val(),
            self.hlink.val(),
            self.fol_hlink.val(),
        ]
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use color_map()")]
    pub fn get_color_map(&self) -> Vec<String> {
        self.color_map()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:accent1" => {
                        self.accent1.set_attributes(reader, e);
                    }
                    b"a:accent2" => {
                        self.accent2.set_attributes(reader, e);
                    }
                    b"a:accent3" => {
                        self.accent3.set_attributes(reader, e);
                    }
                    b"a:accent4" => {
                        self.accent4.set_attributes(reader, e);
                    }
                    b"a:accent5" => {
                        self.accent5.set_attributes(reader, e);
                    }
                    b"a:accent6" => {
                        self.accent6.set_attributes(reader, e);
                    }
                    b"a:dk1" => {
                        self.dk1.set_attributes(reader, e);
                    }
                    b"a:dk2" => {
                        self.dk2.set_attributes(reader, e);
                    }
                    b"a:folHlink" => {
                        self.fol_hlink.set_attributes(reader, e);
                    }
                    b"a:hlink" => {
                        self.hlink.set_attributes(reader, e);
                    }
                    b"a:lt1" => {
                        self.lt1.set_attributes(reader, e);
                    }
                    b"a:lt2" => {
                        self.lt2.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:clrScheme" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:clrScheme")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:clrScheme
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.value_str()).into());
        }
        write_start_tag(writer, "a:clrScheme", attributes, false);

        // a:dk1
        self.dk1.write_to_dk1(writer);

        // a:lt1
        self.lt1.write_to_lt1(writer);

        // a:dk2
        self.dk2.write_to_dk2(writer);

        // a:lt2
        self.lt2.write_to_lt2(writer);

        // a:accent1
        self.accent1.write_to_accent1(writer);

        // a:accent2
        self.accent2.write_to_accent2(writer);

        // a:accent3
        self.accent3.write_to_accent3(writer);

        // a:accent4
        self.accent4.write_to_accent4(writer);

        // a:accent5
        self.accent5.write_to_accent5(writer);

        // a:accent6
        self.accent6.write_to_accent6(writer);

        // a:hlink
        self.hlink.write_to_hlink(writer);

        // a:folHlink
        self.fol_hlink.write_to_fol_hlink(writer);

        write_end_tag(writer, "a:clrScheme");
    }
}
