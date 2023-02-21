// a:scene3d
use super::Camera;
use super::LightRig;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Scene3DType {
    camera: Option<Camera>,
    light_rig: Option<LightRig>,
}
impl Scene3DType {
    pub fn get_camera(&self) -> &Option<Camera> {
        &self.camera
    }

    pub fn set_camera(&mut self, value: Camera) -> &mut Scene3DType {
        self.camera = Some(value);
        self
    }

    pub fn get_light_rig(&self) -> &Option<LightRig> {
        &self.light_rig
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:scene3d" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:scene3d"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:scene3d
        write_start_tag(writer, "a:scene3d", vec![], false);

        // a:camera
        match &self.camera {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:lightRig
        match &self.light_rig {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "a:scene3d");
    }
}
