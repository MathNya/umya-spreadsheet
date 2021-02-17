// xdr:cxnSp
use super::super::super::anchor::Anchor;
use super::non_visual_shape_properties::NonVisualShapeProperties;
use super::non_visual_connection_shape_properties::NonVisualConnectionShapeProperties;
use super::shape_properties::ShapeProperties;
use super::shape_style::ShapeStyle;

#[derive(Default, Debug)]
pub struct ConnectionShape {
    anchor: Anchor,
    non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: ShapeStyle,
}
impl ConnectionShape {
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value:Anchor) {
        self.anchor = value;
    }

    pub fn get_non_visual_connection_shape_properties(&self) -> &NonVisualConnectionShapeProperties {
        &self.non_visual_connection_shape_properties
    }

    pub fn get_non_visual_connection_shape_properties_mut(&mut self) -> &mut NonVisualConnectionShapeProperties {
        &mut self.non_visual_connection_shape_properties
    }

    pub fn set_non_visual_connection_shape_properties(&mut self, value:NonVisualConnectionShapeProperties) {
        self.non_visual_connection_shape_properties = value;
    }

    pub fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value:ShapeProperties) {
        self.shape_properties = value;
    }

    pub fn get_shape_style(&self) -> &ShapeStyle {
        &self.shape_style
    }

    pub fn get_shape_style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.shape_style
    }

    pub fn set_shape_style(&mut self, value:ShapeStyle) {
        self.shape_style = value;
    }
}
