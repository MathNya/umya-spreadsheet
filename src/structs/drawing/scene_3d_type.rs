// a:scene3d
use super::Camera;
use super::LightRig;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Scene3DType {
    camera: Option<Camera>,
    light_rig: Option<LightRig>,
}

impl Scene3DType {
    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }

    pub fn set_camera(&mut self, value: Camera) -> &mut Scene3DType {
        self.camera = Some(value);
        self
    }

    pub fn get_light_rig(&self) -> Option<&LightRig> {
        self.light_rig.as_ref()
    }

    pub fn set_light_rig(&mut self, value: LightRig) -> &mut Scene3DType {
        self.light_rig = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:camera" => {
                        let mut obj = Camera::default();
                        obj.set_attributes(reader, e, true);
                        self.set_camera(obj);
                    }
                    b"a:lightRig" => {
                        let mut obj = LightRig::default();
                        obj.set_attributes(reader, e, true);
                        self.set_light_rig(obj);
                    }
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:camera" => {
                        let mut obj = Camera::default();
                        obj.set_attributes(reader, e, false);
                        self.set_camera(obj);
                    }
                    b"a:lightRig" => {
                        let mut obj = LightRig::default();
                        obj.set_attributes(reader, e, false);
                        self.set_light_rig(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:scene3d" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:scene3d")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:scene3d
        write_start_tag(writer, "a:scene3d", vec![], false);

        // a:camera
        if let Some(v) = &self.camera {
            v.write_to(writer);
        }

        // a:lightRig
        if let Some(v) = &self.light_rig {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:scene3d");
    }
}
