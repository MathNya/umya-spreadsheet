// a:lightRig
use super::super::EnumValue;
use super::LightRigDirectionValues;
use super::LightRigValues;
use super::Rotation;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LightRig {
    rig: EnumValue<LightRigValues>,
    definition: EnumValue<LightRigDirectionValues>,
    rotation: Option<Rotation>,
}

impl LightRig {
    pub fn get_rig(&self) -> &LightRigValues {
        self.rig.get_value()
    }

    pub fn set_rig(&mut self, value: LightRigValues) -> &mut LightRig {
        self.rig.set_value(value);
        self
    }

    pub fn get_definition(&self) -> &LightRigDirectionValues {
        self.definition.get_value()
    }

    pub fn set_definition(&mut self, value: LightRigDirectionValues) -> &mut LightRig {
        self.definition.set_value(value);
        self
    }

    pub fn get_rotation(&self) -> Option<&Rotation> {
        self.rotation.as_ref()
    }

    pub fn get_rotation_mut(&mut self) -> Option<&mut Rotation> {
        self.rotation.as_mut()
    }

    pub fn set_rotation(&mut self, value: Rotation) -> &mut Self {
        self.rotation = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, rig, "rig");
        set_string_from_xml!(self, e, definition, "dir");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:rot" {
                    let mut obj = Rotation::default();
                    obj.set_attributes(reader, e);
                    self.rotation = Some(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lightRig" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lightRig")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let with_inner = self.rotation.is_some();
        // a:lightRig
        write_start_tag(
            writer,
            "a:lightRig",
            vec![
                ("rig", self.rig.get_value_string()),
                ("dir", self.definition.get_value_string()),
            ],
            !with_inner,
        );

        if with_inner {
            // a:rot
            if let Some(v) = &self.rotation {
                v.write_to(writer);
            }
            write_end_tag(writer, "a:lightRig");
        }
    }
}
