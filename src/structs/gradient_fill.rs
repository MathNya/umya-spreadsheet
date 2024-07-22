// gradientFill
use super::DoubleValue;
use super::GradientStop;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::fmt::Write;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GradientFill {
    degree: DoubleValue,
    gradient_stop: Vec<GradientStop>,
}

impl GradientFill {
    pub fn get_degree(&self) -> &f64 {
        self.degree.get_value()
    }

    pub fn set_degree(&mut self, value: f64) -> &mut Self {
        self.degree.set_value(value);
        self
    }

    pub fn get_gradient_stop(&self) -> &Vec<GradientStop> {
        &self.gradient_stop
    }

    pub fn get_gradient_stop_mut(&mut self) -> &mut Vec<GradientStop> {
        &mut self.gradient_stop
    }

    pub fn set_gradient_stop(&mut self, value: GradientStop) -> &mut Self {
        self.gradient_stop.push(value);
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        let mut value = String::from("");
        for stop in &self.gradient_stop {
            write!(value, "{}", stop.get_hash_code().as_str()).unwrap();
        }
        format!(
            "{:x}",
            md5::Md5::digest(format!("{}{}", &self.degree.get_value_string(), value,))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, degree, "degree");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"stop" {
                    let mut obj = GradientStop::default();
                    obj.set_attributes(reader, e);
                    self.set_gradient_stop(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"gradientFill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "gradientFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // gradientFill
        write_start_tag(
            writer,
            "gradientFill",
            vec![("degree", &self.degree.get_value_string())],
            false,
        );

        // stop
        for stop in &self.gradient_stop {
            stop.write_to(writer);
        }

        write_end_tag(writer, "gradientFill");
    }
}
