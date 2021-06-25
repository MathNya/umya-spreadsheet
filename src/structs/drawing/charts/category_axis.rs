// c:catAx
use super::AxisId;
use super::Scaling;
use super::Delete;
use super::AxisPosition;
use super::MajorTickMark;
use super::MinorTickMark;
use super::TickLabelPosition;
use super::CrossingAxis;
use super::Crosses;
use super::AutoLabeled;
use super::LabelAlignment;
use super::LabelOffset;
use super::NoMultiLevelLabels;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct CategoryAxis {
    axis_id: AxisId,
    scaling: Scaling,
    delete: Delete,
    axis_position: AxisPosition,
    major_tick_mark: MajorTickMark,
    minor_tick_mark: MinorTickMark,
    tick_label_position: TickLabelPosition,
    crossing_axis: CrossingAxis,
    crosses: Crosses,
    auto_labeled: AutoLabeled,
    label_alignment: LabelAlignment,
    label_offset: LabelOffset,
    no_multi_level_labels: NoMultiLevelLabels,
}
impl CategoryAxis {
    pub fn get_axis_id(&self)-> &AxisId {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self)-> &AxisId {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value:AxisId)-> &mut CategoryAxis {
        self.axis_id = value;
        self
    }

    pub fn get_scaling(&self)-> &Scaling {
        &self.scaling
    }

    pub fn get_scaling_mut(&mut self)-> &Scaling {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, value:Scaling)-> &mut CategoryAxis {
        self.scaling = value;
        self
    }

    pub fn get_delete(&self)-> &Delete {
        &self.delete
    }

    pub fn get_delete_mut(&mut self)-> &Delete {
        &mut self.delete
    }

    pub fn set_delete(&mut self, value:Delete)-> &mut CategoryAxis {
        self.delete = value;
        self
    }

    pub fn get_axis_position(&self)-> &AxisPosition {
        &self.axis_position
    }

    pub fn get_axis_position_mut(&mut self)-> &AxisPosition {
        &mut self.axis_position
    }

    pub fn set_axis_position(&mut self, value:AxisPosition)-> &mut CategoryAxis {
        self.axis_position = value;
        self
    }

    pub fn get_major_tick_mark(&self)-> &MajorTickMark {
        &self.major_tick_mark
    }

    pub fn get_major_tick_mark_mut(&mut self)-> &MajorTickMark {
        &mut self.major_tick_mark
    }

    pub fn set_major_tick_mark(&mut self, value:MajorTickMark)-> &mut CategoryAxis {
        self.major_tick_mark = value;
        self
    }

    pub fn get_minor_tick_mark(&self)-> &MinorTickMark {
        &self.minor_tick_mark
    }

    pub fn get_minor_tick_mark_mut(&mut self)-> &MinorTickMark {
        &mut self.minor_tick_mark
    }

    pub fn set_minor_tick_mark(&mut self, value:MinorTickMark)-> &mut CategoryAxis {
        self.minor_tick_mark = value;
        self
    }

    pub fn get_tick_label_position(&self)-> &TickLabelPosition {
        &self.tick_label_position
    }

    pub fn get_tick_label_position_mut(&mut self)-> &TickLabelPosition {
        &mut self.tick_label_position
    }

    pub fn set_tick_label_position(&mut self, value:TickLabelPosition)-> &mut CategoryAxis {
        self.tick_label_position = value;
        self
    }

    pub fn get_tick_crossing_axis(&self)-> &CrossingAxis {
        &self.crossing_axis
    }

    pub fn get_tick_crossing_axis_mut(&mut self)-> &CrossingAxis {
        &mut self.crossing_axis
    }

    pub fn set_tick_crossing_axis(&mut self, value:CrossingAxis)-> &mut CategoryAxis {
        self.crossing_axis = value;
        self
    }

    pub fn get_crosses(&self)-> &Crosses {
        &self.crosses
    }

    pub fn get_crosses_mut(&mut self)-> &Crosses {
        &mut self.crosses
    }

    pub fn set_crosses(&mut self, value:Crosses)-> &mut CategoryAxis {
        self.crosses = value;
        self
    }

    pub fn get_auto_labeled(&self)-> &AutoLabeled {
        &self.auto_labeled
    }

    pub fn get_auto_labeled_mut(&mut self)-> &AutoLabeled {
        &mut self.auto_labeled
    }

    pub fn set_auto_labeled(&mut self, value:AutoLabeled)-> &mut CategoryAxis {
        self.auto_labeled = value;
        self
    }

    pub fn get_label_alignment(&self)-> &LabelAlignment {
        &self.label_alignment
    }

    pub fn get_label_alignment_mut(&mut self)-> &LabelAlignment {
        &mut self.label_alignment
    }

    pub fn set_label_alignment(&mut self, value:LabelAlignment)-> &mut CategoryAxis {
        self.label_alignment = value;
        self
    }

    pub fn get_label_offset(&self)-> &LabelOffset {
        &self.label_offset
    }

    pub fn get_label_offset_mut(&mut self)-> &LabelOffset {
        &mut self.label_offset
    }

    pub fn set_label_offset(&mut self, value:LabelOffset)-> &mut CategoryAxis {
        self.label_offset = value;
        self
    }

    pub fn get_no_multi_level_labels(&self)-> &NoMultiLevelLabels {
        &self.no_multi_level_labels
    }

    pub fn get_no_multi_level_labels_mut(&mut self)-> &NoMultiLevelLabels {
        &mut self.no_multi_level_labels
    }

    pub fn set_no_multi_level_labels(&mut self, value:NoMultiLevelLabels)-> &mut CategoryAxis {
        self.no_multi_level_labels = value;
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:scaling" => {
                            self.scaling.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:axId" => {
                            self.axis_id.set_attributes(reader, e);
                        },
                        b"c:delete" => {
                            self.delete.set_attributes(reader, e);
                        },
                        b"c:axPos" => {
                            self.axis_position.set_attributes(reader, e);
                        },
                        b"c:majorTickMark" => {
                            self.major_tick_mark.set_attributes(reader, e);
                        },
                        b"c:minorTickMark" => {
                            self.minor_tick_mark.set_attributes(reader, e);
                        },
                        b"c:tickLblPos" => {
                            self.tick_label_position.set_attributes(reader, e);
                        },
                        b"c:crossAx" => {
                            self.crossing_axis.set_attributes(reader, e);
                        },
                        b"c:crosses" => {
                            self.crosses.set_attributes(reader, e);
                        },
                        b"c:auto" => {
                            self.auto_labeled.set_attributes(reader, e);
                        },
                        b"c:lblAlgn" => {
                            self.label_alignment.set_attributes(reader, e);
                        },
                        b"c:lblOffset" => {
                            self.label_offset.set_attributes(reader, e);
                        },
                        b"c:noMultiLvlLbl" => {
                            self.no_multi_level_labels.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:catAx" => return,
                        _ => (),
                    }
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
        &self.axis_id.write_to(writer);

        // c:scaling
        &self.scaling.write_to(writer);

        // c:delete
        &self.delete.write_to(writer);

        // c:axPos
        &self.axis_position.write_to(writer);

        // c:majorTickMark
        &self.major_tick_mark.write_to(writer);

        // c:minorTickMark
        &self.minor_tick_mark.write_to(writer);

        // c:tickLblPos
        &self.tick_label_position.write_to(writer);

        // c:crossAx
        &self.crossing_axis.write_to(writer);

        // c:crosses
        &self.crosses.write_to(writer);

        // c:auto
        &self.auto_labeled.write_to(writer);

        // c:lblAlgn
        &self.label_alignment.write_to(writer);

        // c:lblOffset
        &self.label_offset.write_to(writer);

        // c:noMultiLvlLbl
        &self.no_multi_level_labels.write_to(writer);

        write_end_tag(writer, "c:catAx");
    }
}
