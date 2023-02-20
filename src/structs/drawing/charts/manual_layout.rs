// c:manualLayout
use super::Height;
use super::HeightMode;
use super::LayoutTarget;
use super::Left;
use super::LeftMode;
use super::Top;
use super::TopMode;
use super::Width;
use super::WidthMode;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ManualLayout {
    height: Option<Height>,
    height_mode: Option<HeightMode>,
    layout_target: Option<LayoutTarget>,
    left: Option<Left>,
    left_mode: Option<LeftMode>,
    top: Option<Top>,
    top_mode: Option<TopMode>,
    width: Option<Width>,
    width_mode: Option<WidthMode>,
}
impl ManualLayout {
    pub fn get_height(&self) -> &Option<Height> {
        &self.height
    }

    pub fn get_height_mut(&mut self) -> &mut Option<Height> {
        &mut self.height
    }

    pub fn set_height(&mut self, value: Height) -> &mut ManualLayout {
        self.height = Some(value);
        self
    }

    pub fn get_height_mode(&self) -> &Option<HeightMode> {
        &self.height_mode
    }

    pub fn get_height_mode_mut(&mut self) -> &mut Option<HeightMode> {
        &mut self.height_mode
    }

    pub fn set_height_mode(&mut self, value: HeightMode) -> &mut ManualLayout {
        self.height_mode = Some(value);
        self
    }

    pub fn get_layout_target(&self) -> &Option<LayoutTarget> {
        &self.layout_target
    }

    pub fn get_layout_target_mut(&mut self) -> &mut Option<LayoutTarget> {
        &mut self.layout_target
    }

    pub fn set_layout_target(&mut self, value: LayoutTarget) -> &mut ManualLayout {
        self.layout_target = Some(value);
        self
    }

    pub fn get_left(&self) -> &Option<Left> {
        &self.left
    }

    pub fn get_left_mut(&mut self) -> &mut Option<Left> {
        &mut self.left
    }

    pub fn set_left(&mut self, value: Left) -> &mut ManualLayout {
        self.left = Some(value);
        self
    }

    pub fn get_left_mode(&self) -> &Option<LeftMode> {
        &self.left_mode
    }

    pub fn get_left_mode_mut(&mut self) -> &mut Option<LeftMode> {
        &mut self.left_mode
    }

    pub fn set_left_mode(&mut self, value: LeftMode) -> &mut ManualLayout {
        self.left_mode = Some(value);
        self
    }

    pub fn get_top(&self) -> &Option<Top> {
        &self.top
    }

    pub fn get_top_mut(&mut self) -> &mut Option<Top> {
        &mut self.top
    }

    pub fn set_top(&mut self, value: Top) -> &mut ManualLayout {
        self.top = Some(value);
        self
    }

    pub fn get_top_mode(&self) -> &Option<TopMode> {
        &self.top_mode
    }

    pub fn get_top_mode_mut(&mut self) -> &mut Option<TopMode> {
        &mut self.top_mode
    }

    pub fn set_top_mode(&mut self, value: TopMode) -> &mut ManualLayout {
        self.top_mode = Some(value);
        self
    }

    pub fn get_width(&self) -> &Option<Width> {
        &self.width
    }

    pub fn get_width_mut(&mut self) -> &mut Option<Width> {
        &mut self.width
    }

    pub fn set_width(&mut self, value: Width) -> &mut ManualLayout {
        self.width = Some(value);
        self
    }

    pub fn get_width_mode(&self) -> &Option<WidthMode> {
        &self.width_mode
    }

    pub fn get_width_mode_mut(&mut self) -> &mut Option<WidthMode> {
        &mut self.width_mode
    }

    pub fn set_width_mode(&mut self, value: WidthMode) -> &mut ManualLayout {
        self.width_mode = Some(value);
        self
    }

    pub fn is_empty(&self) -> bool {
        if self.height.is_none()
            && self.height_mode.is_none()
            && self.layout_target.is_none()
            && self.left.is_none()
            && self.left_mode.is_none()
            && self.top.is_none()
            && self.top_mode.is_none()
            && self.width.is_none()
            && self.width_mode.is_none()
        {
            return true;
        }

        false
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
                    b"c:h" => {
                        let mut obj = Height::default();
                        obj.set_attributes(reader, e);
                        self.set_height(obj);
                    }
                    b"c:hMode" => {
                        let mut obj = HeightMode::default();
                        obj.set_attributes(reader, e);
                        self.set_height_mode(obj);
                    }
                    b"c:layoutTarget" => {
                        let mut obj = LayoutTarget::default();
                        obj.set_attributes(reader, e);
                        self.set_layout_target(obj);
                    }
                    b"c:x" => {
                        let mut obj = Left::default();
                        obj.set_attributes(reader, e);
                        self.set_left(obj);
                    }
                    b"c:xMode" => {
                        let mut obj = LeftMode::default();
                        obj.set_attributes(reader, e);
                        self.set_left_mode(obj);
                    }
                    b"c:y" => {
                        let mut obj = Top::default();
                        obj.set_attributes(reader, e);
                        self.set_top(obj);
                    }
                    b"c:yMode" => {
                        let mut obj = TopMode::default();
                        obj.set_attributes(reader, e);
                        self.set_top_mode(obj);
                    }
                    b"c:w" => {
                        let mut obj = Width::default();
                        obj.set_attributes(reader, e);
                        self.set_width(obj);
                    }
                    b"c:wMode" => {
                        let mut obj = WidthMode::default();
                        obj.set_attributes(reader, e);
                        self.set_width_mode(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:manualLayout" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:manualLayout"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:manualLayout
        write_start_tag(writer, "c:manualLayout", vec![], false);

        // c:hMode
        match &self.height_mode {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:xMode
        match &self.left_mode {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:yMode
        match &self.top_mode {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:wMode
        match &self.width_mode {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:h
        match &self.height {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:x
        match &self.left {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:y
        match &self.top {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:w
        match &self.width {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:layoutTarget
        match &self.layout_target {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:manualLayout");
    }
}
