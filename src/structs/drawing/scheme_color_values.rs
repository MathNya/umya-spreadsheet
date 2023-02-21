use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum SchemeColorValues {
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Background1,
    Background2,
    Dark1,
    Dark2,
    FollowedHyperlink,
    Hyperlink,
    Light1,
    Light2,
    PhColor,
    Text1,
    Text2,
}
impl Default for SchemeColorValues {
    fn default() -> Self {
        Self::Background1
    }
}
impl EnumTrait for SchemeColorValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Accent1 => "accent1",
            Self::Accent2 => "accent2",
            Self::Accent3 => "accent3",
            Self::Accent4 => "accent4",
            Self::Accent5 => "accent5",
            Self::Accent6 => "accent6",
            Self::Background1 => "bg1",
            Self::Background2 => "bg2",
            Self::Dark1 => "dk1",
            Self::Dark2 => "dk2",
            Self::FollowedHyperlink => "folHlink",
            Self::Hyperlink => "hlink",
            Self::Light1 => "lt1",
            Self::Light2 => "lt2",
            Self::PhColor => "phClr",
            Self::Text1 => "tx1",
            Self::Text2 => "tx2",
        }
    }
}
impl FromStr for SchemeColorValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "accent1" => Ok(Self::Accent1),
            "accent2" => Ok(Self::Accent2),
            "accent3" => Ok(Self::Accent3),
            "accent4" => Ok(Self::Accent4),
            "accent5" => Ok(Self::Accent5),
            "accent6" => Ok(Self::Accent6),
            "bg1" => Ok(Self::Background1),
            "bg2" => Ok(Self::Background2),
            "dk1" => Ok(Self::Dark1),
            "dk2" => Ok(Self::Dark2),
            "folHlink" => Ok(Self::FollowedHyperlink),
            "hlink" => Ok(Self::Hyperlink),
            "lt1" => Ok(Self::Light1),
            "lt2" => Ok(Self::Light2),
            "phClr" => Ok(Self::PhColor),
            "tx1" => Ok(Self::Text1),
            "tx2" => Ok(Self::Text2),
            _ => Err(()),
        }
    }
}
