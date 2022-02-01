use structs::drawing::charts::Area3DChart;
use structs::drawing::charts::AreaChart;
use structs::drawing::charts::AreaChartSeries;
use structs::drawing::charts::AreaChartSeriesList;
use structs::drawing::charts::AxisId;
use structs::drawing::charts::AxisPositionValues;
use structs::drawing::charts::Bar3DChart;
use structs::drawing::charts::BarChart;
use structs::drawing::charts::BarDirectionValues;
use structs::drawing::charts::Bubble3D;
use structs::drawing::charts::BubbleChart;
use structs::drawing::charts::BubbleSize;
use structs::drawing::charts::CategoryAxis;
use structs::drawing::charts::CategoryAxisData;
use structs::drawing::charts::ChartSpace;
use structs::drawing::charts::CrossBetweenValues;
use structs::drawing::charts::DisplayBlanksAsValues;
use structs::drawing::charts::DoughnutChart;
use structs::drawing::charts::GroupingValues;
use structs::drawing::charts::InvertIfNegative;
use structs::drawing::charts::Layout;
use structs::drawing::charts::LegendPositionValues;
use structs::drawing::charts::Line3DChart;
use structs::drawing::charts::LineChart;
use structs::drawing::charts::MajorGridlines;
use structs::drawing::charts::OfPieChart;
use structs::drawing::charts::OfPieValues;
use structs::drawing::charts::OrientationValues;
use structs::drawing::charts::Pie3DChart;
use structs::drawing::charts::PieChart;
use structs::drawing::charts::RadarChart;
use structs::drawing::charts::RightAngleAxes;
use structs::drawing::charts::RotateX;
use structs::drawing::charts::RotateY;
use structs::drawing::charts::ScatterChart;
use structs::drawing::charts::ShapeValues;
use structs::drawing::charts::ShowLeaderLines;
use structs::drawing::charts::Smooth;
use structs::drawing::charts::TextProperties;
use structs::drawing::charts::TickLabelPositionValues;
use structs::drawing::charts::TickMarkValues;
use structs::drawing::charts::ValueAxis;
use structs::drawing::charts::Values;
use structs::drawing::charts::View3D;
use structs::drawing::charts::XValues;
use structs::drawing::charts::YValues;
use structs::drawing::spreadsheet::GraphicFrame;
use structs::drawing::spreadsheet::MarkerType;
use structs::drawing::spreadsheet::TwoCellAnchor;
use structs::drawing::DefaultRunProperties;
use structs::drawing::EndParagraphRunProperties;
use structs::drawing::Paragraph;
use structs::drawing::TextCharacterPropertiesType;
use structs::ChartType;

#[derive(Clone, Debug)]
pub struct Chart {
    two_cell_anchor: TwoCellAnchor,
    default_language: String,
}
impl Default for Chart {
    fn default() -> Self {
        Self {
            two_cell_anchor: TwoCellAnchor::default(),
            default_language: "en-GB".into(),
        }
    }
}
impl Chart {
    pub fn get_two_cell_anchor(&self) -> &TwoCellAnchor {
        &self.two_cell_anchor
    }

    pub fn get_two_cell_anchor_mut(&mut self) -> &mut TwoCellAnchor {
        &mut self.two_cell_anchor
    }

    pub fn set_two_cell_anchor(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor = value.into();
        self
    }

    pub fn set_default_language<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.default_language = value.into();
        self
    }

    pub fn get_chart_space(&self) -> &ChartSpace {
        match self.two_cell_anchor.get_graphic_frame() {
            Some(v) => {
                return v.get_graphic().get_graphic_data().get_chart_space();
            }
            None => {
                panic!("Non-ChartSpace.");
            }
        }
    }

    pub fn new_chart(
        &mut self,
        chart_type: ChartType,
        from_marker: MarkerType,
        to_marker: MarkerType,
        area_chart_series_list: Vec<&str>,
    ) {
        self.two_cell_anchor.set_from_marker(from_marker);
        self.two_cell_anchor.set_to_marker(to_marker);

        match chart_type {
            ChartType::LineChart => {
                self.new_chart_line_chart(area_chart_series_list);
            }
            ChartType::Line3DChart => {
                self.new_chart_line_3d_chart(area_chart_series_list);
            }
            ChartType::PieChart => {
                self.new_chart_pie_chart(area_chart_series_list);
            }
            ChartType::Pie3DChart => {
                self.new_chart_pie_3d_chart(area_chart_series_list);
            }
            ChartType::DoughnutChart => {
                self.new_chart_doughnut_chart(area_chart_series_list);
            }
            ChartType::AreaChart => {
                self.new_chart_area_chart(area_chart_series_list);
            }
            ChartType::Area3DChart => {
                self.new_chart_area_3d_chart(area_chart_series_list);
            }
            ChartType::BarChart => {
                self.new_chart_bar_chart(area_chart_series_list);
            }
            ChartType::Bar3DChart => {
                self.new_chart_bar_3d_chart(area_chart_series_list);
            }
            ChartType::OfPieChart => {
                self.new_chart_of_pie_chart(area_chart_series_list);
            }
            ChartType::BubbleChart => {
                self.new_chart_bubble_chart(area_chart_series_list);
            }
            ChartType::RadarChart => {
                self.new_chart_radar_chart(area_chart_series_list);
            }
            ChartType::ScatterChart => {
                self.new_chart_scatter_chart(area_chart_series_list);
            }
        }
    }

    pub(crate) fn new_chart_line_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let smooth = Smooth::default();

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acs_obj.set_smooth(smooth);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut line_chart = LineChart::default();
        line_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Stacked);
        line_chart.set_area_chart_series_list(acsl_obj);
        line_chart.get_show_marker_mut().set_val(true);
        line_chart.add_axis_id(axis_id1);
        line_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);

        let layout = Layout::default();

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_line_chart(line_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_line_3d_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut rotate_x = RotateX::default();
        let mut rotate_y = RotateY::default();
        let mut right_angle_axes = RightAngleAxes::default();
        rotate_x.set_val(15);
        rotate_y.set_val(10);
        right_angle_axes.set_val(true);
        let mut view_3d = View3D::default();
        view_3d.set_rotate_x(rotate_x);
        view_3d.set_rotate_y(rotate_y);
        view_3d.set_right_angle_axes(right_angle_axes);

        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let smooth = Smooth::default();

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acs_obj.set_smooth(smooth);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut line_3d_chart = Line3DChart::default();
        line_3d_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Stacked);
        line_3d_chart.set_area_chart_series_list(acsl_obj);
        line_3d_chart.add_axis_id(axis_id1);
        line_3d_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);

        let layout = Layout::default();

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .set_view_3d(view_3d);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_line_3d_chart(line_3d_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_pie_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut show_leader_lines = ShowLeaderLines::default();
        show_leader_lines.set_val(true);

        let mut pie_chart = PieChart::default();
        pie_chart.get_vary_colors_mut().set_val(true);
        pie_chart.set_area_chart_series_list(acsl_obj);
        pie_chart
            .get_data_labels_mut()
            .set_show_leader_lines(show_leader_lines);

        let layout = Layout::default();

        let default_run_properties = DefaultRunProperties::default();
        let mut end_paragraph_run_properties = EndParagraphRunProperties::default();
        end_paragraph_run_properties.set_language(&self.default_language);
        let mut paragraph = Paragraph::default();
        paragraph
            .get_paragraph_properties_mut()
            .set_right_to_left("0");
        paragraph
            .get_paragraph_properties_mut()
            .set_default_run_properties(default_run_properties);
        paragraph.set_end_para_run_properties(end_paragraph_run_properties);

        let mut text_properties = TextProperties::default();
        text_properties.add_paragraph(paragraph);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_pie_chart(pie_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_text_properties(text_properties);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_pie_3d_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut rotate_x = RotateX::default();
        let mut rotate_y = RotateY::default();
        let mut right_angle_axes = RightAngleAxes::default();
        rotate_x.set_val(15);
        rotate_y.set_val(10);
        right_angle_axes.set_val(true);
        let mut view_3d = View3D::default();
        view_3d.set_rotate_x(rotate_x);
        view_3d.set_rotate_y(rotate_y);
        view_3d.set_right_angle_axes(right_angle_axes);

        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut show_leader_lines = ShowLeaderLines::default();
        show_leader_lines.set_val(true);

        let mut pie_3d_chart = Pie3DChart::default();
        pie_3d_chart.get_vary_colors_mut().set_val(true);
        pie_3d_chart.set_area_chart_series_list(acsl_obj);
        pie_3d_chart
            .get_data_labels_mut()
            .set_show_leader_lines(show_leader_lines);

        let layout = Layout::default();

        let default_run_properties = DefaultRunProperties::default();
        let mut end_paragraph_run_properties = EndParagraphRunProperties::default();
        end_paragraph_run_properties.set_language(&self.default_language);
        let mut paragraph = Paragraph::default();
        paragraph
            .get_paragraph_properties_mut()
            .set_right_to_left("0");
        paragraph
            .get_paragraph_properties_mut()
            .set_default_run_properties(default_run_properties);
        paragraph.set_end_para_run_properties(end_paragraph_run_properties);

        let mut text_properties = TextProperties::default();
        text_properties.add_paragraph(paragraph);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .set_view_3d(view_3d);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_pie_3d_chart(pie_3d_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_text_properties(text_properties);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_doughnut_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut acs_obj = AreaChartSeries::default();
        let mut idx = 0;
        let mut ptn = 0;
        for area_chart_series in area_chart_series_list {
            if ptn == 0 {
                let mut category_axis_data = CategoryAxisData::default();
                category_axis_data
                    .get_string_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                acs_obj.set_category_axis_data(category_axis_data);

                ptn += 1;
            } else if ptn == 1 {
                let mut values = Values::default();
                values
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                values
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_values(values);

                acs_obj.get_index_mut().set_val(idx);
                acs_obj.get_order_mut().set_val(idx);
                acsl_obj.add_area_chart_series(acs_obj);

                acs_obj = AreaChartSeries::default();
                idx += 1;
                ptn = 0;
            }
        }

        let mut show_leader_lines = ShowLeaderLines::default();
        show_leader_lines.set_val(true);

        let mut doughnut_chart = DoughnutChart::default();
        doughnut_chart.get_vary_colors_mut().set_val(true);
        doughnut_chart.set_area_chart_series_list(acsl_obj);
        doughnut_chart
            .get_data_labels_mut()
            .set_show_leader_lines(show_leader_lines);
        doughnut_chart.get_hole_size_mut().set_val(50);

        let layout = Layout::default();

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_doughnut_chart(doughnut_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_area_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut area_chart = AreaChart::default();
        area_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Standard);
        area_chart.set_area_chart_series_list(acsl_obj);
        area_chart.add_axis_id(axis_id1);
        area_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);
        value_axis
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_area_chart(area_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_area_3d_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut rotate_x = RotateX::default();
        let mut rotate_y = RotateY::default();
        let mut right_angle_axes = RightAngleAxes::default();
        rotate_x.set_val(15);
        rotate_y.set_val(10);
        right_angle_axes.set_val(true);
        let mut view_3d = View3D::default();
        view_3d.set_rotate_x(rotate_x);
        view_3d.set_rotate_y(rotate_y);
        view_3d.set_right_angle_axes(right_angle_axes);

        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut area_3d_chart = Area3DChart::default();
        area_3d_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Standard);
        area_3d_chart.set_area_chart_series_list(acsl_obj);
        area_3d_chart.add_axis_id(axis_id1);
        area_3d_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);
        value_axis
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .set_view_3d(view_3d);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_area_3d_chart(area_3d_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_bar_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut invert_if_negative = InvertIfNegative::default();
            invert_if_negative.set_val(0f64);

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_invert_if_negative(invert_if_negative);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut bar_chart = BarChart::default();
        bar_chart
            .get_bar_direction_mut()
            .set_val(BarDirectionValues::Column);
        bar_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Stacked);
        bar_chart.set_area_chart_series_list(acsl_obj);
        bar_chart.get_gap_width_mut().set_val(150);
        bar_chart.get_overlap_mut().set_val(100);
        bar_chart.add_axis_id(axis_id1);
        bar_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Bottom);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_bar_chart(bar_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_bar_3d_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut rotate_x = RotateX::default();
        let mut rotate_y = RotateY::default();
        let mut right_angle_axes = RightAngleAxes::default();
        rotate_x.set_val(15);
        rotate_y.set_val(10);
        right_angle_axes.set_val(true);
        let mut view_3d = View3D::default();
        view_3d.set_rotate_x(rotate_x);
        view_3d.set_rotate_y(rotate_y);
        view_3d.set_right_angle_axes(right_angle_axes);

        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut invert_if_negative = InvertIfNegative::default();
            invert_if_negative.set_val(0f64);

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_invert_if_negative(invert_if_negative);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut bar_3d_chart = Bar3DChart::default();
        bar_3d_chart
            .get_bar_direction_mut()
            .set_val(BarDirectionValues::Column);
        bar_3d_chart
            .get_grouping_mut()
            .set_val(GroupingValues::Stacked);
        bar_3d_chart.set_area_chart_series_list(acsl_obj);
        bar_3d_chart.get_gap_width_mut().set_val(150);
        bar_3d_chart.get_shape_mut().set_val(ShapeValues::Box);
        bar_3d_chart.add_axis_id(axis_id1);
        bar_3d_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Bottom);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .set_view_3d(view_3d);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_bar_3d_chart(bar_3d_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_of_pie_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let mut invert_if_negative = InvertIfNegative::default();
            invert_if_negative.set_val(0f64);

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_invert_if_negative(invert_if_negative);
            acs_obj.set_values(values);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut show_leader_lines = ShowLeaderLines::default();
        show_leader_lines.set_val(true);

        let mut of_pie_chart = OfPieChart::default();
        of_pie_chart.get_of_pie_type_mut().set_val(OfPieValues::Bar);
        of_pie_chart.get_vary_colors_mut().set_val(true);
        of_pie_chart.set_area_chart_series_list(acsl_obj);
        of_pie_chart
            .get_data_labels_mut()
            .set_show_leader_lines(show_leader_lines);

        let default_run_properties = DefaultRunProperties::default();
        let mut end_paragraph_run_properties = EndParagraphRunProperties::default();
        end_paragraph_run_properties.set_language(&self.default_language);
        let mut paragraph = Paragraph::default();
        paragraph
            .get_paragraph_properties_mut()
            .set_right_to_left("0");
        paragraph
            .get_paragraph_properties_mut()
            .set_default_run_properties(default_run_properties);
        paragraph.set_end_para_run_properties(end_paragraph_run_properties);

        let mut text_properties = TextProperties::default();
        text_properties.add_paragraph(paragraph);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_of_pie_chart(of_pie_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_text_properties(text_properties);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_bubble_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut acs_obj = AreaChartSeries::default();
        let mut idx = 0;
        let mut ptn = 0;
        for area_chart_series in area_chart_series_list {
            if ptn == 0 {
                let mut x_values = XValues::default();
                x_values
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                x_values
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_x_values(x_values);

                ptn += 1;
            } else if ptn == 1 {
                let mut y_values = YValues::default();
                y_values
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                y_values
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_y_values(y_values);

                ptn += 1;
            } else if ptn == 2 {
                let mut bubble_size = BubbleSize::default();
                bubble_size
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                bubble_size
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_bubble_size(bubble_size);

                let mut invert_if_negative = InvertIfNegative::default();
                invert_if_negative.set_val(0f64);

                let bubble_3d = Bubble3D::default();

                acs_obj.get_index_mut().set_val(idx);
                acs_obj.get_order_mut().set_val(idx);
                acs_obj.set_invert_if_negative(invert_if_negative);
                acs_obj.set_bubble_3d(bubble_3d);
                acsl_obj.add_area_chart_series(acs_obj);

                acs_obj = AreaChartSeries::default();
                ptn = 0;
                idx += 1;
            }
        }

        let mut show_leader_lines = ShowLeaderLines::default();
        show_leader_lines.set_val(true);

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut bubble_chart = BubbleChart::default();
        bubble_chart.get_vary_colors_mut().set_val(false);
        bubble_chart.set_area_chart_series_list(acsl_obj);
        bubble_chart.get_bubble_scale_mut().set_val(100);
        bubble_chart.add_axis_id(axis_id1);
        bubble_chart.add_axis_id(axis_id2);
        bubble_chart
            .get_data_labels_mut()
            .set_show_leader_lines(show_leader_lines);

        let mut value_axis_1 = ValueAxis::default();
        value_axis_1.get_axis_id_mut().set_val(213468160);
        value_axis_1
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis_1
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Bottom);
        value_axis_1
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis_1
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis_1
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis_1
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis_1
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis_1.get_tick_crossing_axis_mut().set_val(169590080);
        value_axis_1
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let major_gridlines = MajorGridlines::default();
        let mut value_axis_2 = ValueAxis::default();
        value_axis_2.get_axis_id_mut().set_val(169590080);
        value_axis_2
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis_2
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis_2.set_major_gridlines(major_gridlines);
        value_axis_2
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis_2
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis_2
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis_2
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis_2
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis_2.get_tick_crossing_axis_mut().set_val(213468160);
        value_axis_2
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let default_run_properties = DefaultRunProperties::default();
        let mut end_paragraph_run_properties = EndParagraphRunProperties::default();
        end_paragraph_run_properties.set_language(&self.default_language);
        let mut paragraph = Paragraph::default();
        paragraph
            .get_paragraph_properties_mut()
            .set_right_to_left("0");
        paragraph
            .get_paragraph_properties_mut()
            .set_default_run_properties(default_run_properties);
        paragraph.set_end_para_run_properties(end_paragraph_run_properties);

        let mut text_properties = TextProperties::default();
        text_properties.add_paragraph(paragraph);

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_bubble_chart(bubble_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis_1);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis_2);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_text_properties(text_properties);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Gap);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_radar_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut idx = 0;
        for area_chart_series in area_chart_series_list {
            let mut values = Values::default();
            values
                .get_number_reference_mut()
                .get_formula_mut()
                .get_address_mut()
                .set_address(area_chart_series);
            values
                .get_number_reference_mut()
                .get_numbering_cache_mut()
                .get_format_code_mut()
                .set_text("General");

            let smooth = Smooth::default();

            let mut acs_obj = AreaChartSeries::default();
            acs_obj.get_index_mut().set_val(idx);
            acs_obj.get_order_mut().set_val(idx);
            acs_obj.set_values(values);
            acs_obj.set_smooth(smooth);
            acsl_obj.add_area_chart_series(acs_obj);

            idx += 1;
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut radar_chart = RadarChart::default();
        radar_chart.set_area_chart_series_list(acsl_obj);
        radar_chart.add_axis_id(axis_id1);
        radar_chart.add_axis_id(axis_id2);

        let mut category_axis = CategoryAxis::default();
        category_axis.get_axis_id_mut().set_val(213468160);
        category_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        category_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        category_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        category_axis
            .get_tick_crossing_axis_mut()
            .set_val(169590080);
        category_axis.get_auto_labeled_mut().set_val(true);
        category_axis.get_label_offset_mut().set_val(100);

        let major_gridlines = MajorGridlines::default();

        let mut value_axis = ValueAxis::default();
        value_axis.get_axis_id_mut().set_val(169590080);
        value_axis
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis.set_major_gridlines(major_gridlines);
        value_axis
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis.get_tick_crossing_axis_mut().set_val(213468160);

        let layout = Layout::default();

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_radar_chart(radar_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_category_axis(category_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }

    pub(crate) fn new_chart_scatter_chart(&mut self, area_chart_series_list: Vec<&str>) {
        let mut acsl_obj = AreaChartSeriesList::default();
        let mut acs_obj = AreaChartSeries::default();
        let mut idx = 0;
        let mut ptn = 0;
        for area_chart_series in area_chart_series_list {
            if ptn == 0 {
                let mut x_values = XValues::default();
                x_values
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                x_values
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_x_values(x_values);

                ptn += 1;
            } else if ptn == 1 {
                let mut y_values = YValues::default();
                y_values
                    .get_number_reference_mut()
                    .get_formula_mut()
                    .get_address_mut()
                    .set_address(area_chart_series);
                y_values
                    .get_number_reference_mut()
                    .get_numbering_cache_mut()
                    .get_format_code_mut()
                    .set_text("General");
                acs_obj.set_y_values(y_values);

                let mut invert_if_negative = InvertIfNegative::default();
                invert_if_negative.set_val(0f64);

                acs_obj.get_index_mut().set_val(idx);
                acs_obj.get_order_mut().set_val(idx);
                acs_obj.set_invert_if_negative(invert_if_negative);
                acsl_obj.add_area_chart_series(acs_obj);

                acs_obj = AreaChartSeries::default();
                ptn = 0;
                idx += 1;
            }
        }

        let mut axis_id1 = AxisId::default();
        axis_id1.set_val(213468160);

        let mut axis_id2 = AxisId::default();
        axis_id2.set_val(169590080);

        let mut scatter_chart = ScatterChart::default();
        scatter_chart.set_area_chart_series_list(acsl_obj);
        scatter_chart.add_axis_id(axis_id1);
        scatter_chart.add_axis_id(axis_id2);

        let mut value_axis_1 = ValueAxis::default();
        value_axis_1.get_axis_id_mut().set_val(213468160);
        value_axis_1
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis_1
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Bottom);
        value_axis_1
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis_1
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis_1
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis_1
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis_1
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis_1.get_tick_crossing_axis_mut().set_val(169590080);
        value_axis_1
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let major_gridlines = MajorGridlines::default();
        let mut value_axis_2 = ValueAxis::default();
        value_axis_2.get_axis_id_mut().set_val(169590080);
        value_axis_2
            .get_scaling_mut()
            .get_orientation_mut()
            .set_val(OrientationValues::MinMax);
        value_axis_2
            .get_axis_position_mut()
            .set_val(AxisPositionValues::Left);
        value_axis_2.set_major_gridlines(major_gridlines);
        value_axis_2
            .get_numbering_format_mut()
            .set_format_code("General");
        value_axis_2
            .get_numbering_format_mut()
            .set_source_linked(true);
        value_axis_2
            .get_major_tick_mark_mut()
            .set_val(TickMarkValues::Outside);
        value_axis_2
            .get_minor_tick_mark_mut()
            .set_val(TickMarkValues::None);
        value_axis_2
            .get_tick_label_position_mut()
            .set_val(TickLabelPositionValues::NextTo);
        value_axis_2.get_tick_crossing_axis_mut().set_val(213468160);
        value_axis_2
            .get_cross_between_mut()
            .set_val(CrossBetweenValues::MidpointCategory);

        let layout = Layout::default();

        let mut graphic_frame = GraphicFrame::default();
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_editing_language_mut()
            .set_val(&self.default_language);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .set_scatter_chart(scatter_chart);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis_1);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_area_mut()
            .add_value_axis(value_axis_2);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .get_legend_position_mut()
            .set_val(LegendPositionValues::Right);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_legend_mut()
            .set_layout(layout);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_plot_visible_only_mut()
            .set_val(true);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_chart_mut()
            .get_display_blanks_as_mut()
            .set_val(DisplayBlanksAsValues::Zero);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_bottom(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_left(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_right(0.7);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_top(0.75);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_header(0.3);
        graphic_frame
            .get_graphic_mut()
            .get_graphic_data_mut()
            .get_chart_space_mut()
            .get_print_settings_mut()
            .get_page_margins_mut()
            .set_footer(0.3);
        self.two_cell_anchor.set_graphic_frame(graphic_frame);
    }
}
