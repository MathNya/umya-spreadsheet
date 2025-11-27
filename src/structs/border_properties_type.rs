use super::{
    BorderStyleValues,
    Color,
};

pub trait BorderPropertiesType {
    fn color(&self) -> &Color;

    fn color_mut(&mut self) -> &mut Color;

    fn set_color(&mut self, value: Color) -> &mut Self;

    fn style(&self) -> &BorderStyleValues;

    fn set_style(&mut self, value: BorderStyleValues) -> &mut Self;
}
