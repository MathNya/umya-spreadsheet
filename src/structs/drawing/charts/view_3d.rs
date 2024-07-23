// c:view3D
use super::Perspective;
use super::RightAngleAxes;
use super::RotateX;
use super::RotateY;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
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
    pub fn get_rotate_x(&self) -> Option<&RotateX> {
        self.rotate_x.as_ref()
    }

    pub fn get_rotate_x_mut(&mut self) -> Option<&mut RotateX> {
        self.rotate_x.as_mut()
    }

    pub fn set_rotate_x(&mut self, value: RotateX) -> &mut View3D {
        self.rotate_x = Some(value);
        self
    }

    pub fn get_rotate_y(&self) -> Option<&RotateY> {
        self.rotate_y.as_ref()
    }

    pub fn get_rotate_y_mut(&mut self) -> Option<&mut RotateY> {
        self.rotate_y.as_mut()
    }

    pub fn set_rotate_y(&mut self, value: RotateY) -> &mut View3D {
        self.rotate_y = Some(value);
        self
    }

    pub fn get_right_angle_axes(&self) -> Option<&RightAngleAxes> {
        self.right_angle_axes.as_ref()
    }

    pub fn get_right_angle_axes_mut(&mut self) -> Option<&mut RightAngleAxes> {
        self.right_angle_axes.as_mut()
    }

    pub fn set_right_angle_axes(&mut self, value: RightAngleAxes) -> &mut View3D {
        self.right_angle_axes = Some(value);
        self
    }

    pub fn get_perspective(&self) -> Option<&Perspective> {
        self.perspective.as_ref()
    }

    pub fn get_perspective_mut(&mut self) -> Option<&mut Perspective> {
        self.perspective.as_mut()
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
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().0 {
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
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:view3D" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:view3D")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:view3D
        write_start_tag(writer, "c:view3D", vec![], false);

        // c:rotX
        if let Some(v) = &self.rotate_x {
            v.write_to(writer);
        }

        // c:rotY
        if let Some(v) = &self.rotate_y {
            v.write_to(writer);
        }

        // c:rAngAx
        if let Some(v) = &self.right_angle_axes {
            v.write_to(writer);
        }

        // c:perspective
        if let Some(v) = &self.perspective {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:view3D");
    }
}
