// c:barChart
use super::BarDirection;
use super::Grouping;
use super::VaryColors;
use super::AreaChartSeries;
use super::DataLabels;
use super::GapWidth;
use super::Overlap;
use super::AxisId;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct BarChart {
    bar_direction: BarDirection,
    grouping: Grouping,
    vary_colors: VaryColors,
    area_chart_series: Vec<AreaChartSeries>,
    data_labels: DataLabels,
    gap_width: GapWidth,
    overlap: Overlap,
    axis_id: Vec<AxisId>,
}
impl BarChart {
    pub fn get_bar_direction(&self)-> &BarDirection {
        &self.bar_direction
    }

    pub fn get_bar_direction_mut(&mut self)-> &mut BarDirection {
        &mut self.bar_direction
    }

    pub fn set_bar_direction(&mut self, value:BarDirection)-> &mut BarChart {
        self.bar_direction = value;
        self
    }

    pub fn get_grouping(&self)-> &Grouping {
        &self.grouping
    }

    pub fn get_grouping_mut(&mut self)-> &mut Grouping {
        &mut self.grouping
    }

    pub fn set_grouping(&mut self, value:Grouping)-> &mut BarChart {
        self.grouping = value;
        self
    }

    pub fn get_vary_colors(&self)-> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self)-> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value:VaryColors)-> &mut BarChart {
        self.vary_colors = value;
        self
    }

    pub fn get_area_chart_series(&self)-> &Vec<AreaChartSeries> {
        &self.area_chart_series
    }

    pub fn get_area_chart_series_mut(&mut self)-> &mut Vec<AreaChartSeries> {
        &mut self.area_chart_series
    }

    pub fn set_area_chart_series(&mut self, value:Vec<AreaChartSeries>)-> &mut BarChart {
        self.area_chart_series = value;
        self
    }

    pub fn add_area_chart_series(&mut self, value:AreaChartSeries)-> &mut BarChart {
        self.area_chart_series.push(value);
        self
    }

    pub fn get_data_labels(&self)-> &DataLabels {
        &self.data_labels
    }

    pub fn get_data_labels_mut(&mut self)-> &mut DataLabels {
        &mut self.data_labels
    }

    pub fn set_data_labels(&mut self, value:DataLabels)-> &mut BarChart {
        self.data_labels = value;
        self
    }

    pub fn get_gap_width(&self)-> &GapWidth {
        &self.gap_width
    }

    pub fn get_gap_width_mut(&mut self)-> &mut GapWidth {
        &mut self.gap_width
    }

    pub fn set_gap_width(&mut self, value:GapWidth)-> &mut BarChart {
        self.gap_width = value;
        self
    }

    pub fn get_overlap(&self)-> &Overlap {
        &self.overlap
    }

    pub fn get_overlap_mut(&mut self)-> &mut Overlap {
        &mut self.overlap
    }

    pub fn set_overlap(&mut self, value:Overlap)-> &mut BarChart {
        self.overlap = value;
        self
    }

    pub fn get_axis_id(&self)-> &Vec<AxisId> {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self)-> &mut Vec<AxisId> {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value:Vec<AxisId>)-> &mut BarChart {
        self.axis_id = value;
        self
    }

    pub fn add_axis_id(&mut self, value:AxisId)-> &mut BarChart {
        self.axis_id.push(value);
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
                        b"c:ser" => {
                            let mut obj = AreaChartSeries::default();
                            obj.set_attributes(reader, e);
                            self.add_area_chart_series(obj);
                        },
                        b"c:dLbls" => {
                            self.data_labels.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:barDir" => {
                            self.bar_direction.set_attributes(reader, e);
                        },
                        b"c:grouping" => {
                            self.grouping.set_attributes(reader, e);
                        },
                        b"c:varyColors" => {
                            self.vary_colors.set_attributes(reader, e);
                        },
                        b"c:gapWidth" => {
                            self.gap_width.set_attributes(reader, e);
                        },
                        b"c:overlap" => {
                            self.overlap.set_attributes(reader, e);
                        },
                        b"c:axId" => {
                            let mut obj = AxisId::default();
                            obj.set_attributes(reader, e);
                            self.add_axis_id(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:barChart" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:barChart"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:barChart
        write_start_tag(writer, "c:barChart", vec![], false);

        // c:barDir
        &self.bar_direction.write_to(writer);

        // c:grouping
        &self.grouping.write_to(writer);

        // c:varyColors
        &self.vary_colors.write_to(writer);

        // c:ser
        for v in &self.area_chart_series {
            v.write_to(writer);
        }

        // c:dLbls
        &self.data_labels.write_to(writer);

        // c:gapWidth
        &self.gap_width.write_to(writer);

        // c:overlap
        &self.overlap.write_to(writer);

        // c:axId
        for v in &self.axis_id {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:barChart");
    }
}
