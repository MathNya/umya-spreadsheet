use super::Layout;
use super::DataSeries;

#[derive(Default, Debug)]
pub struct PlotArea {
    layout: Layout,
    plot_series: Vec<DataSeries>,
}
impl PlotArea {
    pub fn get_layout(&self)-> &Layout {
        &self.layout
    }

    pub fn set_layout(&mut self, value:Layout)-> &mut PlotArea {
        self.layout = value;
        self
    }

    pub fn get_plot_series(&self)-> &Vec<DataSeries> {
        &self.plot_series
    }

    pub fn get_plot_series_mut(&mut self)-> &mut Vec<DataSeries> {
        &mut self.plot_series
    }

    pub fn set_plot_series(&mut self, value:Vec<DataSeries>)-> &mut PlotArea {
        self.plot_series = value;
        self
    }

    pub fn add_plot_series(&mut self, value:DataSeries)-> &mut PlotArea {
        self.plot_series.push(value);
        self
    }
}
