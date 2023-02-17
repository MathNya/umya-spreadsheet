// c:catAx
use super::AutoLabeled;
use super::AxisId;
use super::AxisPosition;
use super::Crosses;
use super::CrossingAxis;
use super::Delete;
use super::LabelAlignment;
use super::LabelOffset;
use super::MajorGridlines;
use super::MajorTickMark;
use super::MinorTickMark;
use super::NoMultiLevelLabels;
use super::Scaling;
use super::ShapeProperties;
use super::TextProperties;
use super::TickLabelPosition;
use super::Title;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CategoryAxis {
    axis_id: AxisId,
    scaling: Scaling,
    delete: Delete,
    axis_position: AxisPosition,
    title: Option<Title>,
    major_gridlines: Option<MajorGridlines>,
    major_tick_mark: MajorTickMark,
    minor_tick_mark: MinorTickMark,
    tick_label_position: TickLabelPosition,
    crossing_axis: CrossingAxis,
    crosses: Crosses,
    auto_labeled: AutoLabeled,
    label_alignment: LabelAlignment,
    label_offset: LabelOffset,
    no_multi_level_labels: NoMultiLevelLabels,
    shape_properties: Option<ShapeProperties>,
    text_properties: Option<TextProperties>,
}
impl CategoryAxis {
    pub fn get_axis_id(&self) -> &AxisId {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self) -> &mut AxisId {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value: AxisId) -> &mut Self {
        self.axis_id = value;
        self
    }

    pub fn get_scaling(&self) -> &Scaling {
        &self.scaling
    }

    pub fn get_scaling_mut(&mut self) -> &mut Scaling {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, value: Scaling) -> &mut Self {
        self.scaling = value;
        self
    }

    pub fn get_delete(&self) -> &Delete {
        &self.delete
    }

    pub fn get_delete_mut(&mut self) -> &mut Delete {
        &mut self.delete
    }

    pub fn set_delete(&mut self, value: Delete) -> &mut Self {
        self.delete = value;
        self
    }

    pub fn get_axis_position(&self) -> &AxisPosition {
        &self.axis_position
    }

    pub fn get_axis_position_mut(&mut self) -> &mut AxisPosition {
        &mut self.axis_position
    }

    pub fn set_axis_position(&mut self, value: AxisPosition) -> &mut Self {
        self.axis_position = value;
        self
    }

    pub fn get_title(&self) -> &Option<Title> {
        &self.title
    }

    pub fn get_title_mut(&mut self) -> &mut Option<Title> {
        &mut self.title
    }

    pub fn set_title(&mut self, value: Title) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn get_major_gridlines(&self) -> &Option<MajorGridlines> {
        &self.major_gridlines
    }

    pub fn get_major_gridlines_mut(&mut self) -> &mut Option<MajorGridlines> {
        &mut self.major_gridlines
    }

    pub fn set_major_gridlines(&mut self, value: MajorGridlines) -> &mut Self {
        self.major_gridlines = Some(value);
        self
    }

    pub fn get_major_tick_mark(&self) -> &MajorTickMark {
        &self.major_tick_mark
    }

    pub fn get_major_tick_mark_mut(&mut self) -> &mut MajorTickMark {
        &mut self.major_tick_mark
    }

    pub fn set_major_tick_mark(&mut self, value: MajorTickMark) -> &mut Self {
        self.major_tick_mark = value;
        self
    }

    pub fn get_minor_tick_mark(&self) -> &MinorTickMark {
        &self.minor_tick_mark
    }

    pub fn get_minor_tick_mark_mut(&mut self) -> &mut MinorTickMark {
        &mut self.minor_tick_mark
    }

    pub fn set_minor_tick_mark(&mut self, value: MinorTickMark) -> &mut Self {
        self.minor_tick_mark = value;
        self
    }

    pub fn get_tick_label_position(&self) -> &TickLabelPosition {
        &self.tick_label_position
    }

    pub fn get_tick_label_position_mut(&mut self) -> &mut TickLabelPosition {
        &mut self.tick_label_position
    }

    pub fn set_tick_label_position(&mut self, value: TickLabelPosition) -> &mut Self {
        self.tick_label_position = value;
        self
    }

    pub fn get_tick_crossing_axis(&self) -> &CrossingAxis {
        &self.crossing_axis
    }

    pub fn get_tick_crossing_axis_mut(&mut self) -> &mut CrossingAxis {
        &mut self.crossing_axis
    }

    pub fn set_tick_crossing_axis(&mut self, value: CrossingAxis) -> &mut Self {
        self.crossing_axis = value;
        self
    }

    pub fn get_crosses(&self) -> &Crosses {
        &self.crosses
    }

    pub fn get_crosses_mut(&mut self) -> &mut Crosses {
        &mut self.crosses
    }

    pub fn set_crosses(&mut self, value: Crosses) -> &mut Self {
        self.crosses = value;
        self
    }

    pub fn get_auto_labeled(&self) -> &AutoLabeled {
        &self.auto_labeled
    }

    pub fn get_auto_labeled_mut(&mut self) -> &mut AutoLabeled {
        &mut self.auto_labeled
    }

    pub fn set_auto_labeled(&mut self, value: AutoLabeled) -> &mut Self {
        self.auto_labeled = value;
        self
    }

    pub fn get_label_alignment(&self) -> &LabelAlignment {
        &self.label_alignment
    }

    pub fn get_label_alignment_mut(&mut self) -> &mut LabelAlignment {
        &mut self.label_alignment
    }

    pub fn set_label_alignment(&mut self, value: LabelAlignment) -> &mut Self {
        self.label_alignment = value;
        self
    }

    pub fn get_label_offset(&self) -> &LabelOffset {
        &self.label_offset
    }

    pub fn get_label_offset_mut(&mut self) -> &mut LabelOffset {
        &mut self.label_offset
    }

    pub fn set_label_offset(&mut self, value: LabelOffset) -> &mut Self {
        self.label_offset = value;
        self
    }

    pub fn get_no_multi_level_labels(&self) -> &NoMultiLevelLabels {
        &self.no_multi_level_labels
    }

    pub fn get_no_multi_level_labels_mut(&mut self) -> &mut NoMultiLevelLabels {
        &mut self.no_multi_level_labels
    }

    pub fn set_no_multi_level_labels(&mut self, value: NoMultiLevelLabels) -> &mut Self {
        self.no_multi_level_labels = value;
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
                    b"c:title" => {
                        let mut obj = Title::default();
                        obj.set_attributes(reader, e);
                        self.set_title(obj);
                    }
                    b"c:scaling" => {
                        self.scaling.set_attributes(reader, e);
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
                    b"c:majorGridlines" => {
                        let mut obj = MajorGridlines::default();
                        obj.set_attributes(reader, e, false);
                        self.set_major_gridlines(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:axId" => {
                        self.axis_id.set_attributes(reader, e);
                    }
                    b"c:delete" => {
                        self.delete.set_attributes(reader, e);
                    }
                    b"c:axPos" => {
                        self.axis_position.set_attributes(reader, e);
                    }
                    b"c:majorGridlines" => {
                        let mut obj = MajorGridlines::default();
                        obj.set_attributes(reader, e, true);
                        self.set_major_gridlines(obj);
                    }
                    b"c:majorTickMark" => {
                        self.major_tick_mark.set_attributes(reader, e);
                    }
                    b"c:minorTickMark" => {
                        self.minor_tick_mark.set_attributes(reader, e);
                    }
                    b"c:tickLblPos" => {
                        self.tick_label_position.set_attributes(reader, e);
                    }
                    b"c:crossAx" => {
                        self.crossing_axis.set_attributes(reader, e);
                    }
                    b"c:crosses" => {
                        self.crosses.set_attributes(reader, e);
                    }
                    b"c:auto" => {
                        self.auto_labeled.set_attributes(reader, e);
                    }
                    b"c:lblAlgn" => {
                        self.label_alignment.set_attributes(reader, e);
                    }
                    b"c:lblOffset" => {
                        self.label_offset.set_attributes(reader, e);
                    }
                    b"c:noMultiLvlLbl" => {
                        self.no_multi_level_labels.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:catAx" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:catAx"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:catAx
        write_start_tag(writer, "c:catAx", vec![], false);

        // c:axId
        self.axis_id.write_to(writer);

        // c:scaling
        self.scaling.write_to(writer);

        // c:delete
        self.delete.write_to(writer);

        // c:axPos
        self.axis_position.write_to(writer);

        // c:title
        match &self.title {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:majorGridlines
        match &self.major_gridlines {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:majorTickMark
        self.major_tick_mark.write_to(writer);

        // c:minorTickMark
        self.minor_tick_mark.write_to(writer);

        // c:tickLblPos
        self.tick_label_position.write_to(writer);

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

        // c:crossAx
        self.crossing_axis.write_to(writer);

        // c:crosses
        self.crosses.write_to(writer);

        // c:auto
        self.auto_labeled.write_to(writer);

        // c:lblAlgn
        self.label_alignment.write_to(writer);

        // c:lblOffset
        self.label_offset.write_to(writer);

        // c:noMultiLvlLbl
        self.no_multi_level_labels.write_to(writer);

        write_end_tag(writer, "c:catAx");
    }
}
