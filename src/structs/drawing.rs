//! structs of drawing.

pub mod charts;
pub mod spreadsheet;

mod transform2d;
pub use self::transform2d::*;

mod solid_fill;
pub use self::solid_fill::*;

mod scheme_color;
pub use self::scheme_color::*;

mod rgb_color_model_hex;
pub use self::rgb_color_model_hex::*;

mod preset_geometry;
pub use self::preset_geometry::*;

mod adjust_value_list;
pub use self::adjust_value_list::*;

mod shape_guide;
pub use self::shape_guide::*;

mod style_matrix_reference_type;
pub use self::style_matrix_reference_type::*;

mod outline;
pub use self::outline::*;

mod tail_end;
pub use self::tail_end::*;

mod picture_locks;
pub use self::picture_locks::*;

mod stretch;
pub use self::stretch::*;

mod fill_rectangle;
pub use self::fill_rectangle::*;

mod blip;
pub use self::blip::*;

mod source_rectangle;
pub use self::source_rectangle::*;

mod effect_list;
pub use self::effect_list::*;

mod outer_shadow;
pub use self::outer_shadow::*;

mod preset_color;
pub use self::preset_color::*;

mod alpha;
pub use self::alpha::*;

mod run;
pub use self::run::*;

mod run_properties;
pub use self::run_properties::*;

mod paragraph;
pub use self::paragraph::*;

mod body_properties;
pub use self::body_properties::*;

mod list_style;
pub use self::list_style::*;

mod offset;
pub use self::offset::*;

mod extents;
pub use self::extents::*;

mod graphic;
pub use self::graphic::*;

mod graphic_data;
pub use self::graphic_data::*;

mod paragraph_properties;
pub use self::paragraph_properties::*;

mod no_fill;
pub use self::no_fill::*;

mod pattern_fill;
pub use self::pattern_fill::*;

mod foreground_color;
pub use self::foreground_color::*;

mod background_color;
pub use self::background_color::*;

mod gradient_fill;
pub use self::gradient_fill::*;

mod gradient_stop_list;
pub use self::gradient_stop_list::*;

mod gradient_stop;
pub use self::gradient_stop::*;

mod start_connection;
pub use self::start_connection::*;

mod end_connection;
pub use self::end_connection::*;

mod linear_gradient_fill;
pub use self::linear_gradient_fill::*;

mod tile_flip_values;
pub use self::tile_flip_values::*;

mod tile_rectangle;
pub use self::tile_rectangle::*;

mod glow;
pub use self::glow::*;

mod soft_edge;
pub use self::soft_edge::*;

mod bevel;
pub use self::bevel::*;

mod preset_camera_values;
pub use self::preset_camera_values::*;

mod camera;
pub use self::camera::*;

mod light_rig_direction_values;
pub use self::light_rig_direction_values::*;

mod light_rig_values;
pub use self::light_rig_values::*;

mod light_rig;
pub use self::light_rig::*;

mod scene_3d_type;
pub use self::scene_3d_type::*;

mod bevel_preset_values;
pub use self::bevel_preset_values::*;

mod bevel_top;
pub use self::bevel_top::*;

mod bevel_bottom;
pub use self::bevel_bottom::*;

mod shape_3d_type;
pub use self::shape_3d_type::*;

mod preset_material_type_values;
pub use self::preset_material_type_values::*;

mod text_wrapping_values;
pub use self::text_wrapping_values::*;

mod shape_auto_fit;
pub use self::shape_auto_fit::*;

mod text_alignment_type_values;
pub use self::text_alignment_type_values::*;

mod text_caps_values;
pub use self::text_caps_values::*;

mod preset_line_dash_values;
pub use self::preset_line_dash_values::*;

mod preset_dash;
pub use self::preset_dash::*;

mod miter;
pub use self::miter::*;

mod line_spacing;
pub use self::line_spacing::*;

mod spacing_percent;
pub use self::spacing_percent::*;

mod round;
pub use self::round::*;

mod supplemental_font;
pub use self::supplemental_font::*;

mod theme;
pub use self::theme::*;

mod font_collection_type;
pub use self::font_collection_type::*;

mod theme_elements;
pub use self::theme_elements::*;

mod color_scheme;
pub use self::color_scheme::*;

mod font_scheme;
pub use self::font_scheme::*;

mod format_scheme;
pub use self::format_scheme::*;

mod color2_type;
pub use self::color2_type::*;

mod fill_style_list;
pub use self::fill_style_list::*;

mod line_style_list;
pub use self::line_style_list::*;

mod effect_style_list;
pub use self::effect_style_list::*;

mod effect_style;
pub use self::effect_style::*;

mod background_fill_style_list;
pub use self::background_fill_style_list::*;

mod system_color;
pub use self::system_color::*;

mod system_color_values;
pub use self::system_color_values::*;

mod text_font_type;
pub use self::text_font_type::*;

mod scheme_color_values;
pub use self::scheme_color_values::*;

mod percentage_type;
pub use self::percentage_type::*;

mod positive_fixed_percentage_type;
pub use self::positive_fixed_percentage_type::*;

mod pen_alignment_values;
pub use self::pen_alignment_values::*;

mod rotation;
pub use self::rotation::*;
