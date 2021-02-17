// xdr:nvCxnSpPr
use super::non_visual_drawing_properties::NonVisualDrawingProperties;
use super::non_visual_connector_shape_drawing_properties::NonVisualConnectorShapeDrawingProperties;

#[derive(Default, Debug)]
pub struct NonVisualConnectionShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
}
impl NonVisualConnectionShapeProperties {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value:NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }

    pub fn get_non_visual_connector_shape_drawing_properties(&self) -> &NonVisualConnectorShapeDrawingProperties {
        &self.non_visual_connector_shape_drawing_properties
    }

    pub fn get_non_visual_connector_shape_drawing_properties_mut(&mut self) -> &mut NonVisualConnectorShapeDrawingProperties {
        &mut self.non_visual_connector_shape_drawing_properties
    }

    pub fn set_non_visual_connector_shape_drawing_properties(&mut self, value:NonVisualConnectorShapeDrawingProperties) {
        self.non_visual_connector_shape_drawing_properties = value;
    }
}
