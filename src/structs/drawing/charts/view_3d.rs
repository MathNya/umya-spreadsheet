// c:view3D
use super::Perspective;
use super::RightAngleAxes;
use super::RotateX;
use super::RotateY;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct View3D {
    rotate_x: Option<RotateX>,
    rotate_y: Option<RotateY>,
    right_angle_axes: Option<RightAngleAxes>,
    perspective: Option<Perspective>,
}
impl View3D {
    pub fn get_rotate_x(&self) -> &Option<RotateX> {
        &self.rotate_x
    }

    pub fn get_rotate_x_mut(&mut self) -> &mut Option<RotateX> {
        &mut self.rotate_x
    }

    pub fn set_rotate_x(&mut self, value: RotateX) -> &mut View3D {
        self.rotate_x = Some(value);
        self
    }

    pub fn get_rotate_y(&self) -> &Option<RotateY> {
        &self.rotate_y
    }

    pub fn get_rotate_y_mut(&mut self) -> &mut Option<RotateY> {
        &mut self.rotate_y
    }

    pub fn set_rotate_y(&mut self, value: RotateY) -> &mut View3D {
        self.rotate_y = Some(value);
        self
    }

    pub fn get_right_angle_axes(&self) -> &Option<RightAngleAxes> {
        &self.right_angle_axes
    }

    pub fn get_right_angle_axes_mut(&mut self) -> &mut Option<RightAngleAxes> {
        &mut self.right_angle_axes
    }

    pub fn set_right_angle_axes(&mut self, value: RightAngleAxes) -> &mut View3D {
        self.right_angle_axes = Some(value);
        self
    }

    pub fn get_perspective(&self) -> &Option<Perspective> {
        &self.perspective
    }

    pub fn get_perspective_mut(&mut self) -> &mut Option<Perspective> {
        &mut self.perspective
    }

    pub fn set_perspective(&mut self, value: Perspective) -> &mut View3D {
        self.perspective = Some(value);
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
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"c:rotX" => {
                        let mut obj = RotateX::default();
                        obj.set_attributes(reader, e);
                        self.set_rotate_x(obj);
                    }
                    b"c:rotY" => {
                        let mut obj = RotateY::default();
                        obj.set_attributes(reader, e);
                        self.set_rotate_y(obj);
                    }
                    b"c:rAngAx" => {
                        let mut obj = RightAngleAxes::default();
                        obj.set_attributes(reader, e);
                        self.set_right_angle_axes(obj);
                    }
                    b"c:perspective" => {
                        let mut obj = Perspective::default();
                        obj.set_attributes(reader, e);
                        self.set_perspective(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:view3D" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:view3D"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:view3D
        write_start_tag(writer, "c:view3D", vec![], false);

        // c:rotX
        match &self.rotate_x {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:rotY
        match &self.rotate_y {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:rAngAx
        match &self.right_angle_axes {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:perspective
        match &self.perspective {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:view3D");
    }
}
