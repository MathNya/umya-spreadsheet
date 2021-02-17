use super::super::style_matrix_reference_type::StyleMatrixReferenceType;

#[derive(Default, Debug)]
pub struct ShapeStyle {
    line_reference: Option<StyleMatrixReferenceType>,
    fill_reference: Option<StyleMatrixReferenceType>,
    effect_reference: Option<StyleMatrixReferenceType>,
    font_reference: Option<StyleMatrixReferenceType>,
}
impl ShapeStyle {
    pub fn get_line_reference(&self)-> &Option<StyleMatrixReferenceType> {
        &self.line_reference
    }
    
    pub fn set_line_reference(&mut self, value:StyleMatrixReferenceType) {
        self.line_reference = Some(value);
    }

    pub fn get_fill_reference(&self)-> &Option<StyleMatrixReferenceType> {
        &self.fill_reference
    }

    pub fn set_fill_reference(&mut self, value:StyleMatrixReferenceType) {
        self.fill_reference = Some(value);
    }

    pub fn get_effect_reference(&self)-> &Option<StyleMatrixReferenceType> {
        &self.effect_reference
    }

    pub fn set_effect_reference(&mut self, value:StyleMatrixReferenceType) {
        self.effect_reference = Some(value);
    }

    pub fn get_font_reference(&self)-> &Option<StyleMatrixReferenceType> {
        &self.font_reference
    }

    pub fn set_font_reference(&mut self, value:StyleMatrixReferenceType) {
        self.font_reference = Some(value);
    }
}