// c:chartSpace
use super::Date1904;
use super::EditingLanguage;
use super::RoundedCorners;
use super::super::super::AlternateContent;
use super::Chart;
use super::ShapeProperties;
use super::PrintSettings;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ChartSpace {
    date1904: Date1904,
    editing_language: EditingLanguage,
    rounded_corners: RoundedCorners,
    alternate_content: AlternateContent,
    chart: Chart,
    shape_properties: Option<ShapeProperties>,
    print_settings: PrintSettings,
}
impl ChartSpace {
    pub fn get_date1904(&self)-> &Date1904 {
        &self.date1904
    }

    pub fn get_date1904_mut(&mut self)-> &mut Date1904 {
        &mut self.date1904
    }

    pub fn set_date1904(&mut self, value:Date1904)-> &mut ChartSpace {
        self.date1904 = value;
        self
    }

    pub fn get_editing_language(&self)-> &EditingLanguage {
        &self.editing_language
    }

    pub fn get_editing_language_mut(&mut self)-> &mut EditingLanguage {
        &mut self.editing_language
    }

    pub fn set_editing_language(&mut self, value:EditingLanguage)-> &mut ChartSpace {
        self.editing_language = value;
        self
    }

    pub fn get_rounded_corners(&self)-> &RoundedCorners {
        &self.rounded_corners
    }

    pub fn get_rounded_corners_mut(&mut self)-> &mut RoundedCorners {
        &mut self.rounded_corners
    }

    pub fn set_rounded_corners(&mut self, value:RoundedCorners)-> &mut ChartSpace {
        self.rounded_corners = value;
        self
    }

    pub fn get_alternate_content(&self)-> &AlternateContent {
        &self.alternate_content
    }

    pub fn get_alternate_content_mut(&mut self)-> &mut AlternateContent {
        &mut self.alternate_content
    }

    pub fn set_alternate_content(&mut self, value:AlternateContent)-> &mut ChartSpace {
        self.alternate_content = value;
        self
    }

    pub fn get_chart(&self)-> &Chart {
        &self.chart
    }

    pub fn get_chart_mut(&mut self)-> &mut Chart {
        &mut self.chart
    }

    pub fn set_chart(&mut self, value:Chart)-> &mut ChartSpace {
        self.chart = value;
        self
    }

    pub fn get_shape_properties(&self)-> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self)-> &mut Option<ShapeProperties> {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value:ShapeProperties)-> &mut ChartSpace {
        self.shape_properties = Some(value);
        self
    }

    pub fn get_print_settings(&self)-> &PrintSettings {
        &self.print_settings
    }

    pub fn get_print_settings_mut(&mut self)-> &mut PrintSettings {
        &mut self.print_settings
    }

    pub fn set_print_settings(&mut self, value:PrintSettings)-> &mut ChartSpace {
        self.print_settings = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"mc:AlternateContent" => {
                            &mut self.alternate_content.set_attributes(reader, e);
                        }
                        b"c:chart" => {
                            &mut self.chart.set_attributes(reader, e);
                        }
                        b"c:printSettings" => {
                            &mut self.print_settings.set_attributes(reader, e);
                        }
                        b"c:spPr" => {
                            let mut obj = ShapeProperties::default();
                            obj.set_attributes(reader, e);
                            self.set_shape_properties(obj);
                        }
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:date1904" => {
                            &mut self.date1904.set_attributes(reader, e);
                        }
                        b"c:lang" => {
                            &mut self.editing_language.set_attributes(reader, e);
                        }
                        b"c:roundedCorners" => {
                            &mut self.rounded_corners.set_attributes(reader, e);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:chartSpace" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:chartSpace"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:chartSpace
        write_start_tag(writer, "c:chartSpace", vec![
            ("xmlns:c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
            ("xmlns:a", "http://schemas.openxmlformats.org/drawingml/2006/main"),
            ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
        ], false);

        // c:date1904
        &self.date1904.write_to(writer);

        // c:lang
        &self.editing_language.write_to(writer);

        // c:roundedCorners
        &self.rounded_corners.write_to(writer);

        // mc:AlternateContent
        &self.alternate_content.write_to(writer);

        // c:chart
        &self.chart.write_to(writer);

        // c:spPr
        match &self.shape_properties {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:printSettings
        &self.print_settings.write_to(writer);

        write_end_tag(writer, "c:chartSpace");
    }
}
