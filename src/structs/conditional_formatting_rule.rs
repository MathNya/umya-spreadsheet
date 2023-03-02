use super::BooleanValue;
use super::ColorScale;
use super::ConditionalFormatValues;
use super::ConditionalFormattingOperatorValues;
use super::DataBar;
use super::DifferentialFormats;
use super::EnumValue;
use super::Formula;
use super::IconSet;
use super::Int32Value;
use super::StringValue;
use super::Style;
use super::TimePeriodValues;
use super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ConditionalFormattingRule {
    r#type: EnumValue<ConditionalFormatValues>,
    operator: EnumValue<ConditionalFormattingOperatorValues>,
    text: StringValue,
    priority: Int32Value,
    percent: BooleanValue,
    bottom: BooleanValue,
    rank: UInt32Value,
    stop_if_true: BooleanValue,
    std_dev: Int32Value,
    above_average: BooleanValue,
    equal_average: BooleanValue,
    time_period: EnumValue<TimePeriodValues>,
    style: Option<Style>,
    color_scale: Option<ColorScale>,
    data_bar: Option<DataBar>,
    icon_set: Option<IconSet>,
    formula: Option<Formula>,
}
impl ConditionalFormattingRule {
    pub fn get_type(&self) -> &ConditionalFormatValues {
        self.r#type.get_value()
    }

    pub fn set_type(&mut self, value: ConditionalFormatValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    pub fn get_operator(&self) -> &ConditionalFormattingOperatorValues {
        self.operator.get_value()
    }

    pub fn set_operator(&mut self, value: ConditionalFormattingOperatorValues) -> &mut Self {
        self.operator.set_value(value);
        self
    }

    pub fn get_text(&self) -> &str {
        self.text.get_value()
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text.set_value(value.into());
        self
    }

    pub fn get_priority(&self) -> &i32 {
        self.priority.get_value()
    }

    pub fn set_priority(&mut self, value: i32) -> &mut Self {
        self.priority.set_value(value);
        self
    }

    pub fn get_percent(&self) -> &bool {
        self.percent.get_value()
    }

    pub fn set_percent(&mut self, value: bool) -> &mut Self {
        self.percent.set_value(value);
        self
    }

    pub fn get_bottom(&self) -> &bool {
        self.bottom.get_value()
    }

    pub fn set_bottom(&mut self, value: bool) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    pub fn get_rank(&self) -> &u32 {
        self.rank.get_value()
    }

    pub fn set_rank(&mut self, value: u32) -> &mut Self {
        self.rank.set_value(value);
        self
    }

    pub fn get_stop_if_true(&self) -> &bool {
        self.stop_if_true.get_value()
    }

    pub fn set_stop_if_true(&mut self, value: bool) -> &mut Self {
        self.stop_if_true.set_value(value);
        self
    }

    pub fn get_std_dev(&self) -> &i32 {
        self.std_dev.get_value()
    }

    pub fn set_std_dev(&mut self, value: i32) -> &mut Self {
        self.std_dev.set_value(value);
        self
    }

    pub fn get_above_average(&self) -> &bool {
        self.above_average.get_value()
    }

    pub fn set_above_average(&mut self, value: bool) -> &mut Self {
        self.above_average.set_value(value);
        self
    }

    pub fn get_equal_average(&self) -> &bool {
        self.equal_average.get_value()
    }

    pub fn set_equal_average(&mut self, value: bool) -> &mut Self {
        self.equal_average.set_value(value);
        self
    }

    pub fn get_time_period(&self) -> &TimePeriodValues {
        self.time_period.get_value()
    }

    pub fn set_time_period(&mut self, value: TimePeriodValues) -> &mut Self {
        self.time_period.set_value(value);
        self
    }

    pub fn get_style(&self) -> &Option<Style> {
        &self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = Some(value);
        self
    }

    pub fn remove_style(&mut self) -> &mut Self {
        self.style = None;
        self
    }

    pub fn get_color_scale(&self) -> &Option<ColorScale> {
        &self.color_scale
    }

    pub fn set_color_scale(&mut self, value: ColorScale) -> &mut Self {
        self.color_scale = Some(value);
        self
    }

    pub fn remove_color_scale(&mut self) -> &mut Self {
        self.color_scale = None;
        self
    }

    pub fn get_data_bar(&self) -> &Option<DataBar> {
        &self.data_bar
    }

    pub fn set_data_bar(&mut self, value: DataBar) -> &mut Self {
        self.data_bar = Some(value);
        self
    }

    pub fn remove_data_bar(&mut self) -> &mut Self {
        self.data_bar = None;
        self
    }

    pub fn get_icon_set(&self) -> &Option<IconSet> {
        &self.icon_set
    }

    pub fn set_icon_set(&mut self, value: IconSet) -> &mut Self {
        self.icon_set = Some(value);
        self
    }

    pub fn remove_icon_set(&mut self) -> &mut Self {
        self.icon_set = None;
        self
    }

    pub fn get_formula(&self) -> &Option<Formula> {
        &self.formula
    }

    pub fn set_formula(&mut self, value: Formula) -> &mut Self {
        self.formula = Some(value);
        self
    }

    pub fn remove_formula(&mut self) -> &mut Self {
        self.formula = None;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        differential_formats: &DifferentialFormats,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"type") {
            Some(v) => {
                self.r#type.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"operator") {
            Some(v) => {
                self.operator.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"dxfId") {
            Some(v) => {
                let dxf_id = v.parse::<usize>().unwrap();
                let style = differential_formats.get_style(dxf_id);
                self.set_style(style);
            }
            None => {}
        }

        match get_attribute(e, b"priority") {
            Some(v) => {
                self.priority.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"percent") {
            Some(v) => {
                self.percent.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"bottom") {
            Some(v) => {
                self.bottom.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"rank") {
            Some(v) => {
                self.rank.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"stopIfTrue") {
            Some(v) => {
                self.stop_if_true.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"stdDev") {
            Some(v) => {
                self.std_dev.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"timePeriod") {
            Some(v) => {
                self.time_period.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"aboveAverage") {
            Some(v) => {
                self.above_average.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"equalAverage") {
            Some(v) => {
                self.equal_average.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"colorScale" => {
                        let mut obj = ColorScale::default();
                        obj.set_attributes(reader, e);
                        self.color_scale = Some(obj);
                    }
                    b"dataBar" => {
                        let mut obj = DataBar::default();
                        obj.set_attributes(reader, e);
                        self.data_bar = Some(obj);
                    }
                    b"iconSet" => {
                        let mut obj = IconSet::default();
                        obj.set_attributes(reader, e);
                        self.icon_set = Some(obj);
                    }
                    b"formula" => {
                        let mut obj = Formula::default();
                        obj.set_attributes(reader, e);
                        self.formula = Some(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"cfRule" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "cfRule"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        differential_formats: &mut DifferentialFormats,
    ) {
        let is_inner = self.color_scale.is_some()
            || self.data_bar.is_some()
            || self.icon_set.is_some()
            || self.formula.is_some();

        // cfRule
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let r#type = self.r#type.get_value_string();
        if self.r#type.has_value() {
            attributes.push(("type", r#type));
        }

        let operator = self.operator.get_value_string();
        if self.operator.has_value() {
            attributes.push(("operator", operator));
        }

        let dxf_id_str: String;
        match &self.style {
            Some(v) => {
                let dxf_id = differential_formats.set_style(v);
                dxf_id_str = dxf_id.to_string();
                attributes.push(("dxfId", &dxf_id_str));
            }
            None => {}
        }

        let priority = self.priority.get_value_string();
        if self.priority.has_value() {
            attributes.push(("priority", &priority));
        }

        let percent = self.percent.get_value_string();
        if self.percent.has_value() {
            attributes.push(("percent", percent));
        }

        let bottom = self.bottom.get_value_string();
        if self.bottom.has_value() {
            attributes.push(("bottom", bottom));
        }

        let rank = self.rank.get_value_string();
        if self.rank.has_value() {
            attributes.push(("rank", &rank));
        }

        let stop_if_true = self.stop_if_true.get_value_string();
        if self.stop_if_true.has_value() {
            attributes.push(("stopIfTrue", stop_if_true));
        }

        let std_dev = self.std_dev.get_value_string();
        if self.std_dev.has_value() {
            attributes.push(("stdDev", &std_dev));
        }

        let time_period = self.time_period.get_value_string();
        if self.time_period.has_value() {
            attributes.push(("timePeriod", time_period));
        }

        let above_average = self.above_average.get_value_string();
        if self.above_average.has_value() {
            attributes.push(("aboveAverage", above_average));
        }

        let equal_average = self.equal_average.get_value_string();
        if self.equal_average.has_value() {
            attributes.push(("equalAverage", equal_average));
        }

        write_start_tag(writer, "cfRule", attributes, !is_inner);

        if is_inner {
            // colorScale
            match &self.color_scale {
                Some(v) => v.write_to(writer),
                None => {}
            }

            // dataBar
            match &self.data_bar {
                Some(v) => v.write_to(writer),
                None => {}
            }

            // iconSet
            match &self.icon_set {
                Some(v) => v.write_to(writer),
                None => {}
            }

            // formula
            match &self.formula {
                Some(v) => v.write_to(writer),
                None => {}
            }

            write_end_tag(writer, "cfRule");
        }
    }
}
