// a:clrScheme
use super::super::StringValue;
use super::Color2Type;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ColorScheme {
    name: StringValue,
    accent1: Color2Type,
    accent2: Color2Type,
    accent3: Color2Type,
    accent4: Color2Type,
    accent5: Color2Type,
    accent6: Color2Type,
    dk1: Color2Type,
    dk2: Color2Type,
    fol_hlink: Color2Type,
    hlink: Color2Type,
    lt1: Color2Type,
    lt2: Color2Type,
}
impl ColorScheme {
    pub fn get_name(&self) -> &str {
        self.name.get_value()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn set_accent1(&mut self, value: Color2Type) {
        self.accent1 = value;
    }

    pub fn get_accent1(&self) -> &Color2Type {
        &self.accent1
    }

    pub fn get_accent1_mut(&mut self) -> &mut Color2Type {
        &mut self.accent1
    }

    pub fn set_accent2(&mut self, value: Color2Type) {
        self.accent2 = value;
    }

    pub fn get_accent2(&self) -> &Color2Type {
        &self.accent2
    }

    pub fn get_accent2_mut(&mut self) -> &mut Color2Type {
        &mut self.accent2
    }

    pub fn set_accent3(&mut self, value: Color2Type) {
        self.accent3 = value;
    }

    pub fn get_accent3(&self) -> &Color2Type {
        &self.accent3
    }

    pub fn get_accent3_mut(&mut self) -> &mut Color2Type {
        &mut self.accent3
    }

    pub fn set_accent4(&mut self, value: Color2Type) {
        self.accent4 = value;
    }

    pub fn get_accent4(&self) -> &Color2Type {
        &self.accent4
    }

    pub fn get_accent4_mut(&mut self) -> &mut Color2Type {
        &mut self.accent4
    }

    pub fn set_accent5(&mut self, value: Color2Type) {
        self.accent5 = value;
    }

    pub fn get_accent5(&self) -> &Color2Type {
        &self.accent5
    }

    pub fn get_accent5_mut(&mut self) -> &mut Color2Type {
        &mut self.accent5
    }

    pub fn set_accent6(&mut self, value: Color2Type) {
        self.accent6 = value;
    }

    pub fn get_accent6(&self) -> &Color2Type {
        &self.accent6
    }

    pub fn get_accent6_mut(&mut self) -> &mut Color2Type {
        &mut self.accent6
    }

    pub fn set_dk1(&mut self, value: Color2Type) {
        self.dk1 = value;
    }

    pub fn get_dk1(&self) -> &Color2Type {
        &self.dk1
    }

    pub fn get_dk1_mut(&mut self) -> &mut Color2Type {
        &mut self.dk1
    }

    pub fn set_dk2(&mut self, value: Color2Type) {
        self.dk2 = value;
    }

    pub fn get_dk2(&self) -> &Color2Type {
        &self.dk2
    }

    pub fn get_dk2_mut(&mut self) -> &mut Color2Type {
        &mut self.dk2
    }

    pub fn set_fol_hlink(&mut self, value: Color2Type) {
        self.fol_hlink = value;
    }

    pub fn get_fol_hlink(&self) -> &Color2Type {
        &self.fol_hlink
    }

    pub fn get_fol_hlink_mut(&mut self) -> &mut Color2Type {
        &mut self.fol_hlink
    }

    pub fn set_hlink(&mut self, value: Color2Type) {
        self.hlink = value;
    }

    pub fn get_hlink(&self) -> &Color2Type {
        &self.hlink
    }

    pub fn get_hlink_mut(&mut self) -> &mut Color2Type {
        &mut self.hlink
    }

    pub fn set_lt1(&mut self, value: Color2Type) {
        self.lt1 = value;
    }

    pub fn get_lt1(&self) -> &Color2Type {
        &self.lt1
    }

    pub fn get_lt1_mut(&mut self) -> &mut Color2Type {
        &mut self.lt1
    }

    pub fn set_lt2(&mut self, value: Color2Type) {
        self.lt2 = value;
    }

    pub fn get_lt2(&self) -> &Color2Type {
        &self.lt2
    }

    pub fn get_lt2_mut(&mut self) -> &mut Color2Type {
        &mut self.lt2
    }

    pub fn get_color_map(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(self.dk1.get_val());
        result.push(self.lt1.get_val());
        result.push(self.dk2.get_val());
        result.push(self.lt2.get_val());
        result.push(self.accent1.get_val());
        result.push(self.accent2.get_val());
        result.push(self.accent3.get_val());
        result.push(self.accent4.get_val());
        result.push(self.accent5.get_val());
        result.push(self.accent6.get_val());
        result.push(self.hlink.get_val());
        result.push(self.fol_hlink.get_val());
        result
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"name") {
            Some(v) => {
                self.name.set_value(v);
            }
            _ => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:clrScheme" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:clrScheme"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:clrScheme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_string()));
        }
        write_start_tag(writer, "a:clrScheme", attributes, false);

        // a:dk1
        let _ = &self.dk1.write_to_dk1(writer);

        // a:lt1
        let _ = &self.lt1.write_to_lt1(writer);

        // a:dk2
        let _ = &self.dk2.write_to_dk2(writer);

        // a:lt2
        let _ = &self.lt2.write_to_lt2(writer);

        // a:accent1
        let _ = &self.accent1.write_to_accent1(writer);

        // a:accent2
        let _ = &self.accent2.write_to_accent2(writer);

        // a:accent3
        let _ = &self.accent3.write_to_accent3(writer);

        // a:accent4
        let _ = &self.accent4.write_to_accent4(writer);

        // a:accent5
        let _ = &self.accent5.write_to_accent5(writer);

        // a:accent6
        let _ = &self.accent6.write_to_accent6(writer);

        // a:hlink
        let _ = &self.hlink.write_to_hlink(writer);

        // a:folHlink
        let _ = &self.fol_hlink.write_to_fol_hlink(writer);

        write_end_tag(writer, "a:clrScheme");
    }
}
