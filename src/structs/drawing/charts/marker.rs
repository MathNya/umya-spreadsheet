// c:marker
use super::ShapeProperties;
use super::Size;
use super::Symbol;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Marker {
    symbol: Option<Symbol>,
    size: Option<Size>,
    shape_properties: Option<ShapeProperties>,
}

impl Marker {
    pub fn get_symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }

    pub fn get_symbol_mut(&mut self) -> Option<&mut Symbol> {
        self.symbol.as_mut()
    }

    pub fn set_symbol(&mut self, value: Symbol) -> &mut Marker {
        self.symbol = Some(value);
        self
    }

    pub fn get_size(&self) -> Option<&Size> {
        self.size.as_ref()
    }

    pub fn get_size_mut(&mut self) -> Option<&mut Size> {
        self.size.as_mut()
    }

    pub fn set_size(&mut self, value: Size) -> &mut Marker {
        self.size = Some(value);
        self
    }

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
        _: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().0 == b"c:symbol" {
                    let mut obj = Symbol::default();
                    obj.set_attributes(reader, e);
                    self.set_symbol(obj);
                }
                if e.name().0 == b"c:size" {
                    let mut obj = Size::default();
                    obj.set_attributes(reader, e);
                    self.set_size(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().0 == b"c:spPr" {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:marker" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:marker")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:marker
        if self.symbol.is_some() {
            write_start_tag(writer, "c:marker", vec![], false);

            // a:symbol
            if let Some(v) = &self.symbol {
                v.write_to(writer);
            }

            // c:size
            if let Some(v) = &self.size {
                v.write_to(writer);
            }

            // c:spPr
            if let Some(v) = &self.shape_properties {
                v.write_to(writer);
            }

            write_end_tag(writer, "c:marker");
        } else {
            write_start_tag(writer, "c:marker", vec![], true);
        }
    }
}
