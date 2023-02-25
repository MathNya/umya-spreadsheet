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

    pub fn get_rotation(&self) -> &Option<Rotation> {
        &self.rotation
    }

    pub fn get_rotation_mut(&mut self) -> &mut Option<Rotation> {
        &mut self.rotation
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
        match get_attribute(e, b"rig") {
            Some(v) => {
                self.rig.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"dir") {
            Some(v) => {
                self.definition.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:rot" => {
                        let mut obj = Rotation::default();
                        obj.set_attributes(reader, e);
                        self.rotation = Some(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:lightRig" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:lightRig"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
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
            match &self.rotation {
                Some(v) => {
                    v.write_to(writer);
                }
                _ => {}
            }
            write_end_tag(writer, "a:lightRig");
        }
    }
}
