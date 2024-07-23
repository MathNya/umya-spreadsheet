// c:title
use super::ChartText;
use super::Layout;
use super::Overlay;
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Title {
    chart_text: Option<ChartText>,
    layout: Option<Layout>,
    overlay: Overlay,
}

impl Title {
    pub fn get_chart_text(&self) -> Option<&ChartText> {
        self.chart_text.as_ref()
    }

    pub fn get_chart_text_mut(&mut self) -> Option<&mut ChartText> {
        self.chart_text.as_mut()
    }

    pub fn set_chart_text(&mut self, value: ChartText) -> &mut Title {
        self.chart_text = Some(value);
        self
    }

    pub fn get_layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }

    pub fn get_layout_mut(&mut self) -> Option<&mut Layout> {
        self.layout.as_mut()
    }

    pub fn set_layout(&mut self, value: Layout) -> &mut Title {
        self.layout = Some(value);
        self
    }

    pub fn get_overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn get_overlay_mut(&mut self) -> &mut Overlay {
        &mut self.overlay
    }

    pub fn set_overlay(&mut self, value: Overlay) -> &mut Title {
        self.overlay = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().0 {
                b"c:tx" => {
                    let mut obj = ChartText::default();
                    obj.set_attributes(reader, e);
                    self.set_chart_text(obj);
                }
                b"c:layout" => {
                    let mut obj = Layout::default();
                    obj.set_attributes(reader, e, false);
                    self.set_layout(obj);
                }
                _ => (),
            },
            Event::Empty(ref e) => match e.name().0 {
                b"c:overlay" => {
                    self.overlay.set_attributes(reader, e);
                }
                b"c:layout" => {
                    let mut obj = Layout::default();
                    obj.set_attributes(reader, e, true);
                    self.set_layout(obj);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:title" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:title"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:title
        write_start_tag(writer, "c:title", vec![], false);

        // c:tx
        if let Some(v) = &self.chart_text {
            v.write_to(writer);
        }

        // c:layout
        if let Some(v) = &self.layout {
            v.write_to(writer);
        }

        // c:overlay
        self.overlay.write_to(writer);

        write_end_tag(writer, "c:title");
    }
}
