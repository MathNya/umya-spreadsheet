// c:dLbls
use super::ShowBubbleSize;
use super::ShowCategoryName;
use super::ShowLeaderLines;
use super::ShowLegendKey;
use super::ShowPercent;
use super::ShowSeriesName;
use super::ShowValue;
use super::TextProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct DataLabels {
    show_legend_key: ShowLegendKey,
    show_value: ShowValue,
    show_category_name: ShowCategoryName,
    show_series_name: ShowSeriesName,
    show_percent: ShowPercent,
    show_bubble_size: ShowBubbleSize,
    show_leader_lines: Option<ShowLeaderLines>,
    text_properties: Option<TextProperties>,
}
impl DataLabels {
    pub fn get_show_legend_key(&self) -> &ShowLegendKey {
        &self.show_legend_key
    }

    pub fn get_show_legend_key_mut(&mut self) -> &mut ShowLegendKey {
        &mut self.show_legend_key
    }

    pub fn set_show_legend_key(&mut self, value: ShowLegendKey) -> &mut Self {
        self.show_legend_key = value;
        self
    }

    pub fn get_show_value(&self) -> &ShowValue {
        &self.show_value
    }

    pub fn get_show_value_mut(&mut self) -> &mut ShowValue {
        &mut self.show_value
    }

    pub fn set_show_value(&mut self, value: ShowValue) -> &mut Self {
        self.show_value = value;
        self
    }

    pub fn get_show_category_name(&self) -> &ShowCategoryName {
        &self.show_category_name
    }

    pub fn get_show_category_name_mut(&mut self) -> &mut ShowCategoryName {
        &mut self.show_category_name
    }

    pub fn set_show_category_name(&mut self, value: ShowCategoryName) -> &mut Self {
        self.show_category_name = value;
        self
    }

    pub fn get_show_series_name(&self) -> &ShowSeriesName {
        &self.show_series_name
    }

    pub fn get_show_series_name_mut(&mut self) -> &mut ShowSeriesName {
        &mut self.show_series_name
    }

    pub fn set_show_series_name(&mut self, value: ShowSeriesName) -> &mut Self {
        self.show_series_name = value;
        self
    }

    pub fn get_show_percent(&self) -> &ShowPercent {
        &self.show_percent
    }

    pub fn get_show_percent_mut(&mut self) -> &mut ShowPercent {
        &mut self.show_percent
    }

    pub fn set_show_percent(&mut self, value: ShowPercent) -> &mut Self {
        self.show_percent = value;
        self
    }

    pub fn get_show_bubble_size(&self) -> &ShowBubbleSize {
        &self.show_bubble_size
    }

    pub fn get_show_bubble_size_mut(&mut self) -> &mut ShowBubbleSize {
        &mut self.show_bubble_size
    }

    pub fn set_show_bubble_size(&mut self, value: ShowBubbleSize) -> &mut Self {
        self.show_bubble_size = value;
        self
    }

    pub fn get_show_leader_lines(&self) -> &Option<ShowLeaderLines> {
        &self.show_leader_lines
    }

    pub fn get_show_leader_lines_mut(&mut self) -> &mut Option<ShowLeaderLines> {
        &mut self.show_leader_lines
    }

    pub fn set_show_leader_lines(&mut self, value: ShowLeaderLines) -> &mut Self {
        self.show_leader_lines = Some(value);
        self
    }

    pub fn get_text_properties(&self) -> &Option<TextProperties> {
        &self.text_properties
    }

    pub fn get_text_properties_mut(&mut self) -> &mut Option<TextProperties> {
        &mut self.text_properties
    }

    pub fn set_text_properties(&mut self, value: TextProperties) -> &mut Self {
        self.text_properties = Some(value);
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
                    b"c:txPr" => {
                        let mut obj = TextProperties::default();
                        obj.set_attributes(reader, e);
                        self.set_text_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"c:showLegendKey" => {
                        self.show_legend_key.set_attributes(reader, e);
                    }
                    b"c:showVal" => {
                        self.show_value.set_attributes(reader, e);
                    }
                    b"c:showCatName" => {
                        self.show_category_name.set_attributes(reader, e);
                    }
                    b"c:showSerName" => {
                        self.show_series_name.set_attributes(reader, e);
                    }
                    b"c:showPercent" => {
                        self.show_percent.set_attributes(reader, e);
                    }
                    b"c:showBubbleSize" => {
                        self.show_bubble_size.set_attributes(reader, e);
                    }
                    b"c:showLeaderLines" => {
                        let mut obj = ShowLeaderLines::default();
                        obj.set_attributes(reader, e);
                        self.set_show_leader_lines(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:dLbls" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:dLbls"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:dLbls
        write_start_tag(writer, "c:dLbls", vec![], false);

        // c:txPr
        match &self.text_properties {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // c:showLegendKey
        self.show_legend_key.write_to(writer);

        // c:showVal
        self.show_value.write_to(writer);

        // c:showCatName
        self.show_category_name.write_to(writer);

        // c:showSerName
        self.show_series_name.write_to(writer);

        // c:showPercent
        self.show_percent.write_to(writer);

        // c:showBubbleSize
        self.show_bubble_size.write_to(writer);

        // c:showLeaderLines
        match &self.show_leader_lines {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:dLbls");
    }
}
