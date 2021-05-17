use super::title::Title;
use super::legend::Legend;
use super::plot_area::PlotArea;
use super::axis::Axis;
use super::grid_lines::GridLines;
use super::data_series::DataSeries;
use super::super::transform2d::Transform2D;
use super::super::spreadsheet::non_visual_drawing_properties::NonVisualDrawingProperties;
use reader::xlsx::drawing_rels;
use reader::xlsx::chart;
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;
use tempdir::TempDir;

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
            transform: Transform2D::default(),
        }
    }
}
impl Chart {
    
    pub const LANG_EN_GB: &'static str = "en_GB";
    pub const LANG_JA_JP: &'static str = "ja-JP";

    pub fn get_non_visual_drawing_properties(&self)-> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self)-> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value:NonVisualDrawingProperties)-> &mut Chart {
        self.non_visual_drawing_properties = value;
        self
    }

    pub fn get_name(&self)-> &str {
        return &self.non_visual_drawing_properties.get_name();
    }

    pub fn set_name<S: Into<String>>(&mut self, value:S)-> &mut Chart {
        self.non_visual_drawing_properties.set_name(value.into());
        self
    }

    pub fn get_title(&self) -> &Option<Title> {
        return &self.title;
    }

    pub fn set_title(&mut self, value:Title)-> &mut Chart {
        self.title = Some(value);
        self
    }

    pub fn get_lang(&self) -> &str {
        return &self.lang;
    }

    pub fn set_lang<S: Into<String>>(&mut self, value:S)-> &mut Chart {
        self.lang = value.into();
        self
    }

    pub fn get_legend(&self)-> &Legend {
        return &self.legend;
    }

    pub(crate) fn set_legend(&mut self, value:Legend)-> &mut Chart {
        self.legend = value;
        self
    }

    pub fn get_plot_area(&self)-> &PlotArea {
        &self.plot_area
    }

    pub fn get_plot_area_mut(&mut self)-> &mut PlotArea {
        &mut self.plot_area
    }

    pub fn set_plot_area(&mut self, value:PlotArea)-> &mut Chart {
        self.plot_area = value;
        self
    }

    pub fn get_plot_visible_only(&self)-> &bool {
        return &self.plot_visible_only;
    }

    pub fn set_plot_visible_only(&mut self, value:bool)-> &mut Chart {
        self.plot_visible_only = value;
        self
    }

    pub fn get_display_blanks_as(&self)-> &str {
        return &self.display_blanks_as;
    }

    pub fn set_display_blanks_as<S: Into<String>>(&mut self, value:S)-> &mut Chart {
        self.display_blanks_as = value.into();
        self
    }

    pub fn get_chart_axis_y(&self)-> &Axis {
        return &self.y_axis;
    }

    pub fn set_chart_axis_y(&mut self, value:Axis)-> &mut Chart {
        self.y_axis = value;
        self
    }

    pub fn get_chart_axis_x(&self) -> &Axis {
        return &self.x_axis;
    }

    pub fn set_chart_axis_x(&mut self, value:Axis)-> &mut Chart {
        self.x_axis = value;
        self
    }

    pub fn get_major_gridlines(&self) -> &GridLines {
        return &self.major_gridlines;
    }

    pub fn get_minor_gridlines(&self) -> &GridLines {
        return &self.minor_gridlines;
    }

    pub fn set_transform(&mut self, value:Transform2D)-> &mut Chart {
        self.transform = value;
        self
    }

    pub fn get_transform(&self) -> &Transform2D {
        &self.transform
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart,
        dir: &TempDir,
        target: &str
    ) {
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:xfrm" => {
                            &mut self.transform.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"xdr:cNvPr" => {
                            &mut self.non_visual_drawing_properties.set_attributes(reader, e);
                        },
                        b"c:chart" => {
                            let chart_id = get_attribute(e, b"r:id").unwrap();
                            let drawing_rel = drawing_rels::read(dir, target).unwrap();
                            for (drawing_id, _, drawing_target) in &drawing_rel {
                                if &chart_id == drawing_id {
                                    chart::read(&dir, &drawing_target, self).unwrap();
                                }
                            }
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:graphicFrame" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:graphicFrame"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // xdr:graphicFrame
        write_start_tag(writer, "xdr:graphicFrame", vec![
            ("macro", ""),
        ], false);
        
        // xdr:nvGraphicFramePr
        write_start_tag(writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        &self.non_visual_drawing_properties.write_to(writer);
        
        // xdr:cNvGraphicFramePr
        write_start_tag(writer, "xdr:cNvGraphicFramePr", vec![], true);

        write_end_tag(writer, "xdr:nvGraphicFramePr");

        // xdr:xfrm
        &self.transform.write_to(writer, "xdr:xfrm");

        // a:graphic
        write_start_tag(writer, "a:graphic", vec![], false);

        // a:graphicData
        write_start_tag(writer, "a:graphicData", vec![
            ("uri", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
        ], false);

        // c:chart
        write_start_tag(writer, "c:chart", vec![
            ("xmlns:c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
            ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
            ("r:id", format!("rId{}", r_id).as_str()),
        ], true);

        write_end_tag(writer, "a:graphicData");

        write_end_tag(writer, "a:graphic");

        write_end_tag(writer, "xdr:graphicFrame");
    }
}