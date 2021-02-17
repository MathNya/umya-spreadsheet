// a:solidFill
use super::scheme_color::SchemeColor;
use super::rgb_color_model_hex::RgbColorModelHex;

#[derive(Default, Debug)]
pub struct SolidFill {
    scheme_color: Option<SchemeColor>,
    rgb_color_model_hex: Option<RgbColorModelHex>,
}
impl SolidFill {
    pub fn get_scheme_color(&self) -> &Option<SchemeColor> {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut Option<SchemeColor> {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value:SchemeColor) {
        self.scheme_color = Some(value);
    }

    pub fn get_rgb_color_model_hex(&self) -> &Option<RgbColorModelHex> {
        &self.rgb_color_model_hex
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> &mut Option<RgbColorModelHex> {
        &mut self.rgb_color_model_hex
    }
    
    pub fn set_rgb_color_model_hex(&mut self, value:RgbColorModelHex) {
        self.rgb_color_model_hex = Some(value);
    }
}
