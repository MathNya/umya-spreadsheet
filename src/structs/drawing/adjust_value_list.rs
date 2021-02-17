// a:avLst
use super::shape_guide::ShapeGuide;

#[derive(Default, Debug)]
pub struct AdjustValueList {
    shape_guide_collection: Vec<ShapeGuide>,
}
impl AdjustValueList {
    pub fn get_shape_guide_collection(&self) -> &Vec<ShapeGuide> {
        &self.shape_guide_collection
    }

    pub fn get_shape_guide_collection_mut(&mut self) -> &mut Vec<ShapeGuide> {
        &mut self.shape_guide_collection
    }

    pub fn set_shape_guide_collection(&mut self, value:Vec<ShapeGuide>) {
        self.shape_guide_collection = value;
    }

    pub fn add_shape_guide_collection(&mut self, value:ShapeGuide) {
        self.shape_guide_collection.push(value);
    }
}
