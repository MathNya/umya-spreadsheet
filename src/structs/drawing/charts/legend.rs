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
    Layout,
    LegendPosition,
    Overlay,
    ShapeProperties,
    TextProperties,
};
use crate::{
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct Legend {
    legend_position:  LegendPosition,
    layout:           Option<Box<Layout>>,
    overlay:          Overlay,
    shape_properties: Option<Box<ShapeProperties>>,
    text_properties:  Option<Box<TextProperties>>,
}

impl Legend {
    #[must_use]
    pub fn legend_position(&self) -> &LegendPosition {
        &self.legend_position
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use legend_position()")]
    pub fn get_legend_position(&self) -> &LegendPosition {
        self.legend_position()
    }

    pub fn legend_position_mut(&mut self) -> &mut LegendPosition {
        &mut self.legend_position
    }

    #[deprecated(since = "3.0.0", note = "Use legend_position_mut()")]
    pub fn get_legend_position_mut(&mut self) -> &mut LegendPosition {
        self.legend_position_mut()
    }

    pub fn set_legend_position(&mut self, value: LegendPosition) -> &mut Self {
        self.legend_position = value;
        self
    }

    #[must_use]
    pub fn layout(&self) -> Option<&Layout> {
        self.layout.as_deref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use layout()")]
    pub fn get_layout(&self) -> Option<&Layout> {
        self.layout()
    }

    pub fn layout_mut(&mut self) -> Option<&mut Layout> {
        self.layout.as_deref_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use layout_mut()")]
    pub fn get_layout_mut(&mut self) -> Option<&mut Layout> {
        self.layout_mut()
    }

    pub fn set_layout(&mut self, value: Layout) -> &mut Self {
        self.layout = Some(Box::new(value));
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

    pub fn set_overlay(&mut self, value: Overlay) -> &mut Self {
        self.overlay = value;
        self
    }

    #[must_use]
    pub fn shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_deref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape_properties()")]
    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties()
    }

    pub fn shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_deref_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use shape_properties_mut()")]
    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(Box::new(value));
        self
    }

    #[must_use]
    pub fn text_properties(&self) -> Option<&TextProperties> {
        self.text_properties.as_deref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use text_properties()")]
    pub fn get_text_properties(&self) -> Option<&TextProperties> {
        self.text_properties()
    }

    pub fn text_properties_mut(&mut self) -> Option<&mut TextProperties> {
        self.text_properties.as_deref_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use text_properties_mut()")]
    pub fn get_text_properties_mut(&mut self) -> Option<&mut TextProperties> {
        self.text_properties_mut()
    }

    pub fn set_text_properties(&mut self, value: TextProperties) -> &mut Self {
        self.text_properties = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().into_inner() {
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
            Event::Empty(ref e) => match e.name().into_inner() {
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
            Event::End(ref e) => {
                if  e.name().into_inner() == b"c:legend" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:legend"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:legend
        write_start_tag(writer, "c:legend", vec![], false);

        // c:legendPos
        self.legend_position.write_to(writer);

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

        // c:txPr
        if let Some(v) = &self.text_properties {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:legend");
    }
}
