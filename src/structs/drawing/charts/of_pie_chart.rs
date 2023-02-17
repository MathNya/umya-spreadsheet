// c:ofPieChart
use super::AreaChartSeries;
use super::AreaChartSeriesList;
use super::DataLabels;
use super::GapWidth;
use super::OfPieType;
use super::SecondPieSize;
use super::SeriesLines;
use super::VaryColors;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OfPieChart {
    of_pie_type: OfPieType,
    vary_colors: VaryColors,
    area_chart_series_list: AreaChartSeriesList,
    data_labels: DataLabels,
    gap_width: GapWidth,
    second_pie_size: SecondPieSize,
    series_lines: SeriesLines,
}
impl OfPieChart {
    pub fn get_of_pie_type(&self) -> &OfPieType {
        &self.of_pie_type
    }

    pub fn get_of_pie_type_mut(&mut self) -> &mut OfPieType {
        &mut self.of_pie_type
    }

    pub fn set_of_pie_type(&mut self, value: OfPieType) -> &mut OfPieChart {
        self.of_pie_type = value;
        self
    }

    pub fn get_vary_colors(&self) -> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self) -> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value: VaryColors) -> &mut OfPieChart {
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

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut OfPieChart {
        self.data_labels = value;
        self
    }

    pub fn get_gap_width(&self) -> &GapWidth {
        &self.gap_width
    }

    pub fn get_gap_width_mut(&mut self) -> &mut GapWidth {
        &mut self.gap_width
    }

    pub fn set_gap_width(&mut self, value: GapWidth) -> &mut OfPieChart {
        self.gap_width = value;
        self
    }

    pub fn get_second_pie_size(&self) -> &SecondPieSize {
        &self.second_pie_size
    }

    pub fn get_second_pie_size_mut(&mut self) -> &mut SecondPieSize {
        &mut self.second_pie_size
    }

    pub fn set_second_pie_size(&mut self, value: SecondPieSize) -> &mut OfPieChart {
        self.second_pie_size = value;
        self
    }

    pub fn get_series_lines(&self) -> &SeriesLines {
        &self.series_lines
    }

    pub fn get_series_lines_mut(&mut self) -> &mut SeriesLines {
        &mut self.series_lines
    }

    pub fn set_series_lines(&mut self, value: SeriesLines) -> &mut OfPieChart {
        self.series_lines = value;
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
                    b"c:ser" => {
                        let mut obj = AreaChartSeries::default();
                        obj.set_attributes(reader, e);
                        self.get_area_chart_series_list_mut()
                            .add_area_chart_series(obj);
                    }
                    b"c:dLbls" => {
                        self.data_labels.set_attributes(reader, e);
                    }
                    b"c:serLines" => {
                        self.series_lines.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"c:ofPieType" => {
                        self.of_pie_type.set_attributes(reader, e);
                    }
                    b"c:varyColors" => {
                        self.vary_colors.set_attributes(reader, e);
                    }
                    b"c:gapWidth" => {
                        self.gap_width.set_attributes(reader, e);
                    }
                    b"c:secondPieSize" => {
                        self.second_pie_size.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:ofPieChart" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:ofPieChart"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:ofPieChart
        write_start_tag(writer, "c:ofPieChart", vec![], false);

        // c:ofPieType
        self.of_pie_type.write_to(writer);

        // c:varyColors
        self.vary_colors.write_to(writer);

        // c:ser
        for v in self.area_chart_series_list.get_area_chart_series() {
            v.write_to(writer, spreadsheet);
        }

        // c:dLbls
        self.data_labels.write_to(writer);

        // c:gapWidth
        self.gap_width.write_to(writer);

        // c:secondPieSize
        self.second_pie_size.write_to(writer);

        // c:serLines
        self.series_lines.write_to(writer);

        write_end_tag(writer, "c:ofPieChart");
    }
}
