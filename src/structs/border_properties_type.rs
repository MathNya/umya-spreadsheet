use super::BorderStyleValues;
use super::Color;

pub trait BorderPropertiesType {
    fn get_color(&self) -> &Color;

    fn get_color_mut(&mut self) -> &mut Color;

    fn set_color(&mut self, value: Color) -> &mut Self;

    fn get_style(&self) -> &BorderStyleValues;

    fn set_style(&mut self, value: BorderStyleValues) -> &mut Self;
}
