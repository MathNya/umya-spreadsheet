// c:chartSpace
use super::Chart;
use super::Date1904;
use super::EditingLanguage;
use super::PrintSettings;
use super::RoundedCorners;
use super::ShapeProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::office2010::drawing::charts::Style;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ChartSpace {
    date1904: Date1904,
    editing_language: EditingLanguage,
    rounded_corners: RoundedCorners,
    style: Style,
    chart: Chart,
    shape_properties: Option<ShapeProperties>,
    print_settings: Option<PrintSettings>,
}
impl ChartSpace {
    pub fn get_date1904(&self) -> &Date1904 {
        &self.date1904
    }

    pub fn get_date1904_mut(&mut self) -> &mut Date1904 {
        &mut self.date1904
    }

    pub fn set_date1904(&mut self, value: Date1904) -> &mut Self {
        self.date1904 = value;
        self
    }

    pub fn get_editing_language(&self) -> &EditingLanguage {
        &self.editing_language
    }

    pub fn get_editing_language_mut(&mut self) -> &mut EditingLanguage {
        &mut self.editing_language
    }

    pub fn set_editing_language(&mut self, value: EditingLanguage) -> &mut Self {
        self.editing_language = value;
        self
    }

    pub fn get_rounded_corners(&self) -> &RoundedCorners {
        &self.rounded_corners
    }

    pub fn get_rounded_corners_mut(&mut self) -> &mut RoundedCorners {
        &mut self.rounded_corners
    }

    pub fn set_rounded_corners(&mut self, value: RoundedCorners) -> &mut Self {
        self.rounded_corners = value;
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = value;
        self
    }

    pub fn get_chart(&self) -> &Chart {
        &self.chart
    }

    pub fn get_chart_mut(&mut self) -> &mut Chart {
        &mut self.chart
    }

    pub fn set_chart(&mut self, value: Chart) -> &mut Self {
        self.chart = value;
        self
    }

    pub fn get_shape_properties(&self) -> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut Option<ShapeProperties> {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    pub fn get_print_settings(&self) -> &Option<PrintSettings> {
        &self.print_settings
    }

    pub fn get_print_settings_mut(&mut self) -> &mut Option<PrintSettings> {
        &mut self.print_settings
    }

    pub fn set_print_settings(&mut self, value: PrintSettings) -> &mut Self {
        self.print_settings = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        let mut obj = Style::default();
                        obj.set_attributes(reader, e);
                        self.set_style(obj);
                    }
                    b"c:chart" => {
                        self.chart.set_attributes(reader, e);
                    }
                    b"c:printSettings" => {
                        let mut obj = PrintSettings::default();
                        obj.set_attributes(reader, e);
                        self.set_print_settings(obj);
                    }
                    b"c:spPr" => {
                        let mut obj = ShapeProperties::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:date1904" => {
                        self.date1904.set_attributes(reader, e);
                    }
                    b"c:lang" => {
                        self.editing_language.set_attributes(reader, e);
                    }
                    b"c:roundedCorners" => {
                        self.rounded_corners.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:chartSpace" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:chartSpace"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:chartSpace
        write_start_tag(
            writer,
            "c:chartSpace",
            vec![
                (
                    "xmlns:c",
                    "http://schemas.openxmlformats.org/drawingml/2006/chart",
                ),
                (
                    "xmlns:a",
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                ),
                (
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                ),
            ],
            false,
        );

        // c:date1904
        self.date1904.write_to(writer);

        // c:lang
        self.editing_language.write_to(writer);

        // c:roundedCorners
        self.rounded_corners.write_to(writer);

        // mc:AlternateContent
        self.style.write_to(writer);

        // c:chart
        self.chart.write_to(writer, spreadsheet);

        // c:spPr
        match &self.shape_properties {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:printSettings
        match &self.print_settings {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:chartSpace");
    }
}
