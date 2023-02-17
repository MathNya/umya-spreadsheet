// c:serAx
use super::AxisId;
use super::AxisPosition;
use super::Crosses;
use super::CrossingAxis;
use super::Delete;
use super::MajorGridlines;
use super::MajorTickMark;
use super::MinorTickMark;
use super::Scaling;
use super::TickLabelPosition;
use super::Title;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SeriesAxis {
    axis_id: AxisId,
    scaling: Scaling,
    delete: Delete,
    axis_position: AxisPosition,
    major_gridlines: Option<MajorGridlines>,
    title: Option<Title>,
    major_tick_mark: MajorTickMark,
    minor_tick_mark: MinorTickMark,
    tick_label_position: TickLabelPosition,
    crossing_axis: CrossingAxis,
    crosses: Crosses,
}
impl SeriesAxis {
    pub fn get_axis_id(&self) -> &AxisId {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self) -> &mut AxisId {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value: AxisId) -> &mut SeriesAxis {
        self.axis_id = value;
        self
    }

    pub fn get_scaling(&self) -> &Scaling {
        &self.scaling
    }

    pub fn get_scaling_mut(&mut self) -> &mut Scaling {
        &mut self.scaling
    }

    pub fn set_scaling(&mut self, value: Scaling) -> &mut SeriesAxis {
        self.scaling = value;
        self
    }

    pub fn get_delete(&self) -> &Delete {
        &self.delete
    }

    pub fn get_delete_mut(&mut self) -> &mut Delete {
        &mut self.delete
    }

    pub fn set_delete(&mut self, value: Delete) -> &mut SeriesAxis {
        self.delete = value;
        self
    }

    pub fn get_axis_position(&self) -> &AxisPosition {
        &self.axis_position
    }

    pub fn get_axis_position_mut(&mut self) -> &mut AxisPosition {
        &mut self.axis_position
    }

    pub fn set_axis_position(&mut self, value: AxisPosition) -> &mut SeriesAxis {
        self.axis_position = value;
        self
    }

    pub fn get_major_gridlines(&self) -> &Option<MajorGridlines> {
        &self.major_gridlines
    }

    pub fn get_major_gridlines_mut(&mut self) -> &mut Option<MajorGridlines> {
        &mut self.major_gridlines
    }

    pub fn set_major_gridlines(&mut self, value: MajorGridlines) -> &mut SeriesAxis {
        self.major_gridlines = Some(value);
        self
    }

    pub fn get_title(&self) -> &Option<Title> {
        &self.title
    }

    pub fn get_title_mut(&mut self) -> &mut Option<Title> {
        &mut self.title
    }

    pub fn set_title(&mut self, value: Title) -> &mut SeriesAxis {
        self.title = Some(value);
        self
    }

    pub fn get_major_tick_mark(&self) -> &MajorTickMark {
        &self.major_tick_mark
    }

    pub fn get_major_tick_mark_mut(&mut self) -> &mut MajorTickMark {
        &mut self.major_tick_mark
    }

    pub fn set_major_tick_mark(&mut self, value: MajorTickMark) -> &mut SeriesAxis {
        self.major_tick_mark = value;
        self
    }

    pub fn get_minor_tick_mark(&self) -> &MinorTickMark {
        &self.minor_tick_mark
    }

    pub fn get_minor_tick_mark_mut(&mut self) -> &mut MinorTickMark {
        &mut self.minor_tick_mark
    }

    pub fn set_minor_tick_mark(&mut self, value: MinorTickMark) -> &mut SeriesAxis {
        self.minor_tick_mark = value;
        self
    }

    pub fn get_tick_label_position(&self) -> &TickLabelPosition {
        &self.tick_label_position
    }

    pub fn get_tick_label_position_mut(&mut self) -> &mut TickLabelPosition {
        &mut self.tick_label_position
    }

    pub fn set_tick_label_position(&mut self, value: TickLabelPosition) -> &mut SeriesAxis {
        self.tick_label_position = value;
        self
    }

    pub fn get_tick_crossing_axis(&self) -> &CrossingAxis {
        &self.crossing_axis
    }

    pub fn get_tick_crossing_axis_mut(&mut self) -> &mut CrossingAxis {
        &mut self.crossing_axis
    }

    pub fn set_tick_crossing_axis(&mut self, value: CrossingAxis) -> &mut SeriesAxis {
        self.crossing_axis = value;
        self
    }

    pub fn get_crosses(&self) -> &Crosses {
        &self.crosses
    }

    pub fn get_crosses_mut(&mut self) -> &mut Crosses {
        &mut self.crosses
    }

    pub fn set_crosses(&mut self, value: Crosses) -> &mut SeriesAxis {
        self.crosses = value;
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
                    b"c:scaling" => {
                        self.scaling.set_attributes(reader, e);
                    }
                    b"c:title" => {
                        let mut obj = Title::default();
                        obj.set_attributes(reader, e);
                        self.set_title(obj);
                    }
                    b"c:majorGridlines" => {
                        let mut obj = MajorGridlines::default();
                        obj.set_attributes(reader, e, false);
                        self.set_major_gridlines(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
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
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:serAx" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:serAx"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:serAx
        write_start_tag(writer, "c:serAx", vec![], false);

        // c:axId
        self.axis_id.write_to(writer);

        // c:scaling
        self.scaling.write_to(writer);

        // c:delete
        self.delete.write_to(writer);

        // c:axPos
        self.axis_position.write_to(writer);

        // c:majorGridlines
        match &self.major_gridlines {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:title
        match &self.title {
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

        // c:crossAx
        self.crossing_axis.write_to(writer);

        // c:crosses
        self.crosses.write_to(writer);

        write_end_tag(writer, "c:serAx");
    }
}
