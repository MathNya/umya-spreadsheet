// c:barChart
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use super::{
    AreaChartSeries, AreaChartSeriesList, AxisId, BarDirection, DataLabels, GapWidth, Grouping,
    Overlap, VaryColors,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::Workbook,
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct BarChart {
    bar_direction: BarDirection,
    grouping: Grouping,
    vary_colors: VaryColors,
    area_chart_series_list: AreaChartSeriesList,
    data_labels: DataLabels,
    gap_width: GapWidth,
    overlap: Overlap,
    axis_id: Vec<AxisId>,
}

impl BarChart {
    #[must_use]
    pub fn bar_direction(&self) -> &BarDirection {
        &self.bar_direction
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bar_direction()")]
    pub fn get_bar_direction(&self) -> &BarDirection {
        self.bar_direction()
    }

    pub fn bar_direction_mut(&mut self) -> &mut BarDirection {
        &mut self.bar_direction
    }

    #[deprecated(since = "3.0.0", note = "Use bar_direction_mut()")]
    pub fn get_bar_direction_mut(&mut self) -> &mut BarDirection {
        self.bar_direction_mut()
    }

    pub fn set_bar_direction(&mut self, value: BarDirection) -> &mut BarChart {
        self.bar_direction = value;
        self
    }

    #[must_use]
    pub fn grouping(&self) -> &Grouping {
        &self.grouping
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use grouping()")]
    pub fn get_grouping(&self) -> &Grouping {
        self.grouping()
    }

    pub fn grouping_mut(&mut self) -> &mut Grouping {
        &mut self.grouping
    }

    #[deprecated(since = "3.0.0", note = "Use grouping_mut()")]
    pub fn get_grouping_mut(&mut self) -> &mut Grouping {
        self.grouping_mut()
    }

    pub fn set_grouping(&mut self, value: Grouping) -> &mut BarChart {
        self.grouping = value;
        self
    }

    #[must_use]
    pub fn vary_colors(&self) -> &VaryColors {
        &self.vary_colors
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use vary_colors()")]
    pub fn get_vary_colors(&self) -> &VaryColors {
        self.vary_colors()
    }

    pub fn vary_colors_mut(&mut self) -> &mut VaryColors {
        &mut self.vary_colors
    }

    #[deprecated(since = "3.0.0", note = "Use vary_colors_mut()")]
    pub fn get_vary_colors_mut(&mut self) -> &mut VaryColors {
        self.vary_colors_mut()
    }

    pub fn set_vary_colors(&mut self, value: VaryColors) -> &mut BarChart {
        self.vary_colors = value;
        self
    }

    #[must_use]
    pub fn area_chart_series_list(&self) -> &AreaChartSeriesList {
        &self.area_chart_series_list
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use area_chart_series_list()")]
    pub fn get_area_chart_series_list(&self) -> &AreaChartSeriesList {
        self.area_chart_series_list()
    }

    pub fn area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        &mut self.area_chart_series_list
    }

    #[deprecated(since = "3.0.0", note = "Use area_chart_series_list_mut()")]
    pub fn get_area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        self.area_chart_series_list_mut()
    }

    pub fn set_area_chart_series_list(&mut self, value: AreaChartSeriesList) -> &mut Self {
        self.area_chart_series_list = value;
        self
    }

    #[must_use]
    pub fn data_labels(&self) -> &DataLabels {
        &self.data_labels
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_labels()")]
    pub fn get_data_labels(&self) -> &DataLabels {
        self.data_labels()
    }

    pub fn data_labels_mut(&mut self) -> &mut DataLabels {
        &mut self.data_labels
    }

    #[deprecated(since = "3.0.0", note = "Use data_labels_mut()")]
    pub fn get_data_labels_mut(&mut self) -> &mut DataLabels {
        self.data_labels_mut()
    }

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut BarChart {
        self.data_labels = value;
        self
    }

    #[must_use]
    pub fn gap_width(&self) -> &GapWidth {
        &self.gap_width
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use gap_width()")]
    pub fn get_gap_width(&self) -> &GapWidth {
        self.gap_width()
    }

    pub fn gap_width_mut(&mut self) -> &mut GapWidth {
        &mut self.gap_width
    }

    #[deprecated(since = "3.0.0", note = "Use gap_width_mut()")]
    pub fn get_gap_width_mut(&mut self) -> &mut GapWidth {
        self.gap_width_mut()
    }

    pub fn set_gap_width(&mut self, value: GapWidth) -> &mut BarChart {
        self.gap_width = value;
        self
    }

    #[must_use]
    pub fn overlap(&self) -> &Overlap {
        &self.overlap
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use overlap()")]
    pub fn get_overlap(&self) -> &Overlap {
        self.overlap()
    }

    pub fn overlap_mut(&mut self) -> &mut Overlap {
        &mut self.overlap
    }

    #[deprecated(since = "3.0.0", note = "Use overlap_mut()")]
    pub fn get_overlap_mut(&mut self) -> &mut Overlap {
        self.overlap_mut()
    }

    pub fn set_overlap(&mut self, value: Overlap) -> &mut BarChart {
        self.overlap = value;
        self
    }

    #[must_use]
    pub fn axis_id(&self) -> &[AxisId] {
        &self.axis_id
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use axis_id()")]
    pub fn get_axis_id(&self) -> &[AxisId] {
        self.axis_id()
    }

    pub fn axis_id_mut(&mut self) -> &mut Vec<AxisId> {
        &mut self.axis_id
    }

    #[deprecated(since = "3.0.0", note = "Use axis_id_mut()")]
    pub fn get_axis_id_mut(&mut self) -> &mut Vec<AxisId> {
        self.axis_id_mut()
    }

    pub fn set_axis_id(&mut self, value: impl Into<Vec<AxisId>>) -> &mut BarChart {
        self.axis_id = value.into();
        self
    }

    pub fn add_axis_id(&mut self, value: AxisId) -> &mut BarChart {
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
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"c:ser" => {
                        let mut obj = AreaChartSeries::default();
                        obj.set_attributes(reader, e);
                        self.area_chart_series_list_mut()
                            .add_area_chart_series(obj);
                        }
                    b"c:dLbls" => {
                        self.data_labels.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"c:barDir" => {
                        self.bar_direction.set_attributes(reader, e);
                    }
                    b"c:grouping" => {
                        self.grouping.set_attributes(reader, e);
                    }
                    b"c:varyColors" => {
                        self.vary_colors.set_attributes(reader, e);
                    }
                    b"c:gapWidth" => {
                        self.gap_width.set_attributes(reader, e);
                    }
                    b"c:overlap" => {
                        self.overlap.set_attributes(reader, e);
                    }
                    b"c:axId" => {
                        let mut obj = AxisId::default();
                        obj.set_attributes(reader, e);
                        self.add_axis_id(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:barChart" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:barChart")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:barChart
        write_start_tag(writer, "c:barChart", vec![], false);

        // c:barDir
        self.bar_direction.write_to(writer);

        // c:grouping
        self.grouping.write_to(writer);

        // c:varyColors
        self.vary_colors.write_to(writer);

        // c:ser
        for v in self.area_chart_series_list.area_chart_series() {
            v.write_to(writer, wb);
        }

        // c:dLbls
        self.data_labels.write_to(writer);

        // c:gapWidth
        self.gap_width.write_to(writer);

        // c:overlap
        self.overlap.write_to(writer);

        // c:axId
        for v in &self.axis_id {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:barChart");
    }
}
