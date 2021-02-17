use super::non_visual_drawing_properties::NonVisualDrawingProperties;

#[derive(Default, Debug)]
pub struct NonVisualShapeProperties  {
    non_visual_drawing_properties: NonVisualDrawingProperties,
}
impl NonVisualShapeProperties  {
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    pub fn set_non_visual_drawing_properties(&mut self, value:NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }
}
