// c:ser
use super::Index;
use super::Order;
use super::Explosion;
use super::InvertIfNegative;
use super::Marker;
use super::ShapeProperties;
use super::CategoryAxisData;
use super::Values;
use super::XValues;
use super::YValues;
use super::BubbleSize;
use super::Bubble3D;
use super::Smooth;
use super::Formula;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct AreaChartSeries {
    index: Index,
    order: Order,
    explosion: Option<Explosion>,
    invert_if_negative: Option<InvertIfNegative>,
    marker: Option<Marker>,
    shape_properties: Option<ShapeProperties>,
    category_axis_data: Option<CategoryAxisData>,
    values: Option<Values>,
    x_values: Option<XValues>,
    y_values: Option<YValues>,
    bubble_size: Option<BubbleSize>,
    bubble_3d: Option<Bubble3D>,
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

    pub fn get_explosion(&self)-> &Option<Explosion> {
        &self.explosion
    }

    pub fn get_explosion_mut(&mut self)-> &Option<Explosion> {
        &mut self.explosion
    }

    pub fn set_explosion(&mut self, value:Explosion)-> &mut AreaChartSeries {
        self.explosion = Some(value);
        self
    }

    pub fn get_invert_if_negative(&self)-> &Option<InvertIfNegative> {
        &self.invert_if_negative
    }

    pub fn get_invert_if_negative_mut(&mut self)-> &Option<InvertIfNegative> {
        &mut self.invert_if_negative
    }

    pub fn set_invert_if_negative(&mut self, value:InvertIfNegative)-> &mut AreaChartSeries {
        self.invert_if_negative = Some(value);
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

    pub fn get_shape_properties(&self)-> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self)-> &Option<ShapeProperties> {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value:ShapeProperties)-> &mut AreaChartSeries {
        self.shape_properties = Some(value);
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

    pub fn get_values(&self)-> &Option<Values> {
        &self.values
    }

    pub fn get_values_mut(&mut self)-> &mut Option<Values> {
        &mut self.values
    }

    pub fn set_values(&mut self, value:Values)-> &mut AreaChartSeries {
        self.values = Some(value);
        self
    }

    pub fn get_x_values(&self)-> &Option<XValues> {
        &self.x_values
    }

    pub fn get_x_values_mut(&mut self)-> &mut Option<XValues> {
        &mut self.x_values
    }

    pub fn set_x_values(&mut self, value:XValues)-> &mut AreaChartSeries {
        self.x_values = Some(value);
        self
    }

    pub fn get_y_values(&self)-> &Option<YValues> {
        &self.y_values
    }

    pub fn get_y_values_mut(&mut self)-> &mut Option<YValues> {
        &mut self.y_values
    }

    pub fn set_y_values(&mut self, value:YValues)-> &mut AreaChartSeries {
        self.y_values = Some(value);
        self
    }

    pub fn get_bubble_size(&self)-> &Option<BubbleSize> {
        &self.bubble_size
    }

    pub fn get_bubble_size_mut(&mut self)-> &mut Option<BubbleSize> {
        &mut self.bubble_size
    }

    pub fn set_bubble_size(&mut self, value:BubbleSize)-> &mut AreaChartSeries {
        self.bubble_size = Some(value);
        self
    }

    pub fn get_bubble_3d(&self)-> &Option<Bubble3D> {
        &self.bubble_3d
    }

    pub fn get_bubble_3d_mut(&mut self)-> &Option<Bubble3D> {
        &mut self.bubble_3d
    }

    pub fn set_bubble_3d(&mut self, value:Bubble3D)-> &mut AreaChartSeries {
        self.bubble_3d = Some(value);
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

    pub fn get_formula_mut(&mut self)-> Vec<&mut Formula> {
        let mut result:Vec<&mut Formula> = Vec::default();

        match &mut self.category_axis_data {
            Some(v) => {
                result.push(v.get_string_reference_mut().get_formula_mut());
            }
            None => {}
        }
        match &mut self.values {
            Some(v) => {
                result.push(v.get_number_reference_mut().get_formula_mut());
            }
            None => {}
        }
        match &mut self.x_values {
            Some(v) => {
                result.push(v.get_number_reference_mut().get_formula_mut());
            }
            None => {}
        }
        match &mut self.y_values {
            Some(v) => {
                result.push(v.get_number_reference_mut().get_formula_mut());
            }
            None => {}
        }
        match &mut self.bubble_size {
            Some(v) => {
                result.push(v.get_number_reference_mut().get_formula_mut());
            }
            None => {}
        }
        result
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
                        b"c:spPr" => {
                            let mut obj = ShapeProperties::default();
                            obj.set_attributes(reader, e);
                            self.set_shape_properties(obj);
                        },
                        b"c:cat" => {
                            let mut obj = CategoryAxisData::default();
                            obj.set_attributes(reader, e);
                            self.set_category_axis_data(obj);
                        },
                        b"c:val" => {
                            let mut obj = Values::default();
                            obj.set_attributes(reader, e);
                            self.set_values(obj);
                        },
                        b"c:xVal" => {
                            let mut obj = XValues::default();
                            obj.set_attributes(reader, e);
                            self.set_x_values(obj);
                        },
                        b"c:yVal" => {
                            let mut obj = YValues::default();
                            obj.set_attributes(reader, e);
                            self.set_y_values(obj);
                        },
                        b"c:bubbleSize" => {
                            let mut obj = BubbleSize::default();
                            obj.set_attributes(reader, e);
                            self.set_bubble_size(obj);
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
                        b"c:explosion" => {
                            let mut obj = Explosion::default();
                            obj.set_attributes(reader, e);
                            self.set_explosion(obj);
                        },
                        b"c:invertIfNegative" => {
                            let mut obj = InvertIfNegative::default();
                            obj.set_attributes(reader, e);
                            self.set_invert_if_negative(obj);
                        },
                        b"c:bubble3D" => {
                            let mut obj = Bubble3D::default();
                            obj.set_attributes(reader, e);
                            self.set_bubble_3d(obj);
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

        // c:explosion
        match &self.explosion {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:invertIfNegative
        match &self.invert_if_negative {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:marker
        match &self.marker {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:spPr
        match &self.shape_properties {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:cat
        match &self.category_axis_data {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:val
        match &self.values {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:xVal
        match &self.x_values {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:yVal
        match &self.y_values {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:bubbleSize
        match &self.bubble_size {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:bubble3D
        match &self.bubble_3d {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:smooth
        match &self.smooth {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        write_end_tag(writer, "c:ser");
    }
}
