use super::Color;

#[derive(Default, Debug)]
pub(crate) struct Shadow {
    visible: bool,
    blur_radius: i32,
    distance: i32,
    direction: i32,
    alignment: i32,
    color: Color,
    alpha: i32,
}
