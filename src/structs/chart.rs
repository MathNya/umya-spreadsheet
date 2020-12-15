use super::title::Title;
use super::legend::Legend;
use super::plot_area::PlotArea;
use super::axis::Axis;
use super::grid_lines::GridLines;
use super::data_series::DataSeries;

#[derive(Debug)]
pub struct Chart {
    name: String,
    title: Option<Title>,
    lang: String,
    legend: Legend,
    plot_area: PlotArea,
    plot_visible_only: bool,
    display_blanks_as: String,
    y_axis: Axis,
    x_axis: Axis,
    major_gridlines: GridLines,
    minor_gridlines: GridLines,
    top_left_cell: String,
    top_left_x_offset: usize,
    top_left_y_offset: usize,
    bottom_right_cell: String,
    bottom_right_x_offset: usize,
    bottom_right_y_offset: usize,
}
impl Default for Chart {
    fn default() -> Self {
        Self {
            name: String::from(""),
            title: None,
            lang: Self::LANG_EN_GB.into(),
            legend: Legend::default(),
            plot_area: PlotArea::default(),
            plot_visible_only: false,
            display_blanks_as: DataSeries::EMPTY_AS_GAP.to_string(),
            y_axis: Axis::default(),
            x_axis: Axis::default(),
            major_gridlines: GridLines::default(),
            minor_gridlines: GridLines::default(),
            top_left_cell: String::from(""),
            top_left_x_offset: 0,
            top_left_y_offset: 0,
            bottom_right_cell: String::from(""),
            bottom_right_x_offset: 0,
            bottom_right_y_offset: 0,
        }
    }
}
impl Chart {
    
    pub const LANG_EN_GB: &'static str = "en_GB";
    pub const LANG_JA_JP: &'static str = "ja-JP";

    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub(crate) fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }
    pub fn get_title(&self) -> &Option<Title> {
        return &self.title;
    }
    pub(crate) fn set_title(&mut self, value:Title) {
        self.title = Some(value);
    }
    pub fn get_lang(&self) -> &str {
        return &self.lang;
    }
    pub(crate) fn set_lang<S: Into<String>>(&mut self, value:S) {
        self.lang = value.into();
    }
    pub fn get_legend(&self) -> &Legend {
        return &self.legend;
    }
    pub(crate) fn set_legend(&mut self, value:Legend) {
        self.legend = value;
    }
    pub fn get_plot_area(&self) -> &PlotArea {
        &self.plot_area
    }
    pub fn get_plot_area_mut(&mut self) -> &mut PlotArea {
        &mut self.plot_area
    }
    pub(crate) fn set_plot_area(&mut self, value:PlotArea) {
        self.plot_area = value;
    }
    pub fn get_plot_visible_only(&self) -> &bool {
        return &self.plot_visible_only;
    }
    pub(crate) fn set_plot_visible_only(&mut self, value:bool) {
        self.plot_visible_only = value;
    }
    pub fn get_display_blanks_as(&self) -> &str {
        return &self.display_blanks_as;
    }
    pub(crate) fn set_display_blanks_as<S: Into<String>>(&mut self, value:S) {
        self.display_blanks_as = value.into();
    }
    pub fn get_chart_axis_y(&self) -> &Axis {
        return &self.y_axis;
    }
    pub(crate) fn set_chart_axis_y(&mut self, value:Axis) {
        self.y_axis = value;
    }
    pub fn get_chart_axis_x(&self) -> &Axis {
        return &self.x_axis;
    }
    pub(crate) fn set_chart_axis_x(&mut self, value:Axis) {
        self.x_axis = value;
    }
    pub fn get_major_gridlines(&self) -> &GridLines {
        return &self.major_gridlines;
    }
    pub fn get_minor_gridlines(&self) -> &GridLines {
        return &self.minor_gridlines;
    }
    pub(crate) fn set_top_left_position(&mut self, value:String, x_offset:Option<usize>, y_offset:Option<usize>) {
        self.top_left_cell = value;
        self.set_top_left_offset(x_offset, y_offset);
    }
    pub fn get_top_left_cell(&self) -> &String {
        &self.top_left_cell
    }
    pub(crate) fn set_top_left_cell(&mut self, value:String) {
        self.top_left_cell = value;
    }
    pub(crate) fn set_top_left_offset(&mut self, x_offset:Option<usize>, y_offset:Option<usize>) {
        if x_offset.is_some() {
            self.set_top_left_x_offset(x_offset.unwrap());
        }
        if y_offset.is_some() {
            self.set_top_left_y_offset(y_offset.unwrap());
        }
    }
    pub fn get_top_left_x_offset(&self) -> &usize {
        &self.top_left_x_offset
    }
    pub(crate) fn set_top_left_x_offset(&mut self, value:usize) {
        self.top_left_x_offset = value;
    }
    pub fn get_top_left_y_offset(&self) -> &usize {
        &self.top_left_y_offset
    }
    pub(crate) fn set_top_left_y_offset(&mut self, value:usize) {
        self.top_left_y_offset = value;
    }
    pub(crate) fn set_bottom_right_position(&mut self, value:String, x_offset:Option<usize>, y_offset:Option<usize>) {
        self.bottom_right_cell = value;
        self.set_bottom_right_offset(x_offset, y_offset);
    }
    pub fn get_bottom_right_cell(&self) -> &str {
        &self.bottom_right_cell
    }
    pub(crate) fn set_bottom_right_cell(&mut self, value:String) {
        self.bottom_right_cell = value;
    }
    pub(crate) fn set_bottom_right_offset(&mut self, x_offset:Option<usize>, y_offset:Option<usize>) {
        if x_offset.is_some() {
            self.set_bottom_right_x_offset(x_offset.unwrap());
        }
        if y_offset.is_some() {
            self.set_bottom_right_y_offset(y_offset.unwrap());
        }
    }
    pub fn get_bottom_right_x_offset(&self) -> &usize {
        &self.bottom_right_x_offset
    }
    pub(crate) fn set_bottom_right_x_offset(&mut self, value:usize) {
        self.bottom_right_x_offset = value;
    }
    pub fn get_bottom_right_y_offset(&self) -> &usize {
        &self.bottom_right_y_offset
    }
    pub fn set_bottom_right_y_offset(&mut self, value:usize)
    {
        self.bottom_right_y_offset = value;
    }
}