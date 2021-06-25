// c:valAx
use super::AxisId;
use super::Scaling;
use super::Delete;
use super::AxisPosition;
use super::MajorGridlines;
use super::Title;
use super::NumberingFormat;
use super::MajorTickMark;
use super::MinorTickMark;
use super::TickLabelPosition;
use super::CrossingAxis;
use super::Crosses;
use super::CrossBetween;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ValueAxis {
    axis_id: AxisId,
    scaling: Scaling,
    delete: Delete,
    axis_position: AxisPosition,
    major_gridlines: MajorGridlines,
    title: Option<Title>,
    numbering_format: NumberingFormat,
    major_tick_mark: MajorTickMark,
    minor_tick_mark: MinorTickMark,
    tick_label_position: TickLabelPosition,
    crossing_axis: CrossingAxis,
    crosses: Crosses,
    cross_between: CrossBetween,
}
impl ValueAxis {
    pub fn get_axis_id(&self)-> &AxisId {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self)-> &AxisId {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value:AxisId)-> &mut ValueAxis {
        self.axis_id = value;
        self
    }

    pub fn get_scaling(&self)-> &Scaling {
        &self.scaling
    }

    pub fn get_scaling_mut(&mut self)-> &Scaling {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, value:Scaling)-> &mut ValueAxis {
        self.scaling = value;
        self
    }

    pub fn get_delete(&self)-> &Delete {
        &self.delete
    }

    pub fn get_delete_mut(&mut self)-> &Delete {
        &mut self.delete
    }

    pub fn set_delete(&mut self, value:Delete)-> &mut ValueAxis {
        self.delete = value;
        self
    }

    pub fn get_axis_position(&self)-> &AxisPosition {
        &self.axis_position
    }

    pub fn get_axis_position_mut(&mut self)-> &AxisPosition {
        &mut self.axis_position
    }

    pub fn set_axis_position(&mut self, value:AxisPosition)-> &mut ValueAxis {
        self.axis_position = value;
        self
    }

    pub fn get_major_gridlines(&self)-> &MajorGridlines {
        &self.major_gridlines
    }

    pub fn get_major_gridlines_mut(&mut self)-> &MajorGridlines {
        &mut self.major_gridlines
    }

    pub fn set_major_gridlines(&mut self, value:MajorGridlines)-> &mut ValueAxis {
        self.major_gridlines = value;
        self
    }

    pub fn get_title(&self)-> &Option<Title> {
        &self.title
    }

    pub fn get_title_mut(&mut self)-> &Option<Title> {
        &mut self.title
    }

    pub fn set_title(&mut self, value:Title)-> &mut ValueAxis {
        self.title = Some(value);
        self
    }

    pub fn get_numbering_format(&self)-> &NumberingFormat {
        &self.numbering_format
    }

    pub fn get_numbering_format_mut(&mut self)-> &NumberingFormat {
        &mut self.numbering_format
    }

    pub fn set_numbering_format(&mut self, value:NumberingFormat)-> &mut ValueAxis {
        self.numbering_format = value;
        self
    }

    pub fn get_major_tick_mark(&self)-> &MajorTickMark {
        &self.major_tick_mark
    }

    pub fn get_major_tick_mark_mut(&mut self)-> &MajorTickMark {
        &mut self.major_tick_mark
    }

    pub fn set_major_tick_mark(&mut self, value:MajorTickMark)-> &mut ValueAxis {
        self.major_tick_mark = value;
        self
    }

    pub fn get_minor_tick_mark(&self)-> &MinorTickMark {
        &self.minor_tick_mark
    }

    pub fn get_minor_tick_mark_mut(&mut self)-> &MinorTickMark {
        &mut self.minor_tick_mark
    }

    pub fn set_minor_tick_mark(&mut self, value:MinorTickMark)-> &mut ValueAxis {
        self.minor_tick_mark = value;
        self
    }

    pub fn get_tick_label_position(&self)-> &TickLabelPosition {
        &self.tick_label_position
    }

    pub fn get_tick_label_position_mut(&mut self)-> &TickLabelPosition {
        &mut self.tick_label_position
    }

    pub fn set_tick_label_position(&mut self, value:TickLabelPosition)-> &mut ValueAxis {
        self.tick_label_position = value;
        self
    }

    pub fn get_tick_crossing_axis(&self)-> &CrossingAxis {
        &self.crossing_axis
    }

    pub fn get_tick_crossing_axis_mut(&mut self)-> &CrossingAxis {
        &mut self.crossing_axis
    }

    pub fn set_tick_crossing_axis(&mut self, value:CrossingAxis)-> &mut ValueAxis {
        self.crossing_axis = value;
        self
    }

    pub fn get_crosses(&self)-> &Crosses {
        &self.crosses
    }

    pub fn get_crosses_mut(&mut self)-> &Crosses {
        &mut self.crosses
    }

    pub fn set_crosses(&mut self, value:Crosses)-> &mut ValueAxis {
        self.crosses = value;
        self
    }

    pub fn get_cross_between(&self)-> &CrossBetween {
        &self.cross_between
    }

    pub fn get_cross_between_mut(&mut self)-> &CrossBetween {
        &mut self.cross_between
    }

    pub fn set_cross_between(&mut self, value:CrossBetween)-> &mut ValueAxis {
        self.cross_between = value;
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
                        b"c:title" => {
                            let mut obj = Title::default();
                            obj.set_attributes(reader, e);
                            self.set_title(obj);
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
                        b"c:numFmt" => {
                            self.numbering_format.set_attributes(reader, e);
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
                        b"c:crossBetween" => {
                            self.cross_between.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:valAx" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:valAx"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:valAx
        write_start_tag(writer, "c:valAx", vec![], false);

        // c:axId
        &self.axis_id.write_to(writer);

        // c:scaling
        &self.scaling.write_to(writer);

        // c:delete
        &self.delete.write_to(writer);

        // c:axPos
        &self.axis_position.write_to(writer);

        // majorGridlines
        &self.major_gridlines.write_to(writer);
        
        // c:title
        match &self.title {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:numFmt
        &self.numbering_format.write_to(writer);

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

        // c:crossBetween
        &self.cross_between.write_to(writer);

        write_end_tag(writer, "c:valAx");
    }
}
