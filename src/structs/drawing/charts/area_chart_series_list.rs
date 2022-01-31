use super::AreaChartSeries;

#[derive(Clone, Default, Debug)]
pub struct AreaChartSeriesList {
    area_chart_series: Vec<AreaChartSeries>,
}
impl AreaChartSeriesList {
    pub fn get_area_chart_series(&self) -> &Vec<AreaChartSeries> {
        &self.area_chart_series
    }

    pub fn get_area_chart_series_mut(&mut self) -> &mut Vec<AreaChartSeries> {
        &mut self.area_chart_series
    }

    pub fn set_area_chart_series(&mut self, value: Vec<AreaChartSeries>) -> &mut Self {
        self.area_chart_series = value;
        self
    }

    pub fn add_area_chart_series(&mut self, value: AreaChartSeries) -> &mut Self {
        self.area_chart_series.push(value);
        self
    }
}
