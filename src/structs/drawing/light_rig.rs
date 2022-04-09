// a:lightRig
use super::super::EnumValue;
use super::LightRigDirectionValues;
use super::LightRigValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LightRig {
    rig: EnumValue<LightRigValues>,
    definition: EnumValue<LightRigDirectionValues>,
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

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
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
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lightRig
        write_start_tag(
            writer,
            "a:lightRig",
            vec![
                ("rig", self.rig.get_value_string()),
                ("dir", self.definition.get_value_string()),
            ],
            true,
        );
    }
}
