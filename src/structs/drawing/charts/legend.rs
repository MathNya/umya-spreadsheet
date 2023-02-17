use super::Layout;
use super::LegendPosition;
use super::Overlay;
use super::ShapeProperties;
use super::TextProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Legend {
    legend_position: LegendPosition,
    layout: Option<Layout>,
    overlay: Overlay,
    shape_properties: Option<ShapeProperties>,
    text_properties: Option<TextProperties>,
}
impl Legend {
    pub fn get_legend_position(&self) -> &LegendPosition {
        &self.legend_position
    }

    pub fn get_legend_position_mut(&mut self) -> &mut LegendPosition {
        &mut self.legend_position
    }

    pub fn set_legend_position(&mut self, value: LegendPosition) -> &mut Self {
        self.legend_position = value;
        self
    }

    pub fn get_layout(&self) -> &Option<Layout> {
        &self.layout
    }

    pub fn get_layout_mut(&mut self) -> &mut Option<Layout> {
        &mut self.layout
    }

    pub fn set_layout(&mut self, value: Layout) -> &mut Self {
        self.layout = Some(value);
        self
    }

    pub fn get_overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn get_overlay_mut(&mut self) -> &mut Overlay {
        &mut self.overlay
    }

    pub fn set_overlay(&mut self, value: Overlay) -> &mut Self {
        self.overlay = value;
        self
    }

    pub fn get_shape_properties(&self) -> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut Option<ShapeProperties> {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    pub fn get_text_properties(&self) -> &Option<TextProperties> {
        &self.text_properties
    }

    pub fn get_text_properties_mut(&mut self) -> &mut Option<TextProperties> {
        &mut self.text_properties
    }

    pub fn set_text_properties(&mut self, value: TextProperties) -> &mut Self {
        self.text_properties = Some(value);
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
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"c:layout" => {
                        let mut obj = Layout::default();
                        obj.set_attributes(reader, e, false);
                        self.set_layout(obj);
                    }
                    b"c:spPr" => {
                        let mut obj = ShapeProperties::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_properties(obj);
                    }
                    b"c:txPr" => {
                        let mut obj = TextProperties::default();
                        obj.set_attributes(reader, e);
                        self.set_text_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:legendPos" => {
                        self.legend_position.set_attributes(reader, e);
                    }
                    b"c:layout" => {
                        let mut obj = Layout::default();
                        obj.set_attributes(reader, e, true);
                        self.set_layout(obj);
                    }
                    b"c:overlay" => {
                        self.overlay.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:legend" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:legend"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:legend
        write_start_tag(writer, "c:legend", vec![], false);

        // c:legendPos
        self.legend_position.write_to(writer);

        // c:layout
        match &self.layout {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:overlay
        self.overlay.write_to(writer);

        // c:spPr
        match &self.shape_properties {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:txPr
        match &self.text_properties {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:legend");
    }
}
