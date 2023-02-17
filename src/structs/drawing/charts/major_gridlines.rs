// c:majorGridlines
use super::ShapeProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct MajorGridlines {
    shape_properties: Option<ShapeProperties>,
}
impl MajorGridlines {
    pub fn get_shape_properties(&self) -> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut Option<ShapeProperties> {
        &mut self.shape_properties
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

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"c:spPr" => {
                        let mut obj = ShapeProperties::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:majorGridlines" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:majorGridlines"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    fn with_include(&self) -> bool {
        if self.shape_properties.is_some() {
            return true;
        }
        false
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.with_include() {
            // c:majorGridlines
            write_start_tag(writer, "c:majorGridlines", vec![], false);

            // c:spPr
            match &self.shape_properties {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "c:majorGridlines");
        } else {
            // c:majorGridlines
            write_start_tag(writer, "c:majorGridlines", vec![], true);
        }
    }
}
