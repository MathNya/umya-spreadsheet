use super::layout::Layout;
use super::data_series::DataSeries;

#[derive(Default, Debug)]
pub struct PlotArea {
    layout: Layout,
    plot_series: Vec<DataSeries>,
}
impl PlotArea {
    pub fn get_layout(&self)-> &Layout {
        &self.layout
    }
    pub(crate) fn set_layout(&mut self, value:Layout) {
        self.layout = value;
    }
    pub fn get_plot_series(&self)-> &Vec<DataSeries> {
        &self.plot_series
    }
    pub fn get_plot_series_mut(&mut self)-> &mut Vec<DataSeries> {
        &mut self.plot_series
    }
    pub(crate) fn set_plot_series(&mut self, value:Vec<DataSeries>) {
        self.plot_series = value;
    }
    pub(crate) fn add_plot_series(&mut self, value:DataSeries) {
        self.plot_series.push(value);
    }
}
