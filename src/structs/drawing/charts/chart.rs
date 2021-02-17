use super::title::Title;
use super::legend::Legend;
use super::plot_area::PlotArea;
use super::axis::Axis;
use super::grid_lines::GridLines;
use super::data_series::DataSeries;
use super::super::super::anchor::Anchor;
use super::super::transform2d::Transform2D;
use super::super::spreadsheet::non_visual_drawing_properties::NonVisualDrawingProperties;

#[derive(Debug)]
pub struct Chart {
    non_visual_drawing_properties: NonVisualDrawingProperties,
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
    anchor: Anchor,
    transform: Transform2D,
}
impl Default for Chart {
    fn default() -> Self {
        Self {
            non_visual_drawing_properties: NonVisualDrawingProperties::default(),
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
            anchor: Anchor::default(),
            transform: Transform2D::default(),
        }
    }
}
impl Chart {
    
    pub const LANG_EN_GB: &'static str = "en_GB";
    pub const LANG_JA_JP: &'static str = "ja-JP";

    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value:NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }

    pub fn get_name(&self) -> &str {
        return &self.non_visual_drawing_properties.get_name();
    }
    pub fn set_name<S: Into<String>>(&mut self, value:S) {
        self.non_visual_drawing_properties.set_name(value.into());
    }

    pub fn get_title(&self) -> &Option<Title> {
        return &self.title;
    }
    pub fn set_title(&mut self, value:Title) {
        self.title = Some(value);
    }
    pub fn get_lang(&self) -> &str {
        return &self.lang;
    }
    pub fn set_lang<S: Into<String>>(&mut self, value:S) {
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
    pub(crate) fn set_anchor(&mut self, value:Anchor) {
        self.anchor = value;
    }
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }
    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_transform(&mut self, value:Transform2D) {
        self.transform = value;
    }

    pub fn get_transform(&self) -> &Transform2D {
        &self.transform
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }    
}