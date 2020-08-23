#[derive(Default, Debug)]
pub struct GridLines {
    object_state: bool,
    line_properties: Vec<String>,
    shadow_properties: Vec<String>,
    glow_properties: Vec<String>,
    soft_edges: Vec<String>,
}
impl GridLines {
    pub fn get_object_state(&self)-> &bool {
        &self.object_state
    }
}