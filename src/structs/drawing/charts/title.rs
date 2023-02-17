// c:title
use super::ChartText;
use super::Layout;
use super::Overlay;
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
    pub fn get_chart_text(&self) -> &Option<ChartText> {
        &self.chart_text
    }

    pub fn get_chart_text_mut(&mut self) -> &mut Option<ChartText> {
        &mut self.chart_text
    }

    pub fn set_chart_text(&mut self, value: ChartText) -> &mut Title {
        self.chart_text = Some(value);
        self
    }

    pub fn get_layout(&self) -> &Option<Layout> {
        &self.layout
    }

    pub fn get_layout_mut(&mut self) -> &mut Option<Layout> {
        &mut self.layout
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
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
                Ok(Event::Empty(ref e)) => match e.name().0 {
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
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:title" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:title"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:title
        write_start_tag(writer, "c:title", vec![], false);

        // c:tx
        match &self.chart_text {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:layout
        match &self.layout {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:overlay
        self.overlay.write_to(writer);

        write_end_tag(writer, "c:title");
    }
}
