use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::data_series::DataSeries;
use super::super::structs::data_series_values::DataSeriesValues;
use super::super::structs::layout::Layout;
use super::super::structs::plot_area::PlotArea;
use super::super::structs::chart::Chart;
use super::super::structs::title::Title;
use super::super::structs::text_element::TextElement;
use super::super::structs::legend::Legend;
use super::super::structs::axis::Axis;

pub fn read(dir: &TempDir, target: &String, chart: &mut Chart) -> result::Result<(), XlsxError> {
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut layout:Layout = Layout::default();
    let mut plot_area:PlotArea = PlotArea::default();
    let mut plot_series:Vec<DataSeries> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:title" => {
                        chart.set_title(chart_title(&mut reader));
                    },
                    b"c:layout" => {
                        layout = chart_layout_details(&mut reader);
                    },
                    b"c:lineChart" => {
                        let mut data_series = DataSeries::default();
                        data_series.set_plot_type(DataSeries::TYPE_LINECHART);
                        chart_data_series(&mut reader, &mut data_series);
                        plot_series.push(data_series);
                    },
                    b"c:line3DChart" => {
                        let mut data_series = DataSeries::default();
                        data_series.set_plot_type(DataSeries::TYPE_LINECHART_3D);
                        chart_data_series(&mut reader, &mut data_series);
                        plot_series.push(data_series);
                    },
                    b"c:doughnutChart" => {
                        let mut data_series = DataSeries::default();
                        data_series.set_plot_type(DataSeries::TYPE_DOUGHNUTCHART);
                        chart_data_series(&mut reader, &mut data_series);
                        plot_series.push(data_series);
                    },
                    b"c:pieChart" => {
                        let mut data_series = DataSeries::default();
                        data_series.set_plot_type(DataSeries::TYPE_PIECHART);
                        chart_data_series(&mut reader, &mut data_series);
                        plot_series.push(data_series);
                    },
                    b"c:pie3DChart" => {
                        let mut data_series = DataSeries::default();
                        data_series.set_plot_type(DataSeries::TYPE_PIECHART_3D);
                        chart_data_series(&mut reader, &mut data_series);
                        plot_series.push(data_series);
                    },
                    b"c:dLbls" => {
                        read_chart_attributes(&mut reader, &mut layout);
                    },
                    b"c:catAx" => {
                        chart.set_chart_axis_y(read_axis(&mut reader));
                    },
                    b"c:valAx" => {
                        chart.set_chart_axis_x(read_axis(&mut reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:lang" => {
                        chart.set_lang(get_attribute(e, b"val").unwrap());
                    },
                    b"c:plotVisOnly" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        chart.set_plot_visible_only(value);
                    },
                    b"c:dispBlanksAs" => {
                        chart.set_display_blanks_as(get_attribute(e, b"val").unwrap());
                    },
                    b"c:legend" => {
                        chart.set_legend(chart_legend(&mut reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:plotArea" => {
                        plot_area.set_layout(layout);
                        plot_area.set_plot_series(plot_series);
                        chart.set_plot_area(plot_area);
                        plot_area = PlotArea::default();
                        layout = Layout::default();
                        plot_series = Vec::new();
                    },
                    b"c:chartSpace" => {
                        //spreadsheet.
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}

fn read_axis(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->Axis
{
    let mut buf = Vec::new();
    let mut axis = Axis::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:title" => {
                        axis.set_label(chart_title(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:axId" => {
                        let value = get_attribute(e, b"val").unwrap();
                        axis.set_id(value.parse::<usize>().unwrap());
                    },
                    b"c:orientation" => {
                        axis.set_axis_options("orientation", get_attribute(e, b"val").unwrap().as_str());
                    },
                    b"c:majorTickMark" => {
                        axis.set_axis_options("major_tick_mark", get_attribute(e, b"val").unwrap().as_str());
                    },
                    b"c:minorTickMark" => {
                        axis.set_axis_options("minor_tick_mark", get_attribute(e, b"val").unwrap().as_str());
                    },
                    b"c:tickLblPos" => {
                        axis.set_axis_options("axis_labels", get_attribute(e, b"val").unwrap().as_str());
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:catAx" => return axis,
                    b"c:valAx" => return axis,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:catAx or c:valAx"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn chart_legend(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->Legend
{
    let mut buf = Vec::new();
    let mut legend = Legend::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:layout" => {
                        legend.set_layout(chart_layout_details(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:legendPos" => {
                        legend.set_position(get_attribute(e, b"val").unwrap());
                    },
                    b"c:overlay" => {
                        legend.set_overlay(get_attribute(e, b"val").unwrap() == "1");
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:legend" => return legend,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:legend"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn chart_title(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->Title
{
    let mut buf = Vec::new();
    let mut title = Title::default();

    let mut string_value:String = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:layout" => {
                        title.set_layout(chart_layout_details(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:t" => {
                        let mut text_element = TextElement::default();
                        text_element.set_text(string_value.clone());
                        title.get_caption_mut().add_rich_text_elements(text_element);
                    },
                    b"c:title" => return title,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:title"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn chart_layout_details(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->Layout
{
    let mut buf = Vec::new();
    let mut layout = Layout::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:layoutTarget" => {
                        layout.set_layout_target(get_attribute(e, b"val").unwrap());
                    },
                    b"c:xMode" => {
                        layout.set_x_mode(get_attribute(e, b"val").unwrap());
                    },
                    b"c:yMode" => {
                        layout.set_y_mode(get_attribute(e, b"val").unwrap());
                    },
                    b"c:x" => {
                        let value = get_attribute(e, b"val").unwrap();
                        layout.set_x_pos(value.parse::<f64>().unwrap());
                    },
                    b"c:y" => {
                        let value = get_attribute(e, b"val").unwrap();
                        layout.set_y_pos(value.parse::<f64>().unwrap());
                    },
                    b"c:w" => {
                        let value = get_attribute(e, b"val").unwrap();
                        layout.set_width(value.parse::<i32>().unwrap());
                    },
                    b"c:h" => {
                        let value = get_attribute(e, b"val").unwrap();
                        layout.set_height(value.parse::<i32>().unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:layout" => return layout,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:layout"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn chart_data_series(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    data_series:&mut DataSeries
)
{
    let mut buf = Vec::new();

    let mut marker:Option<String> = None;
    let mut series_index:i32 = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:tx" => {
                        data_series.add_plot_label(series_index, get_val(reader, &marker));
                    },
                    b"c:cat" => {
                        data_series.add_plot_category(series_index, get_val(reader, &marker));
                    },
                     b"c:val" => {
                        data_series.add_plot_values(series_index, get_val(reader, &marker));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:grouping" => {
                        data_series.set_plot_grouping(get_attribute(e, b"val").unwrap());
                    },
                    b"c:varyColors" => {
                        get_attribute(e, b"val");
                    },
                    b"c:idx" => {
                        let value = get_attribute(e, b"val").unwrap();
                        series_index = value.parse::<i32>().unwrap();
                    },
                    b"c:order"=> {
                        let value = get_attribute(e, b"val").unwrap();
                        data_series.add_plot_order(series_index, value.parse::<i32>().unwrap());
                    },
                    b"c:symbol" => {
                        marker = Some(get_attribute(e, b"val").unwrap());
                    },
                    b"c:smooth" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        data_series.set_smooth_line(value);
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:lineChart" => return,
                    b"c:line3DChart" => return,
                    b"c:doughnutChart" => return,
                    b"c:pieChart" => return,
                    b"c:pie3DChart" => return,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:lineChart"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_val(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    marker:&Option<String>
)->DataSeriesValues
{
    let mut buf = Vec::new();
    let mut data_series_values:DataSeriesValues = DataSeriesValues::default();

    let mut string_value:String = String::from("");
    let mut idx:i32 = 0;

    match marker {
        Some(v) => {data_series_values.set_point_marker(v);},
        None => {}
    }

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:strRef" => {
                        data_series_values.set_data_type(DataSeriesValues::DATASERIES_TYPE_STRING);
                    },
                    b"c:numRef" => {
                        data_series_values.set_data_type(DataSeriesValues::DATASERIES_TYPE_NUMBER);
                    },
                    b"c:pt" => {
                        let value = get_attribute(e, b"idx").unwrap();
                        idx = value.parse::<i32>().unwrap();
                    }
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:ptCount" => {
                        let value = get_attribute(e, b"val").unwrap();
                        data_series_values.set_point_count(value.parse::<i32>().unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:formatCode" => {
                        data_series_values.set_format_code(string_value.clone());
                    },
                    b"c:f" => {
                        data_series_values.set_data_source(string_value.clone());
                    }
                    b"c:v" => {
                        data_series_values.add_data_values(idx, string_value.clone());
                    }
                    b"c:tx" => return data_series_values,
                    b"c:cat" => return data_series_values,
                    b"c:val" => return data_series_values,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:val"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn read_chart_attributes(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    layout:&mut Layout
)
{
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"c:showLegendKey" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_legend_key(value);
                    },
                    b"c:showVal" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_val(value);
                    },
                    b"c:showCatName" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_cat_name(value);
                    },
                    b"c:showSerName" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_ser_name(value);
                    },
                    b"c:showPercent" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_percent(value);
                    },
                    b"c:showBubbleSize" => {
                        let value = get_attribute(e, b"val").unwrap() == "1";
                        layout.set_show_bubble_size(value);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"c:dLbls" => return,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:dLbls"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}