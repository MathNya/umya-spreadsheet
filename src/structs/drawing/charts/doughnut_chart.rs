// c:doughnutChart
use super::AreaChartSeries;
use super::AreaChartSeriesList;
use super::DataLabels;
use super::FirstSliceAngle;
use super::HoleSize;
use super::VaryColors;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct DoughnutChart {
    vary_colors: VaryColors,
    area_chart_series_list: AreaChartSeriesList,
    data_labels: DataLabels,
    first_slice_angle: FirstSliceAngle,
    hole_size: HoleSize,
}
impl DoughnutChart {
    pub fn get_vary_colors(&self) -> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self) -> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value: VaryColors) -> &mut DoughnutChart {
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

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut DoughnutChart {
        self.data_labels = value;
        self
    }

    pub fn get_first_slice_angle(&self) -> &FirstSliceAngle {
        &self.first_slice_angle
    }

    pub fn get_first_slice_angle_mut(&mut self) -> &mut FirstSliceAngle {
        &mut self.first_slice_angle
    }

    pub fn set_first_slice_angle(&mut self, value: FirstSliceAngle) -> &mut DoughnutChart {
        self.first_slice_angle = value;
        self
    }

    pub fn get_hole_size(&self) -> &HoleSize {
        &self.hole_size
    }

    pub fn get_hole_size_mut(&mut self) -> &mut HoleSize {
        &mut self.hole_size
    }

    pub fn set_hole_size(&mut self, value: HoleSize) -> &mut DoughnutChart {
        self.hole_size = value;
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
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:varyColors" => {
                        self.vary_colors.set_attributes(reader, e);
                    }
                    b"c:firstSliceAng" => {
                        self.first_slice_angle.set_attributes(reader, e);
                    }
                    b"c:holeSize" => {
                        self.hole_size.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:doughnutChart" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:doughnutChart"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:doughnutChart
        write_start_tag(writer, "c:doughnutChart", vec![], false);

        // c:varyColors
        self.vary_colors.write_to(writer);

        // c:ser
        for v in self.area_chart_series_list.get_area_chart_series() {
            v.write_to(writer, spreadsheet);
        }

        // c:dLbls
        self.data_labels.write_to(writer);

        // c:firstSliceAng
        self.first_slice_angle.write_to(writer);

        // c:holeSize
        self.hole_size.write_to(writer);

        write_end_tag(writer, "c:doughnutChart");
    }
}
