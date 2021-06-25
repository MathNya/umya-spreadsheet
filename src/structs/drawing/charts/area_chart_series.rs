// c:ser
use super::Index;
use super::Order;
use super::Marker;
use super::CategoryAxisData;
use super::Values;
use super::Smooth;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct AreaChartSeries {
    index: Index,
    order: Order,
    marker: Option<Marker>,
    category_axis_data: Option<CategoryAxisData>,
    values: Values,
    smooth: Option<Smooth>,
}
impl AreaChartSeries {
    pub fn get_index(&self)-> &Index {
        &self.index
    }

    pub fn get_index_mut(&mut self)-> &Index {
        &mut self.index
    }

    pub fn set_index(&mut self, value:Index)-> &mut AreaChartSeries {
        self.index = value;
        self
    }

    pub fn get_order(&self)-> &Order {
        &self.order
    }

    pub fn get_order_mut(&mut self)-> &Order {
        &mut self.order
    }

    pub fn set_order(&mut self, value:Order)-> &mut AreaChartSeries {
        self.order = value;
        self
    }

    pub fn get_marker(&self)-> &Option<Marker> {
        &self.marker
    }

    pub fn get_marker_mut(&mut self)-> &Option<Marker> {
        &mut self.marker
    }

    pub fn set_marker(&mut self, value:Marker)-> &mut AreaChartSeries {
        self.marker = Some(value);
        self
    }

    pub fn get_category_axis_data(&self)-> &Option<CategoryAxisData> {
        &self.category_axis_data
    }

    pub fn get_category_axis_data_mut(&mut self)-> &Option<CategoryAxisData> {
        &mut self.category_axis_data
    }

    pub fn set_category_axis_data(&mut self, value:CategoryAxisData)-> &mut AreaChartSeries {
        self.category_axis_data = Some(value);
        self
    }

    pub fn get_values(&self)-> &Values {
        &self.values
    }

    pub fn get_values_mut(&mut self)-> &mut Values {
        &mut self.values
    }

    pub fn set_values(&mut self, value:Values)-> &mut AreaChartSeries {
        self.values = value;
        self
    }

    pub fn get_smooth(&self)-> &Option<Smooth> {
        &self.smooth
    }

    pub fn get_smooth_mut(&mut self)-> &Option<Smooth> {
        &mut self.smooth
    }

    pub fn set_smooth(&mut self, value:Smooth)-> &mut AreaChartSeries {
        self.smooth = Some(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:marker" => {
                            let mut obj = Marker::default();
                            obj.set_attributes(reader, e, false);
                            self.set_marker(obj);
                        },
                        b"c:cat" => {
                            let mut obj = CategoryAxisData::default();
                            obj.set_attributes(reader, e);
                            self.set_category_axis_data(obj);
                        },
                        b"c:val" => {
                            self.values.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:idx" => {
                            self.index.set_attributes(reader, e);
                        },
                        b"c:order" => {
                            self.order.set_attributes(reader, e);
                        },
                        b"c:smooth" => {
                            let mut obj = Smooth::default();
                            obj.set_attributes(reader, e);
                            self.set_smooth(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:ser" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:ser"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:ser
        write_start_tag(writer, "c:ser", vec![], false);

        // c:idx
        &self.index.write_to(writer);

        // c:order
        &self.order.write_to(writer);

        // c:marker
        match &self.marker {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:cat
        match &self.category_axis_data {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:val
        &self.values.write_to(writer);

        // c:smooth
        match &self.smooth {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        write_end_tag(writer, "c:ser");
    }
}
