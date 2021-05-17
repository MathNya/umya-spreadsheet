use super::data_series_values::DataSeriesValues;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct DataSeries {
    plot_type: String,
    plot_grouping: Option<String>,
    plot_direction: String,
    plot_style: String,
    plot_order: BTreeMap<i32, i32>,
    plot_label: BTreeMap<i32, DataSeriesValues>,
    plot_category: BTreeMap<i32, DataSeriesValues>,
    smooth_line: bool,
    plot_values: BTreeMap<i32, DataSeriesValues>,
}
impl DataSeries {
    pub const TYPE_BARCHART: &'static str = "barChart";
    pub const TYPE_BARCHART_3D: &'static str = "bar3DChart";
    pub const TYPE_LINECHART: &'static str = "lineChart";
    pub const TYPE_LINECHART_3D: &'static str = "line3DChart";
    pub const TYPE_AREACHART: &'static str = "areaChart";
    pub const TYPE_AREACHART_3D: &'static str = "area3DChart";
    pub const TYPE_PIECHART: &'static str = "pieChart";
    pub const TYPE_PIECHART_3D: &'static str = "pie3DChart";
    pub const TYPE_DOUGHNUTCHART: &'static str = "doughnutChart";
    pub const TYPE_DONUTCHART: &'static str = Self::TYPE_DOUGHNUTCHART; // Synonym
    pub const TYPE_SCATTERCHART: &'static str = "scatterChart";
    pub const TYPE_SURFACECHART: &'static str = "surfaceChart";
    pub const TYPE_SURFACECHART_3D: &'static str = "surface3DChart";
    pub const TYPE_RADARCHART: &'static str = "radarChart";
    pub const TYPE_BUBBLECHART: &'static str = "bubbleChart";
    pub const TYPE_STOCKCHART: &'static str = "stockChart";
    pub const TYPE_CANDLECHART: &'static str = Self::TYPE_STOCKCHART; // Synonym

    pub const GROUPING_CLUSTERED: &'static str = "clustered";
    pub const GROUPING_STACKED: &'static str = "stacked";
    pub const GROUPING_PERCENT_STACKED: &'static str = "percentStacked";
    pub const GROUPING_STANDARD: &'static str = "standard";

    pub const DIRECTION_BAR: &'static str = "bar";
    pub const DIRECTION_HORIZONTAL: &'static str = Self::DIRECTION_BAR;
    pub const DIRECTION_COL: &'static str = "col";
    pub const DIRECTION_COLUMN: &'static str = Self::DIRECTION_COL;
    pub const DIRECTION_VERTICAL: &'static str = Self::DIRECTION_COL;

    pub const STYLE_LINEMARKER: &'static str = "lineMarker";
    pub const STYLE_SMOOTHMARKER: &'static str = "smoothMarker";
    pub const STYLE_MARKER: &'static str = "marker";
    pub const STYLE_FILLED: &'static str = "filled";

    pub const EMPTY_AS_GAP: &'static str = "gap";
    pub const EMPTY_AS_ZERO: &'static str = "zero";
    pub const EMPTY_AS_SPAN: &'static str = "span";

    pub fn get_plot_type(&self)-> &str {
        &self.plot_type
    }

    pub fn set_plot_type<S: Into<String>>(&mut self, value:S)-> &mut DataSeries {
        self.plot_type = value.into();
        self
    }

    pub fn get_plot_grouping(&self)-> &Option<String> {
        &self.plot_grouping
    }

    pub fn set_plot_grouping<S: Into<String>>(&mut self, value:S)-> &mut DataSeries {
        self.plot_grouping = Some(value.into());
        self
    }

    pub fn get_plot_direction(&self)-> &str {
        &self.plot_direction
    }

    pub fn set_plot_direction<S: Into<String>>(&mut self, value:S)-> &mut DataSeries {
        self.plot_direction = value.into();
        self
    }

    pub fn get_plot_style(&self)-> &str {
        &self.plot_style
    }

    pub fn set_plot_style<S: Into<String>>(&mut self, value:S)-> &mut DataSeries {
        self.plot_style = value.into();
        self
    }

    pub fn get_plot_order(&self)-> &BTreeMap<i32, i32> {
        &self.plot_order
    }

    pub fn add_plot_order(&mut self, index:i32, value:i32)-> &mut DataSeries {
        self.plot_order.insert(index, value);
        self
    }

    pub fn get_plot_label(&self)-> &BTreeMap<i32, DataSeriesValues>
    {
        &self.plot_label
    }

    pub fn get_plot_label_mut(&mut self)-> &mut BTreeMap<i32, DataSeriesValues>
    {
        &mut self.plot_label
    }

    pub fn add_plot_label(&mut self, index:i32, value:DataSeriesValues)-> &mut DataSeries {
        self.plot_label.insert(index, value);
        self
    }

    pub fn get_plot_category(&self)-> &BTreeMap<i32, DataSeriesValues> {
        &self.plot_category
    }

    pub fn get_plot_category_mut(&mut self)-> &mut BTreeMap<i32, DataSeriesValues> {
        &mut self.plot_category
    }

    pub(crate) fn add_plot_category(&mut self, index:i32, value:DataSeriesValues)-> &mut DataSeries {
        self.plot_category.insert(index, value);
        self
    }

    pub fn get_smooth_line(&self)-> &bool {
        &self.smooth_line
    }

    pub fn set_smooth_line(&mut self, value:bool)-> &mut DataSeries {
        self.smooth_line = value.into();
        self
    }

    pub fn get_plot_values(&self)-> &BTreeMap<i32, DataSeriesValues> {
        &self.plot_values
    }

    pub fn get_plot_values_mut(&mut self)-> &mut BTreeMap<i32, DataSeriesValues> {
        &mut self.plot_values
    }

    pub fn add_plot_values(&mut self, index:i32, value:DataSeriesValues)-> &mut DataSeries {
        self.plot_values.insert(index, value);
        self
    }
}