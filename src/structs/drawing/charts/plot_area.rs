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
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
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

    pub fn get_line_chart(&self) -> &Option<LineChart> {
        &self.line_chart
    }

    pub fn get_line_chart_mut(&mut self) -> &mut Option<LineChart> {
        &mut self.line_chart
    }

    pub fn set_line_chart(&mut self, value: LineChart) -> &mut Self {
        self.line_chart = Some(value);
        self
    }

    pub fn get_line_3d_chart(&self) -> &Option<Line3DChart> {
        &self.line_3d_chart
    }

    pub fn get_line_3d_chart_mut(&mut self) -> &mut Option<Line3DChart> {
        &mut self.line_3d_chart
    }

    pub fn set_line_3d_chart(&mut self, value: Line3DChart) -> &mut Self {
        self.line_3d_chart = Some(value);
        self
    }

    pub fn get_pie_chart(&self) -> &Option<PieChart> {
        &self.pie_chart
    }

    pub fn get_pie_chart_mut(&mut self) -> &mut Option<PieChart> {
        &mut self.pie_chart
    }

    pub fn set_pie_chart(&mut self, value: PieChart) -> &mut Self {
        self.pie_chart = Some(value);
        self
    }

    pub fn get_pie_3d_chart(&self) -> &Option<Pie3DChart> {
        &self.pie_3d_chart
    }

    pub fn get_pie_3d_chart_mut(&mut self) -> &mut Option<Pie3DChart> {
        &mut self.pie_3d_chart
    }

    pub fn set_pie_3d_chart(&mut self, value: Pie3DChart) -> &mut Self {
        self.pie_3d_chart = Some(value);
        self
    }

    pub fn get_doughnut_chart(&self) -> &Option<DoughnutChart> {
        &self.doughnut_chart
    }

    pub fn get_doughnut_chart_mut(&mut self) -> &mut Option<DoughnutChart> {
        &mut self.doughnut_chart
    }

    pub fn set_doughnut_chart(&mut self, value: DoughnutChart) -> &mut Self {
        self.doughnut_chart = Some(value);
        self
    }

    pub fn get_scatter_chart(&self) -> &Option<ScatterChart> {
        &self.scatter_chart
    }

    pub fn get_scatter_chart_mut(&mut self) -> &mut Option<ScatterChart> {
        &mut self.scatter_chart
    }

    pub fn set_scatter_chart(&mut self, value: ScatterChart) -> &mut Self {
        self.scatter_chart = Some(value);
        self
    }

    pub fn get_bar_chart(&self) -> &Option<BarChart> {
        &self.bar_chart
    }

    pub fn get_bar_chart_mut(&mut self) -> &mut Option<BarChart> {
        &mut self.bar_chart
    }

    pub fn set_bar_chart(&mut self, value: BarChart) -> &mut Self {
        self.bar_chart = Some(value);
        self
    }

    pub fn get_bar_3d_chart(&self) -> &Option<Bar3DChart> {
        &self.bar_3d_chart
    }

    pub fn get_bar_3d_chart_mut(&mut self) -> &mut Option<Bar3DChart> {
        &mut self.bar_3d_chart
    }

    pub fn set_bar_3d_chart(&mut self, value: Bar3DChart) -> &mut Self {
        self.bar_3d_chart = Some(value);
        self
    }

    pub fn get_radar_chart(&self) -> &Option<RadarChart> {
        &self.radar_chart
    }

    pub fn get_radar_chart_mut(&mut self) -> &mut Option<RadarChart> {
        &mut self.radar_chart
    }

    pub fn set_radar_chart(&mut self, value: RadarChart) -> &mut Self {
        self.radar_chart = Some(value);
        self
    }

    pub fn get_bubble_chart(&self) -> &Option<BubbleChart> {
        &self.bubble_chart
    }

    pub fn get_bubble_chart_mut(&mut self) -> &mut Option<BubbleChart> {
        &mut self.bubble_chart
    }

    pub fn set_bubble_chart(&mut self, value: BubbleChart) -> &mut Self {
        self.bubble_chart = Some(value);
        self
    }

    pub fn get_area_chart(&self) -> &Option<AreaChart> {
        &self.area_chart
    }

    pub fn get_area_chart_mut(&mut self) -> &mut Option<AreaChart> {
        &mut self.area_chart
    }

    pub fn set_area_chart(&mut self, value: AreaChart) -> &mut Self {
        self.area_chart = Some(value);
        self
    }

    pub fn get_area_3d_chart(&self) -> &Option<Area3DChart> {
        &self.area_3d_chart
    }

    pub fn get_area_3d_chart_mut(&mut self) -> &mut Option<Area3DChart> {
        &mut self.area_3d_chart
    }

    pub fn set_area_3d_chart(&mut self, value: Area3DChart) -> &mut Self {
        self.area_3d_chart = Some(value);
        self
    }

    pub fn get_of_pie_chart(&self) -> &Option<OfPieChart> {
        &self.of_pie_chart
    }

    pub fn get_of_pie_chart_mut(&mut self) -> &mut Option<OfPieChart> {
        &mut self.of_pie_chart
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

    pub fn set_grouping(&mut self, value: GroupingValues) -> &mut Self {
        match &mut self.line_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        match &mut self.line_3d_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        match &mut self.bar_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        match &mut self.bar_3d_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        match &mut self.area_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        match &mut self.area_3d_chart {
            Some(chart) => {
                chart.get_grouping_mut().set_val(value);
                return self;
            }
            None => {}
        }
        panic! {"Non-Grouping."};
    }

    pub fn get_area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        match &mut self.line_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.line_3d_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.pie_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.pie_3d_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.doughnut_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.scatter_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.bar_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.bar_3d_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.radar_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.bubble_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.area_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.area_3d_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        match &mut self.of_pie_chart {
            Some(chart) => {
                return chart.get_area_chart_series_list_mut();
            }
            None => {}
        }
        panic! {"Non-ChartSeriesList."};
    }

    pub fn get_formula_mut(&mut self) -> Vec<&mut Formula> {
        let mut result: Vec<&mut Formula> = Vec::default();
        match &mut self.line_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.line_3d_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.pie_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.pie_3d_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.doughnut_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.scatter_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bar_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bar_3d_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.radar_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bubble_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.area_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.area_3d_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.of_pie_chart {
            Some(v) => {
                for ser in v
                    .get_area_chart_series_list_mut()
                    .get_area_chart_series_mut()
                {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        result
    }

    pub(crate) fn is_support(&self) -> bool {
        match &self.line_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.line_3d_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.pie_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.pie_3d_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.doughnut_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.scatter_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.bar_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.bar_3d_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.radar_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.bubble_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.area_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.area_3d_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        match &self.of_pie_chart {
            Some(_) => {
                return true;
            }
            None => {}
        }
        false
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
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:plotArea" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:plotArea"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:plotArea
        write_start_tag(writer, "c:plotArea", vec![], false);

        // c:layout
        self.layout.write_to(writer);

        // c:lineChart
        match &self.line_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:line3DChart
        match &self.line_3d_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:pieChart
        match &self.pie_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:pie3DChart
        match &self.pie_3d_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:doughnutChart
        match &self.doughnut_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:scatterChart
        match &self.scatter_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:barChart
        match &self.bar_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:bar3DChart
        match &self.bar_3d_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:radarChart
        match &self.radar_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:bubbleChart
        match &self.bubble_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:areaChart
        match &self.area_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:area3DChart
        match &self.area_3d_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
        }

        // c:ofPieChart
        match &self.of_pie_chart {
            Some(v) => {
                v.write_to(writer, spreadsheet);
            }
            None => {}
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
        match &self.shape_properties {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:plotArea");
    }
}
