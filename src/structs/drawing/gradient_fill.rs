// a:gradFill
use super::super::super::EnumValue;
use super::super::BooleanValue;
use super::GradientStopList;
use super::LinearGradientFill;
use super::TileFlipValues;
use super::TileRectangle;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct GradientFill {
    flip: EnumValue<TileFlipValues>,
    rotate_with_shape: BooleanValue,
    gradient_stop_list: GradientStopList,
    linear_gradient_fill: Option<Box<LinearGradientFill>>,
    tile_rectangle: Option<Box<TileRectangle>>,
}

impl GradientFill {
    #[inline]
    pub fn get_flip(&self) -> &TileFlipValues {
        self.flip.get_value()
    }

    #[inline]
    pub fn set_flip(&mut self, value: TileFlipValues) -> &mut GradientFill {
        self.flip.set_value(value);
        self
    }

    #[inline]
    pub fn get_rotate_with_shape(&self) -> &bool {
        self.rotate_with_shape.get_value()
    }

    #[inline]
    pub fn set_rotate_with_shape(&mut self, value: bool) -> &mut GradientFill {
        self.rotate_with_shape.set_value(value);
        self
    }

    #[inline]
    pub fn get_gradient_stop_list(&self) -> &GradientStopList {
        &self.gradient_stop_list
    }

    #[inline]
    pub fn get_gradient_stop_list_mut(&mut self) -> &mut GradientStopList {
        &mut self.gradient_stop_list
    }

    #[inline]
    pub fn set_gradient_stop_list(&mut self, value: GradientStopList) -> &mut GradientFill {
        self.gradient_stop_list = value;
        self
    }

    #[inline]
    pub fn get_linear_gradient_fill(&self) -> Option<&LinearGradientFill> {
        self.linear_gradient_fill.as_deref()
    }

    #[inline]
    pub fn get_linear_gradient_fill_mut(&mut self) -> Option<&mut LinearGradientFill> {
        self.linear_gradient_fill.as_deref_mut()
    }

    #[inline]
    pub fn set_linear_gradient_fill(&mut self, value: LinearGradientFill) -> &mut GradientFill {
        self.linear_gradient_fill = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn get_tile_rectangle(&self) -> Option<&TileRectangle> {
        self.tile_rectangle.as_deref()
    }

    #[inline]
    pub fn get_tile_rectangle_mut(&mut self) -> Option<&mut TileRectangle> {
        self.tile_rectangle.as_deref_mut()
    }

    #[inline]
    pub fn set_tile_rectangle(&mut self, value: TileRectangle) -> &mut GradientFill {
        self.tile_rectangle = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, flip, "flip");
        set_string_from_xml!(self, e, rotate_with_shape, "rotWithShape");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                b"a:lin" => {
                    let mut obj = LinearGradientFill::default();
                    obj.set_attributes(reader, e, true);
                    self.set_linear_gradient_fill(obj);
                }
                b"a:tileRect" => {
                    let mut obj = TileRectangle::default();
                    obj.set_attributes(reader, e, true);
                    self.set_tile_rectangle(obj);
                }
                _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:lin" => {
                    let mut obj = LinearGradientFill::default();
                    obj.set_attributes(reader, e, false);
                    self.set_linear_gradient_fill(obj);
                }
                b"a:tileRect" => {
                    let mut obj = TileRectangle::default();
                    obj.set_attributes(reader, e, false);
                    self.set_tile_rectangle(obj);
                }
                b"a:gsLst" => {
                    self.gradient_stop_list.set_attributes(reader, e);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:gradFill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:gradFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:gradFill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.flip.has_value() {
            attributes.push(("flip", self.flip.get_value_string()));
        }
        if self.rotate_with_shape.has_value() {
            attributes.push(("rotWithShape", self.rotate_with_shape.get_value_string()));
        }
        write_start_tag(writer, "a:gradFill", attributes, false);

        // a:gsLst
        self.gradient_stop_list.write_to(writer);

        // a:lin
        if let Some(v) = &self.linear_gradient_fill {
            v.write_to(writer);
        }

        // a:tileRect
        if let Some(v) = &self.tile_rectangle {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:gradFill");
    }
}
