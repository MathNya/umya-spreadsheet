// line3DChart
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    AreaChartSeries,
    AreaChartSeriesList,
    AxisId,
    DataLabels,
    Grouping,
    VaryColors,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::Workbook,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Line3DChart {
    grouping:               Grouping,
    vary_colors:            VaryColors,
    area_chart_series_list: AreaChartSeriesList,
    data_labels:            DataLabels,
    axis_id:                Vec<AxisId>,
}

impl Line3DChart {
    #[must_use]
    pub fn get_grouping(&self) -> &Grouping {
        &self.grouping
    }

    pub fn get_grouping_mut(&mut self) -> &mut Grouping {
        &mut self.grouping
    }

    pub fn set_grouping(&mut self, value: Grouping) -> &mut Line3DChart {
        self.grouping = value;
        self
    }

    #[must_use]
    pub fn get_vary_colors(&self) -> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self) -> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value: VaryColors) -> &mut Line3DChart {
        self.vary_colors = value;
        self
    }

    #[must_use]
    pub fn get_area_chart_series_list(&self) -> &AreaChartSeriesList {
        &self.area_chart_series_list
    }

    pub fn get_area_chart_series_list_mut(&mut self) -> &mut AreaChartSeriesList {
        &mut self.area_chart_series_list
    }

    pub fn set_area_chart_series_list(&mut self, value: AreaChartSeriesList) -> &mut Self {
        self.area_chart_series_list = value;
        self
    }

    #[must_use]
    pub fn get_data_labels(&self) -> &DataLabels {
        &self.data_labels
    }

    pub fn get_data_labels_mut(&mut self) -> &mut DataLabels {
        &mut self.data_labels
    }

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut Line3DChart {
        self.data_labels = value;
        self
    }

    #[must_use]
    pub fn get_axis_id(&self) -> &[AxisId] {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self) -> &mut Vec<AxisId> {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value: impl Into<Vec<AxisId>>) -> &mut Line3DChart {
        self.axis_id = value.into();
        self
    }

    pub fn add_axis_id(&mut self, value: AxisId) -> &mut Line3DChart {
        self.axis_id.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"c:ser" => {
                        let mut obj = AreaChartSeries::default();
                        obj.set_attributes(reader, e);
                        self.get_area_chart_series_list_mut()
                            .add_area_chart_series(obj);
                        }
                    b"c:dLbls" => {
                        self.data_labels.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"c:grouping" => {
                        self.grouping.set_attributes(reader, e);
                    }
                    b"c:varyColors" => {
                        self.vary_colors.set_attributes(reader, e);
                    }
                    b"c:axId" => {
                        let mut obj = AxisId::default();
                        obj.set_attributes(reader, e);
                        self.add_axis_id(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:line3DChart" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:line3DChart")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:line3DChart
        write_start_tag(writer, "c:line3DChart", vec![], false);

        // c:grouping
        self.grouping.write_to(writer);

        // c:varyColors
        self.vary_colors.write_to(writer);

        // c:ser
        for v in self.area_chart_series_list.get_area_chart_series() {
            v.write_to(writer, wb);
        }

        // c:dLbls
        self.data_labels.write_to(writer);

        // c:axId
        for v in &self.axis_id {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:line3DChart");
    }
}
