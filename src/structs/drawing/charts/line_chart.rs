use crate::xml_read_loop;

// lineChart
use super::AreaChartSeries;
use super::AreaChartSeriesList;
use super::AxisId;
use super::DataLabels;
use super::Grouping;
use super::ShowMarker;
use super::Smooth;
use super::VaryColors;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LineChart {
    grouping: Grouping,
    vary_colors: VaryColors,
    area_chart_series_list: AreaChartSeriesList,
    data_labels: DataLabels,
    show_marker: ShowMarker,
    smooth: Smooth,
    axis_id: Vec<AxisId>,
}

impl LineChart {
    pub fn get_grouping(&self) -> &Grouping {
        &self.grouping
    }

    pub fn get_grouping_mut(&mut self) -> &mut Grouping {
        &mut self.grouping
    }

    pub fn set_grouping(&mut self, value: Grouping) -> &mut Self {
        self.grouping = value;
        self
    }

    pub fn get_vary_colors(&self) -> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self) -> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value: VaryColors) -> &mut Self {
        self.vary_colors = value;
        self
    }

    pub fn get_area_chart_series_list(&self) -> &AreaChartSeriesList {
        &self.area_chart_series_list
    }

    pub fn get_area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        &mut self.area_chart_series_list
    }

    pub fn set_area_chart_series_list(&mut self, value: AreaChartSeriesList) -> &mut Self {
        self.area_chart_series_list = value;
        self
    }

    pub fn get_data_labels(&self) -> &DataLabels {
        &self.data_labels
    }

    pub fn get_data_labels_mut(&mut self) -> &mut DataLabels {
        &mut self.data_labels
    }

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut Self {
        self.data_labels = value;
        self
    }

    pub fn get_show_marker(&self) -> &ShowMarker {
        &self.show_marker
    }

    pub fn get_show_marker_mut(&mut self) -> &mut ShowMarker {
        &mut self.show_marker
    }

    pub fn set_show_marker(&mut self, value: ShowMarker) -> &mut Self {
        self.show_marker = value;
        self
    }

    pub fn get_smooth(&self) -> &Smooth {
        &self.smooth
    }

    pub fn get_smooth_mut(&mut self) -> &mut Smooth {
        &mut self.smooth
    }

    pub fn set_smooth(&mut self, value: Smooth) -> &mut Self {
        self.smooth = value;
        self
    }

    pub fn get_axis_id(&self) -> &Vec<AxisId> {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self) -> &mut Vec<AxisId> {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value: Vec<AxisId>) -> &mut Self {
        self.axis_id = value;
        self
    }

    pub fn add_axis_id(&mut self, value: AxisId) -> &mut Self {
        self.axis_id.push(value);
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
                b"c:ser" => {
                    let mut obj = AreaChartSeries::default();
                    obj.set_attributes(reader, e);
                    self.get_area_chart_series_list_mut()
                        .add_area_chart_series(obj);
                }
                b"c:dLbls" => {
                    self.data_labels.set_attributes(reader, e);
                }
                _ => (),
            },
            Event::Empty(ref e) => match e.name().into_inner() {
                b"c:grouping" => {
                    self.grouping.set_attributes(reader, e);
                }
                b"c:varyColors" => {
                    self.vary_colors.set_attributes(reader, e);
                }
                b"c:marker" => {
                    self.show_marker.set_attributes(reader, e);
                }
                b"c:smooth" => {
                    self.smooth.set_attributes(reader, e);
                }
                b"c:axId" => {
                    let mut obj = AxisId::default();
                    obj.set_attributes(reader, e);
                    self.add_axis_id(obj);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:lineChart" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:lineChart"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:lineChart
        write_start_tag(writer, "c:lineChart", vec![], false);

        // c:grouping
        self.grouping.write_to(writer);

        // c:varyColors
        self.vary_colors.write_to(writer);

        // c:ser
        for v in self.area_chart_series_list.get_area_chart_series() {
            v.write_to(writer, spreadsheet);
        }

        // c:dLbls
        self.data_labels.write_to(writer);

        // c:marker
        self.show_marker.write_to(writer);

        // c:smooth
        self.smooth.write_to(writer);

        // c:axId
        for v in &self.axis_id {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:lineChart");
    }
}
