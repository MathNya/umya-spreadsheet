// c:strLit
use super::StringPoint;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct StringLiteral {
    string_point_list: ThinVec<StringPoint>,
}

impl StringLiteral {
    pub fn get_string_point_list(&self) -> &[StringPoint] {
        &self.string_point_list
    }

    pub fn get_string_point_list_mut(&mut self) -> &mut ThinVec<StringPoint> {
        &mut self.string_point_list
    }

    pub fn add_string_point_list(&mut self, value: StringPoint) -> &mut Self {
        self.string_point_list.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().0 == b"c:pt" {
                    let mut obj = StringPoint::default();
                    obj.set_attributes(reader, e);
                    self.add_string_point_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:strLit" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:strLit")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:strLit
        write_start_tag(writer, "c:strLit", vec![], false);

        // c:ptCount
        let count = self.string_point_list.len().to_string();
        write_start_tag(writer, "c:ptCount", vec![("val", &count)], true);

        // c:pt
        for (index, obj) in self.string_point_list.iter().enumerate() {
            obj.write_to(writer, &(index as u32));
        }

        write_end_tag(writer, "c:strLit");
    }
}
