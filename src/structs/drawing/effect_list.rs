// a:effectLst
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
    Glow,
    OuterShadow,
    SoftEdge,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct EffectList {
    glow:         Option<Box<Glow>>,
    outer_shadow: Option<Box<OuterShadow>>,
    soft_edge:    Option<Box<SoftEdge>>,
}

impl EffectList {
    #[must_use]
    pub fn get_glow(&self) -> Option<&Glow> {
        self.glow.as_deref()
    }

    pub fn get_glow_mut(&mut self) -> Option<&mut Glow> {
        self.glow.as_deref_mut()
    }

    pub fn set_glow(&mut self, value: Glow) {
        self.glow = Some(Box::new(value));
    }

    #[must_use]
    pub fn get_outer_shadow(&self) -> Option<&OuterShadow> {
        self.outer_shadow.as_deref()
    }

    pub fn get_outer_shadow_mut(&mut self) -> Option<&mut OuterShadow> {
        self.outer_shadow.as_deref_mut()
    }

    pub fn set_outer_shadow(&mut self, value: OuterShadow) {
        self.outer_shadow = Some(Box::new(value));
    }

    #[must_use]
    pub fn get_soft_edge(&self) -> Option<&SoftEdge> {
        self.soft_edge.as_deref()
    }

    pub fn get_soft_edge_mut(&mut self) -> Option<&mut SoftEdge> {
        self.soft_edge.as_deref_mut()
    }

    pub fn set_soft_edge(&mut self, value: SoftEdge) {
        self.soft_edge = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:softEdge" {
                    let mut obj = SoftEdge::default();
                    obj.set_attributes(reader, e);
                    self.set_soft_edge(obj);
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:glow" => {
                        let mut obj = Glow::default();
                        obj.set_attributes(reader, e);
                        self.set_glow(obj);
                    }
                    b"a:outerShdw" => {
                        let mut obj = OuterShadow::default();
                        obj.set_attributes(reader, e);
                        self.set_outer_shadow(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:effectLst" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:effectLst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag =
            self.glow.is_none() && self.outer_shadow.is_none() && self.soft_edge.is_none();

        // a:effectLst
        write_start_tag(writer, "a:effectLst", vec![], empty_flag);

        if !empty_flag {
            // a:glow
            if let Some(v) = &self.glow {
                v.write_to(writer);
            }

            // a:outerShdow
            if let Some(v) = &self.outer_shadow {
                v.write_to(writer);
            }

            // a:softEdge
            if let Some(v) = &self.soft_edge {
                v.write_to(writer);
            }

            write_end_tag(writer, "a:effectLst");
        }
    }
}
