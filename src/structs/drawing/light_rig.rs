// a:lightRig
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
    super::EnumValue,
    LightRigDirectionValues,
    LightRigValues,
    Rotation,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct LightRig {
    rig:        EnumValue<LightRigValues>,
    definition: EnumValue<LightRigDirectionValues>,
    rotation:   Option<Box<Rotation>>,
}

impl LightRig {
    #[inline]
    #[must_use]
    pub fn rig(&self) -> &LightRigValues {
        self.rig.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rig()")]
    pub fn get_rig(&self) -> &LightRigValues {
        self.rig()
    }

    #[inline]
    pub fn set_rig(&mut self, value: LightRigValues) -> &mut LightRig {
        self.rig.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn definition(&self) -> &LightRigDirectionValues {
        self.definition.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use definition()")]
    pub fn get_definition(&self) -> &LightRigDirectionValues {
        self.definition()
    }

    #[inline]
    pub fn set_definition(&mut self, value: LightRigDirectionValues) -> &mut LightRig {
        self.definition.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn rotation(&self) -> Option<&Rotation> {
        self.rotation.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rotation()")]
    pub fn get_rotation(&self) -> Option<&Rotation> {
        self.rotation()
    }

    #[inline]
    pub fn rotation_mut(&mut self) -> Option<&mut Rotation> {
        self.rotation.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use rotation_mut()")]
    pub fn get_rotation_mut(&mut self) -> Option<&mut Rotation> {
        self.rotation_mut()
    }

    #[inline]
    pub fn set_rotation(&mut self, value: Rotation) -> &mut Self {
        self.rotation = Some(Box::new(value));
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
                    self.rotation = Some(Box::new(obj));
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
                ("rig", self.rig.value_string()).into(),
                ("dir", self.definition.value_string()).into(),
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
