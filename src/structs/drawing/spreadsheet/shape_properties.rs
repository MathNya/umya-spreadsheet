// xdr:spPr
use super::super::transform2d::Transform2D;
use super::super::preset_geometry::PresetGeometry;
use super::super::solid_fill::SolidFill;
use super::super::outline::Outline;

#[derive(Default, Debug)]
pub struct ShapeProperties {
    transform2d: Transform2D,
    preset_geometry: PresetGeometry,
    solid_fill: Option<SolidFill>,
    outline: Option<Outline>,
}
impl ShapeProperties {
    pub fn get_geometry(&self) -> &PresetGeometry {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut PresetGeometry {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value:PresetGeometry) {
        self.preset_geometry = value;
    }

    pub fn get_transform2d(&self) -> &Transform2D {
        &self.transform2d
    }

    pub fn get_transform2d_mut(&mut self) -> &mut Transform2D {
        &mut self.transform2d
    }

    pub fn set_transform2d(&mut self, value:Transform2D) {
        self.transform2d = value;
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value:SolidFill) {
        self.solid_fill = Some(value);
    }

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value:Outline) {
        self.outline = Some(value);
    }
}
