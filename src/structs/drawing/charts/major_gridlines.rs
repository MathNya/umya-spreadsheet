// c:majorGridlines
use super::ShapeProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct MajorGridlines {
    shape_properties: Option<ShapeProperties>,
}

impl MajorGridlines {
    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_ref()
    }

    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flg: bool,
    ) {
        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"c:spPr" {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:majorGridlines" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:majorGridlines")
        );
    }

    fn with_include(&self) -> bool {
        self.shape_properties.is_some()
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.with_include() {
            // c:majorGridlines
            write_start_tag(writer, "c:majorGridlines", vec![], true);
            return;
        }
        // c:majorGridlines
        write_start_tag(writer, "c:majorGridlines", vec![], false);

        // c:spPr
        if let Some(v) = &self.shape_properties {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:majorGridlines");
    }
}
