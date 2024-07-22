// c:plotArea
use super::Area3DChart;
use super::AreaChart;
use super::AreaChartSeriesList;
use super::Bar3DChart;
use super::BarChart;
use super::BubbleChart;
use super::CategoryAxis;
use super::DoughnutChart;
use super::Formula;
use super::GroupingValues;
use super::Layout;
use super::Line3DChart;
use super::LineChart;
use super::OfPieChart;
use super::Pie3DChart;
use super::PieChart;
use super::RadarChart;
use super::ScatterChart;
use super::SeriesAxis;
use super::ShapeProperties;
use super::ValueAxis;
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PlotArea {
    layout: Layout,
    line_chart: Option<LineChart>,
    line_3d_chart: Option<Line3DChart>,
    pie_chart: Option<PieChart>,
    pie_3d_chart: Option<Pie3DChart>,
    doughnut_chart: Option<DoughnutChart>,
    scatter_chart: Option<ScatterChart>,
    bar_chart: Option<BarChart>,
    bar_3d_chart: Option<Bar3DChart>,
    radar_chart: Option<RadarChart>,
    bubble_chart: Option<BubbleChart>,
    area_chart: Option<AreaChart>,
    area_3d_chart: Option<Area3DChart>,
    of_pie_chart: Option<OfPieChart>,
    category_axis: Vec<CategoryAxis>,
    value_axis: Vec<ValueAxis>,
    series_axis: Vec<SeriesAxis>,
    shape_properties: Option<ShapeProperties>,
}

impl PlotArea {
    pub fn get_layout(&self) -> &Layout {
        &self.layout
    }

    pub fn get_layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    pub fn set_layout(&mut self, value: Layout) -> &mut Self {
        self.layout = value;
        self
    }

    pub fn get_line_chart(&self) -> Option<&LineChart> {
        self.line_chart.as_ref()
    }

    pub fn get_line_chart_mut(&mut self) -> Option<&mut LineChart> {
        self.line_chart.as_mut()
    }

    pub fn set_line_chart(&mut self, value: LineChart) -> &mut Self {
        self.line_chart = Some(value);
        self
    }

    pub fn get_line_3d_chart(&self) -> Option<&Line3DChart> {
        self.line_3d_chart.as_ref()
    }

    pub fn get_line_3d_chart_mut(&mut self) -> Option<&mut Line3DChart> {
        self.line_3d_chart.as_mut()
    }

    pub fn set_line_3d_chart(&mut self, value: Line3DChart) -> &mut Self {
        self.line_3d_chart = Some(value);
        self
    }

    pub fn get_pie_chart(&self) -> Option<&PieChart> {
        self.pie_chart.as_ref()
    }

    pub fn get_pie_chart_mut(&mut self) -> Option<&mut PieChart> {
        self.pie_chart.as_mut()
    }

    pub fn set_pie_chart(&mut self, value: PieChart) -> &mut Self {
        self.pie_chart = Some(value);
        self
    }

    pub fn get_pie_3d_chart(&self) -> Option<&Pie3DChart> {
        self.pie_3d_chart.as_ref()
    }

    pub fn get_pie_3d_chart_mut(&mut self) -> Option<&mut Pie3DChart> {
        self.pie_3d_chart.as_mut()
    }

    pub fn set_pie_3d_chart(&mut self, value: Pie3DChart) -> &mut Self {
        self.pie_3d_chart = Some(value);
        self
    }

    pub fn get_doughnut_chart(&self) -> Option<&DoughnutChart> {
        self.doughnut_chart.as_ref()
    }

    pub fn get_doughnut_chart_mut(&mut self) -> Option<&mut DoughnutChart> {
        self.doughnut_chart.as_mut()
    }

    pub fn set_doughnut_chart(&mut self, value: DoughnutChart) -> &mut Self {
        self.doughnut_chart = Some(value);
        self
    }

    pub fn get_scatter_chart(&self) -> Option<&ScatterChart> {
        self.scatter_chart.as_ref()
    }

    pub fn get_scatter_chart_mut(&mut self) -> Option<&mut ScatterChart> {
        self.scatter_chart.as_mut()
    }

    pub fn set_scatter_chart(&mut self, value: ScatterChart) -> &mut Self {
        self.scatter_chart = Some(value);
        self
    }

    pub fn get_bar_chart(&self) -> Option<&BarChart> {
        self.bar_chart.as_ref()
    }

    pub fn get_bar_chart_mut(&mut self) -> Option<&mut BarChart> {
        self.bar_chart.as_mut()
    }

    pub fn set_bar_chart(&mut self, value: BarChart) -> &mut Self {
        self.bar_chart = Some(value);
        self
    }

    pub fn get_bar_3d_chart(&self) -> Option<&Bar3DChart> {
        self.bar_3d_chart.as_ref()
    }

    pub fn get_bar_3d_chart_mut(&mut self) -> Option<&mut Bar3DChart> {
        self.bar_3d_chart.as_mut()
    }

    pub fn set_bar_3d_chart(&mut self, value: Bar3DChart) -> &mut Self {
        self.bar_3d_chart = Some(value);
        self
    }

    pub fn get_radar_chart(&self) -> Option<&RadarChart> {
        self.radar_chart.as_ref()
    }

    pub fn get_radar_chart_mut(&mut self) -> Option<&mut RadarChart> {
        self.radar_chart.as_mut()
    }

    pub fn set_radar_chart(&mut self, value: RadarChart) -> &mut Self {
        self.radar_chart = Some(value);
        self
    }

    pub fn get_bubble_chart(&self) -> Option<&BubbleChart> {
        self.bubble_chart.as_ref()
    }

    pub fn get_bubble_chart_mut(&mut self) -> Option<&mut BubbleChart> {
        self.bubble_chart.as_mut()
    }

    pub fn set_bubble_chart(&mut self, value: BubbleChart) -> &mut Self {
        self.bubble_chart = Some(value);
        self
    }

    pub fn get_area_chart(&self) -> Option<&AreaChart> {
        self.area_chart.as_ref()
    }

    pub fn get_area_chart_mut(&mut self) -> Option<&mut AreaChart> {
        self.area_chart.as_mut()
    }

    pub fn set_area_chart(&mut self, value: AreaChart) -> &mut Self {
        self.area_chart = Some(value);
        self
    }

    pub fn get_area_3d_chart(&self) -> Option<&Area3DChart> {
        self.area_3d_chart.as_ref()
    }

    pub fn get_area_3d_chart_mut(&mut self) -> Option<&mut Area3DChart> {
        self.area_3d_chart.as_mut()
    }

    pub fn set_area_3d_chart(&mut self, value: Area3DChart) -> &mut Self {
        self.area_3d_chart = Some(value);
        self
    }

    pub fn get_of_pie_chart(&self) -> Option<&OfPieChart> {
        self.of_pie_chart.as_ref()
    }

    pub fn get_of_pie_chart_mut(&mut self) -> Option<&mut OfPieChart> {
        self.of_pie_chart.as_mut()
    }

    pub fn set_of_pie_chart(&mut self, value: OfPieChart) -> &mut Self {
        self.of_pie_chart = Some(value);
        self
    }

    pub fn get_category_axis(&self) -> &Vec<CategoryAxis> {
        &self.category_axis
    }

    pub fn get_category_axis_mut(&mut self) -> &mut Vec<CategoryAxis> {
        &mut self.category_axis
    }

    pub fn set_category_axis(&mut self, value: Vec<CategoryAxis>) -> &mut Self {
        self.category_axis = value;
        self
    }

    pub fn add_category_axis(&mut self, value: CategoryAxis) -> &mut Self {
        self.category_axis.push(value);
        self
    }

    pub fn get_value_axis(&self) -> &Vec<ValueAxis> {
        &self.value_axis
    }

    pub fn get_value_axis_mut(&mut self) -> &mut Vec<ValueAxis> {
        &mut self.value_axis
    }

    pub fn set_value_axis(&mut self, value: Vec<ValueAxis>) -> &mut Self {
        self.value_axis = value;
        self
    }

    pub fn add_value_axis(&mut self, value: ValueAxis) -> &mut Self {
        self.value_axis.push(value);
        self
    }

    pub fn get_series_axis(&self) -> &Vec<SeriesAxis> {
        &self.series_axis
    }

    pub fn get_series_axis_mut(&mut self) -> &mut Vec<SeriesAxis> {
        &mut self.series_axis
    }

    pub fn set_series_axis(&mut self, value: Vec<SeriesAxis>) -> &mut Self {
        self.series_axis = value;
        self
    }

    pub fn add_series_axis(&mut self, value: SeriesAxis) -> &mut Self {
        self.series_axis.push(value);
        self
    }

    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_ref()
    }

    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    pub fn set_grouping(&mut self, value: GroupingValues) -> &mut Self {
        if let Some(chart) = &mut self.line_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        if let Some(chart) = &mut self.line_3d_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        if let Some(chart) = &mut self.bar_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        if let Some(chart) = &mut self.bar_3d_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        if let Some(chart) = &mut self.area_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        if let Some(chart) = &mut self.area_3d_chart {
            chart.get_grouping_mut().set_val(value);
            return self;
        }
        panic! {"Non-Grouping."};
    }

    pub fn get_area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        if let Some(chart) = &mut self.line_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.line_3d_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.pie_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.pie_3d_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.doughnut_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.scatter_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.bar_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.bar_3d_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.radar_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.bubble_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.area_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.area_3d_chart {
            return chart.get_area_chart_series_list_mut();
        }
        if let Some(chart) = &mut self.of_pie_chart {
            return chart.get_area_chart_series_list_mut();
        }
        panic! {"Non-ChartSeriesList."};
    }

    pub fn get_formula_mut(&mut self) -> Vec<&mut Formula> {
        let mut result: Vec<&mut Formula> = Vec::default();
        if let Some(v) = &mut self.line_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.line_3d_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.pie_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.pie_3d_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.doughnut_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.scatter_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.bar_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.bar_3d_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.radar_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.bubble_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.area_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.area_3d_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        if let Some(v) = &mut self.of_pie_chart {
            for ser in v
                .get_area_chart_series_list_mut()
                .get_area_chart_series_mut()
            {
                for formula in ser.get_formula_mut() {
                    result.push(formula);
                }
            }
        }
        result
    }

    pub(crate) fn is_support(&self) -> bool {
        self.line_chart.is_some()
            || self.line_3d_chart.is_some()
            || self.pie_chart.is_some()
            || self.pie_3d_chart.is_some()
            || self.doughnut_chart.is_some()
            || self.scatter_chart.is_some()
            || self.bar_chart.is_some()
            || self.bar_3d_chart.is_some()
            || self.radar_chart.is_some()
            || self.bubble_chart.is_some()
            || self.area_chart.is_some()
            || self.area_3d_chart.is_some()
            || self.of_pie_chart.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().0 {
                b"c:layout" => {
                    self.layout.set_attributes(reader, e, false);
                }
                b"c:lineChart" => {
                    let mut obj = LineChart::default();
                    obj.set_attributes(reader, e);
                    self.set_line_chart(obj);
                }
                b"c:line3DChart" => {
                    let mut obj = Line3DChart::default();
                    obj.set_attributes(reader, e);
                    self.set_line_3d_chart(obj);
                }
                b"c:pieChart" => {
                    let mut obj = PieChart::default();
                    obj.set_attributes(reader, e);
                    self.set_pie_chart(obj);
                }
                b"c:pie3DChart" => {
                    let mut obj = Pie3DChart::default();
                    obj.set_attributes(reader, e);
                    self.set_pie_3d_chart(obj);
                }
                b"c:doughnutChart" => {
                    let mut obj = DoughnutChart::default();
                    obj.set_attributes(reader, e);
                    self.set_doughnut_chart(obj);
                }
                b"c:scatterChart" => {
                    let mut obj = ScatterChart::default();
                    obj.set_attributes(reader, e);
                    self.set_scatter_chart(obj);
                }
                b"c:barChart" => {
                    let mut obj = BarChart::default();
                    obj.set_attributes(reader, e);
                    self.set_bar_chart(obj);
                }
                b"c:bar3DChart" => {
                    let mut obj = Bar3DChart::default();
                    obj.set_attributes(reader, e);
                    self.set_bar_3d_chart(obj);
                }
                b"c:radarChart" => {
                    let mut obj = RadarChart::default();
                    obj.set_attributes(reader, e);
                    self.set_radar_chart(obj);
                }
                b"c:bubbleChart" => {
                    let mut obj = BubbleChart::default();
                    obj.set_attributes(reader, e);
                    self.set_bubble_chart(obj);
                }
                b"c:areaChart" => {
                    let mut obj = AreaChart::default();
                    obj.set_attributes(reader, e);
                    self.set_area_chart(obj);
                }
                b"c:area3DChart" => {
                    let mut obj = Area3DChart::default();
                    obj.set_attributes(reader, e);
                    self.set_area_3d_chart(obj);
                }
                b"c:ofPieChart" => {
                    let mut obj = OfPieChart::default();
                    obj.set_attributes(reader, e);
                    self.set_of_pie_chart(obj);
                }
                b"c:catAx" => {
                    let mut obj = CategoryAxis::default();
                    obj.set_attributes(reader, e);
                    self.add_category_axis(obj);
                }
                b"c:valAx" => {
                    let mut obj = ValueAxis::default();
                    obj.set_attributes(reader, e);
                    self.add_value_axis(obj);
                }
                b"c:serAx" => {
                    let mut obj = SeriesAxis::default();
                    obj.set_attributes(reader, e);
                    self.add_series_axis(obj);
                }
                b"c:spPr" => {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:plotArea" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:plotArea"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:plotArea
        write_start_tag(writer, "c:plotArea", vec![], false);

        // c:layout
        self.layout.write_to(writer);

        // c:lineChart
        if let Some(v) = &self.line_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:line3DChart
        if let Some(v) = &self.line_3d_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:pieChart
        if let Some(v) = &self.pie_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:pie3DChart
        if let Some(v) = &self.pie_3d_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:doughnutChart
        if let Some(v) = &self.doughnut_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:scatterChart
        if let Some(v) = &self.scatter_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:barChart
        if let Some(v) = &self.bar_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:bar3DChart
        if let Some(v) = &self.bar_3d_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:radarChart
        if let Some(v) = &self.radar_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:bubbleChart
        if let Some(v) = &self.bubble_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:areaChart
        if let Some(v) = &self.area_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:area3DChart
        if let Some(v) = &self.area_3d_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:ofPieChart
        if let Some(v) = &self.of_pie_chart {
            v.write_to(writer, spreadsheet);
        }

        // c:catAx
        for v in &self.category_axis {
            v.write_to(writer);
        }

        // c:valAx
        for v in &self.value_axis {
            v.write_to(writer);
        }

        // c:serAx
        for v in &self.series_axis {
            v.write_to(writer);
        }

        // c:spPr
        if let Some(v) = &self.shape_properties {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:plotArea");
    }
}
impl AdjustmentCoordinateWithSheet for PlotArea {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for formula in self.get_formula_mut() {
            formula.adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for formula in self.get_formula_mut() {
            formula.adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
