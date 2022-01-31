use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum PresetLineDashValues {
    Dash,
    DashDot,
    Dot,
    LargeDash,
    LargeDashDot,
    LargeDashDotDot,
    Solid,
    SystemDash,
    SystemDashDot,
    SystemDashDotDot,
    SystemDot,
}
impl Default for PresetLineDashValues {
    fn default() -> Self {
        Self::Solid
    }
}
impl EnumTrait for PresetLineDashValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Dash => "dash",
            Self::DashDot => "dashDot",
            Self::Dot => "dot",
            Self::LargeDash => "lgDash",
            Self::LargeDashDot => "lgDashDot",
            Self::LargeDashDotDot => "lgDashDotDot",
            Self::Solid => "solid",
            Self::SystemDash => "sysDash",
            Self::SystemDashDot => "sysDashDot",
            Self::SystemDashDotDot => "sysDashDotDot",
            Self::SystemDot => "sysDot",
        }
    }
}
impl FromStr for PresetLineDashValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dash" => Ok(Self::Dash),
            "dashDot" => Ok(Self::DashDot),
            "dot" => Ok(Self::Dot),
            "lgDash" => Ok(Self::LargeDash),
            "lgDashDot" => Ok(Self::LargeDashDot),
            "lgDashDotDot" => Ok(Self::LargeDashDotDot),
            "solid" => Ok(Self::Solid),
            "sysDash" => Ok(Self::SystemDash),
            "sysDashDot" => Ok(Self::SystemDashDot),
            "sysDashDotDot" => Ok(Self::SystemDashDotDot),
            "sysDot" => Ok(Self::SystemDot),
            _ => Err(()),
        }
    }
}
