// c:ser
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
    Bubble3D,
    BubbleSize,
    CategoryAxisData,
    DataLabels,
    Explosion,
    Formula,
    Index,
    InvertIfNegative,
    Marker,
    Order,
    SeriesText,
    ShapeProperties,
    Smooth,
    Values,
    XValues,
    YValues,
};
use crate::{
    structs::Workbook,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct AreaChartSeries {
    index:              Index,
    order:              Order,
    series_text:        Option<SeriesText>,
    explosion:          Option<Explosion>,
    invert_if_negative: Option<InvertIfNegative>,
    marker:             Option<Marker>,
    shape_properties:   Option<ShapeProperties>,
    category_axis_data: Option<CategoryAxisData>,
    values:             Option<Values>,
    x_values:           Option<XValues>,
    y_values:           Option<YValues>,
    bubble_size:        Option<BubbleSize>,
    bubble_3d:          Option<Bubble3D>,
    smooth:             Option<Smooth>,
    data_labels:        Option<DataLabels>,
}

impl AreaChartSeries {
    #[must_use]
    pub fn index(&self) -> &Index {
        &self.index
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use index()")]
    pub fn get_index(&self) -> &Index {
        self.index()
    }

    pub fn index_mut(&mut self) -> &mut Index {
        &mut self.index
    }

    #[deprecated(since = "3.0.0", note = "Use index_mut()")]
    pub fn get_index_mut(&mut self) -> &mut Index {
        self.index_mut()
    }

    pub fn set_index(&mut self, value: Index) -> &mut Self {
        self.index = value;
        self
    }

    #[must_use]
    pub fn order(&self) -> &Order {
        &self.order
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use order()")]
    pub fn get_order(&self) -> &Order {
        self.order()
    }

    pub fn order_mut(&mut self) -> &mut Order {
        &mut self.order
    }

    #[deprecated(since = "3.0.0", note = "Use order_mut()")]
    pub fn get_order_mut(&mut self) -> &mut Order {
        self.order_mut()
    }

    pub fn set_order(&mut self, value: Order) -> &mut Self {
        self.order = value;
        self
    }

    #[must_use]
    pub fn series_text(&self) -> Option<&SeriesText> {
        self.series_text.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use series_text()")]
    pub fn get_series_text(&self) -> Option<&SeriesText> {
        self.series_text()
    }

    pub fn series_text_mut(&mut self) -> Option<&mut SeriesText> {
        self.series_text.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use series_text_mut()")]
    pub fn get_series_text_mut(&mut self) -> Option<&mut SeriesText> {
        self.series_text_mut()
    }

    pub fn set_series_text(&mut self, value: SeriesText) -> &mut Self {
        self.series_text = Some(value);
        self
    }

    #[must_use]
    pub fn explosion(&self) -> Option<&Explosion> {
        self.explosion.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use explosion()")]
    pub fn get_explosion(&self) -> Option<&Explosion> {
        self.explosion()
    }

    pub fn explosion_mut(&mut self) -> Option<&mut Explosion> {
        self.explosion.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use explosion_mut()")]
    pub fn get_explosion_mut(&mut self) -> Option<&mut Explosion> {
        self.explosion_mut()
    }

    pub fn set_explosion(&mut self, value: Explosion) -> &mut Self {
        self.explosion = Some(value);
        self
    }

    #[must_use]
    pub fn invert_if_negative(&self) -> Option<&InvertIfNegative> {
        self.invert_if_negative.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use invert_if_negative()")]
    pub fn get_invert_if_negative(&self) -> Option<&InvertIfNegative> {
        self.invert_if_negative()
    }

    pub fn invert_if_negative_mut(&mut self) -> Option<&mut InvertIfNegative> {
        self.invert_if_negative.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use invert_if_negative_mut()")]
    pub fn get_invert_if_negative_mut(&mut self) -> Option<&mut InvertIfNegative> {
        self.invert_if_negative_mut()
    }

    pub fn set_invert_if_negative(&mut self, value: InvertIfNegative) -> &mut Self {
        self.invert_if_negative = Some(value);
        self
    }

    #[must_use]
    pub fn marker(&self) -> Option<&Marker> {
        self.marker.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use marker()")]
    pub fn get_marker(&self) -> Option<&Marker> {
        self.marker()
    }

    pub fn marker_mut(&mut self) -> Option<&mut Marker> {
        self.marker.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use marker_mut()")]
    pub fn get_marker_mut(&mut self) -> Option<&mut Marker> {
        self.marker_mut()
    }

    pub fn set_marker(&mut self, value: Marker) -> &mut Self {
        self.marker = Some(value);
        self
    }

    #[must_use]
    pub fn shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape_properties()")]
    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties()
    }

    pub fn shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use shape_properties_mut()")]
    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    #[must_use]
    pub fn category_axis_data(&self) -> Option<&CategoryAxisData> {
        self.category_axis_data.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use category_axis_data()")]
    pub fn get_category_axis_data(&self) -> Option<&CategoryAxisData> {
        self.category_axis_data()
    }

    pub fn category_axis_data_mut(&mut self) -> Option<&mut CategoryAxisData> {
        self.category_axis_data.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use category_axis_data_mut()")]
    pub fn get_category_axis_data_mut(&mut self) -> Option<&mut CategoryAxisData> {
        self.category_axis_data_mut()
    }

    pub fn set_category_axis_data(&mut self, value: CategoryAxisData) -> &mut Self {
        self.category_axis_data = Some(value);
        self
    }

    #[must_use]
    pub fn values(&self) -> Option<&Values> {
        self.values.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use values()")]
    pub fn get_values(&self) -> Option<&Values> {
        self.values()
    }

    pub fn values_mut(&mut self) -> Option<&mut Values> {
        self.values.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use values_mut()")]
    pub fn get_values_mut(&mut self) -> Option<&mut Values> {
        self.values_mut()
    }

    pub fn set_values(&mut self, value: Values) -> &mut Self {
        self.values = Some(value);
        self
    }

    #[must_use]
    pub fn x_values(&self) -> Option<&XValues> {
        self.x_values.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use x_values()")]
    pub fn get_x_values(&self) -> Option<&XValues> {
        self.x_values()
    }

    pub fn x_values_mut(&mut self) -> Option<&mut XValues> {
        self.x_values.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use x_values_mut()")]
    pub fn get_x_values_mut(&mut self) -> Option<&mut XValues> {
        self.x_values_mut()
    }

    pub fn set_x_values(&mut self, value: XValues) -> &mut Self {
        self.x_values = Some(value);
        self
    }

    #[must_use]
    pub fn y_values(&self) -> Option<&YValues> {
        self.y_values.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use y_values()")]
    pub fn get_y_values(&self) -> Option<&YValues> {
        self.y_values()
    }

    pub fn y_values_mut(&mut self) -> Option<&mut YValues> {
        self.y_values.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use y_values_mut()")]
    pub fn get_y_values_mut(&mut self) -> Option<&mut YValues> {
        self.y_values_mut()
    }

    pub fn set_y_values(&mut self, value: YValues) -> &mut Self {
        self.y_values = Some(value);
        self
    }

    #[must_use]
    pub fn bubble_size(&self) -> Option<&BubbleSize> {
        self.bubble_size.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bubble_size()")]
    pub fn get_bubble_size(&self) -> Option<&BubbleSize> {
        self.bubble_size()
    }

    pub fn bubble_size_mut(&mut self) -> Option<&mut BubbleSize> {
        self.bubble_size.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use bubble_size_mut()")]
    pub fn get_bubble_size_mut(&mut self) -> Option<&mut BubbleSize> {
        self.bubble_size_mut()
    }

    pub fn set_bubble_size(&mut self, value: BubbleSize) -> &mut Self {
        self.bubble_size = Some(value);
        self
    }

    #[must_use]
    pub fn bubble_3d(&self) -> Option<&Bubble3D> {
        self.bubble_3d.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bubble_3d()")]
    pub fn get_bubble_3d(&self) -> Option<&Bubble3D> {
        self.bubble_3d()
    }

    pub fn bubble_3d_mut(&mut self) -> Option<&mut Bubble3D> {
        self.bubble_3d.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use bubble_3d_mut()")]
    pub fn get_bubble_3d_mut(&mut self) -> Option<&mut Bubble3D> {
        self.bubble_3d_mut()
    }

    pub fn set_bubble_3d(&mut self, value: Bubble3D) -> &mut Self {
        self.bubble_3d = Some(value);
        self
    }

    #[must_use]
    pub fn smooth(&self) -> Option<&Smooth> {
        self.smooth.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use smooth()")]
    pub fn get_smooth(&self) -> Option<&Smooth> {
        self.smooth()
    }

    pub fn smooth_mut(&mut self) -> Option<&mut Smooth> {
        self.smooth.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use smooth_mut()")]
    pub fn get_smooth_mut(&mut self) -> Option<&mut Smooth> {
        self.smooth_mut()
    }

    pub fn set_smooth(&mut self, value: Smooth) -> &mut Self {
        self.smooth = Some(value);
        self
    }

    #[must_use]
    pub fn data_labels(&self) -> Option<&DataLabels> {
        self.data_labels.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_labels()")]
    pub fn get_data_labels(&self) -> Option<&DataLabels> {
        self.data_labels()
    }

    pub fn data_labels_mut(&mut self) -> Option<&mut DataLabels> {
        self.data_labels.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use data_labels_mut()")]
    pub fn get_data_labels_mut(&mut self) -> Option<&mut DataLabels> {
        self.data_labels_mut()
    }

    pub fn set_data_labels(&mut self, value: DataLabels) -> &mut Self {
        self.data_labels = Some(value);
        self
    }

    pub fn formula_mut(&mut self) -> Vec<&mut Formula> {
        let mut result: Vec<&mut Formula> = Vec::default();

        if let Some(v) = &mut self.category_axis_data {
            if let Some(h) = v.string_reference_mut() {
                result.push(h.get_formula_mut());
            }
        }
        if let Some(v) = &mut self.values {
            result.push(v.get_number_reference_mut().get_formula_mut());
        }
        if let Some(v) = &mut self.x_values {
            result.push(v.get_number_reference_mut().get_formula_mut());
        }
        if let Some(v) = &mut self.y_values {
            result.push(v.get_number_reference_mut().get_formula_mut());
        }
        if let Some(v) = &mut self.bubble_size {
            result.push(v.number_reference_mut().get_formula_mut());
        }
        result
    }

    #[deprecated(since = "3.0.0", note = "Use formula_mut()")]
    pub fn get_formula_mut(&mut self) -> Vec<&mut Formula> {
        self.formula_mut()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().into_inner() {
                b"c:v" => {
                    let mut obj = SeriesText::default();
                    obj.set_attributes(reader, e);
                    self.set_series_text(obj);
                }
                b"c:marker" => {
                    let mut obj = Marker::default();
                    obj.set_attributes(reader, e, false);
                    self.set_marker(obj);
                }
                b"c:spPr" => {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
                }
                b"c:cat" => {
                    let mut obj = CategoryAxisData::default();
                    obj.set_attributes(reader, e);
                    self.set_category_axis_data(obj);
                }
                b"c:val" => {
                    let mut obj = Values::default();
                    obj.set_attributes(reader, e);
                    self.set_values(obj);
                }
                b"c:xVal" => {
                    let mut obj = XValues::default();
                    obj.set_attributes(reader, e);
                    self.set_x_values(obj);
                }
                b"c:yVal" => {
                    let mut obj = YValues::default();
                    obj.set_attributes(reader, e);
                    self.set_y_values(obj);
                }
                b"c:bubbleSize" => {
                    let mut obj = BubbleSize::default();
                    obj.set_attributes(reader, e);
                    self.set_bubble_size(obj);
                }
                b"c:dLbls" => {
                    let mut obj = DataLabels::default();
                    obj.set_attributes(reader, e);
                    self.set_data_labels(obj);
                }
                _ => (),
            },
            Event::Empty(ref e) => match e.name().into_inner() {
                b"c:idx" => {
                    self.index.set_attributes(reader, e);
                }
                b"c:order" => {
                    self.order.set_attributes(reader, e);
                }
                b"c:explosion" => {
                    let mut obj = Explosion::default();
                    obj.set_attributes(reader, e);
                    self.set_explosion(obj);
                }
                b"c:invertIfNegative" => {
                    let mut obj = InvertIfNegative::default();
                    obj.set_attributes(reader, e);
                    self.set_invert_if_negative(obj);
                }
                b"c:bubble3D" => {
                    let mut obj = Bubble3D::default();
                    obj.set_attributes(reader, e);
                    self.set_bubble_3d(obj);
                }
                b"c:smooth" => {
                    let mut obj = Smooth::default();
                    obj.set_attributes(reader, e);
                    self.set_smooth(obj);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:ser" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:ser"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:ser
        write_start_tag(writer, "c:ser", vec![], false);

        // c:idx
        self.index.write_to(writer);

        // c:order
        self.order.write_to(writer);

        // c:v
        if let Some(v) = &self.series_text {
            v.write_to(writer);
        }

        // c:explosion
        if let Some(v) = &self.explosion {
            v.write_to(writer);
        }

        // c:invertIfNegative
        if let Some(v) = &self.invert_if_negative {
            v.write_to(writer);
        }

        // c:marker
        if let Some(v) = &self.marker {
            v.write_to(writer);
        }

        // c:spPr
        if let Some(v) = &self.shape_properties {
            v.write_to(writer);
        }

        // c:dLbls
        if let Some(v) = &self.data_labels {
            v.write_to(writer);
        }

        // c:cat
        if let Some(v) = &self.category_axis_data {
            v.write_to(writer, wb);
        }

        // c:val
        if let Some(v) = &self.values {
            v.write_to(writer, wb);
        }

        // c:xVal
        if let Some(v) = &self.x_values {
            v.write_to(writer, wb);
        }

        // c:yVal
        if let Some(v) = &self.y_values {
            v.write_to(writer, wb);
        }

        // c:bubbleSize
        if let Some(v) = &self.bubble_size {
            v.write_to(writer, wb);
        }

        // c:bubble3D
        if let Some(v) = &self.bubble_3d {
            v.write_to(writer);
        }

        // c:smooth
        if let Some(v) = &self.smooth {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:ser");
    }
}
