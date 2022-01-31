use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum ChartType {
    LineChart,
    Line3DChart,
    PieChart,
    Pie3DChart,
    DoughnutChart,
    ScatterChart,
    BarChart,
    Bar3DChart,
    RadarChart,
    BubbleChart,
    AreaChart,
    Area3DChart,
    OfPieChart,
}
impl Default for ChartType {
    fn default() -> Self {
        Self::LineChart
    }
}
impl EnumTrait for ChartType {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::LineChart => "line_chart",
            Self::Line3DChart => "line_3d_chart",
            Self::PieChart => "pie_chart",
            Self::Pie3DChart => "pie_3d_chart",
            Self::DoughnutChart => "doughnut_chart",
            Self::ScatterChart => "scatter_chart",
            Self::BarChart => "bar_chart",
            Self::Bar3DChart => "bar_3d_chart",
            Self::RadarChart => "radar_chart",
            Self::BubbleChart => "bubble_chart",
            Self::AreaChart => "area_chart",
            Self::Area3DChart => "area_3d_chart",
            Self::OfPieChart => "of_pie_chart",
        }
    }
}
impl FromStr for ChartType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "line_chart" => Ok(Self::LineChart),
            "line_3d_chart" => Ok(Self::Line3DChart),
            "pie_chart" => Ok(Self::PieChart),
            "pie_3d_chart" => Ok(Self::Pie3DChart),
            "doughnut_chart" => Ok(Self::DoughnutChart),
            "scatter_chart" => Ok(Self::ScatterChart),
            "bar_chart" => Ok(Self::BarChart),
            "bar_3d_chart" => Ok(Self::Bar3DChart),
            "radar_chart" => Ok(Self::RadarChart),
            "bubble_chart" => Ok(Self::BubbleChart),
            "area_chart" => Ok(Self::AreaChart),
            "area_3d_chart" => Ok(Self::Area3DChart),
            "of_pie_chart" => Ok(Self::OfPieChart),
            _ => Err(()),
        }
    }
}
