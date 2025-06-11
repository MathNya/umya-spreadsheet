// c:title
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
    ChartText,
    Layout,
    Overlay,
};
use crate::{
    Workbook,
    drawing::charts::ShapeProperties,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct Title {
    chart_text:       Option<ChartText>,
    layout:           Option<Layout>,
    overlay:          Overlay,
    shape_properties: Option<ShapeProperties>,
}

impl Title {
    #[must_use]
    pub fn chart_text(&self) -> Option<&ChartText> {
        self.chart_text.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use chart_text()")]
    pub fn get_chart_text(&self) -> Option<&ChartText> {
        self.chart_text()
    }

    pub fn chart_text_mut(&mut self) -> Option<&mut ChartText> {
        self.chart_text.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use chart_text_mut()")]
    pub fn get_chart_text_mut(&mut self) -> Option<&mut ChartText> {
        self.chart_text_mut()
    }

    pub fn set_chart_text(&mut self, value: ChartText) -> &mut Title {
        self.chart_text = Some(value);
        self
    }

    #[must_use]
    pub fn layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use layout()")]
    pub fn get_layout(&self) -> Option<&Layout> {
        self.layout()
    }

    pub fn layout_mut(&mut self) -> Option<&mut Layout> {
        self.layout.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use layout_mut()")]
    pub fn get_layout_mut(&mut self) -> Option<&mut Layout> {
        self.layout_mut()
    }

    pub fn set_layout(&mut self, value: Layout) -> &mut Title {
        self.layout = Some(value);
        self
    }

    #[must_use]
    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use overlay()")]
    pub fn get_overlay(&self) -> &Overlay {
        self.overlay()
    }

    pub fn overlay_mut(&mut self) -> &mut Overlay {
        &mut self.overlay
    }

    #[deprecated(since = "3.0.0", note = "Use overlay_mut()")]
    pub fn get_overlay_mut(&mut self) -> &mut Overlay {
        self.overlay_mut()
    }

    pub fn set_overlay(&mut self, value: Overlay) -> &mut Title {
        self.overlay = value;
        self
    }

    #[must_use]
    pub fn shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape_properties()")]
    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties()
    }

    pub fn shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use shape_properties_mut()")]
    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
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
                b"c:spPr" => {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
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

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:title
        write_start_tag(writer, "c:title", vec![], false);

        // c:tx
        if let Some(v) = &self.chart_text {
            v.write_to(writer, wb);
        }

        // c:layout
        if let Some(v) = &self.layout {
            v.write_to(writer);
        }

        // c:overlay
        self.overlay.write_to(writer);

        // c:spPr
        if let Some(v) = &self.shape_properties {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:title");
    }
}
