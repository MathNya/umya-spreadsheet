use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LightRigValues {
    Balanced,
    BrightRoom,
    Chilly,
    Contrasting,
    Flat,
    Flood,
    Freezing,
    Glow,
    Harsh,
    LegacyFlat1,
    LegacyFlat2,
    LegacyFlat3,
    LegacyFlat4,
    LegacyHarsh1,
    LegacyHarsh2,
    LegacyHarsh3,
    LegacyHarsh4,
    LegacyNormal1,
    LegacyNormal2,
    LegacyNormal3,
    LegacyNormal4,
    Morning,
    Soft,
    Sunrise,
    Sunset,
    ThreePoints,
    TwoPoints,
}
impl Default for LightRigValues {
    fn default() -> Self {
        Self::LegacyFlat1
    }
}
impl EnumTrait for LightRigValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Balanced => "balanced",
            Self::BrightRoom => "brightRoom",
            Self::Chilly => "chilly",
            Self::Contrasting => "contrasting",
            Self::Flat => "flat",
            Self::Flood => "flood",
            Self::Freezing => "freezing",
            Self::Glow => "glow",
            Self::Harsh => "harsh",
            Self::LegacyFlat1 => "legacyFlat1",
            Self::LegacyFlat2 => "legacyFlat2",
            Self::LegacyFlat3 => "legacyFlat3",
            Self::LegacyFlat4 => "legacyFlat4",
            Self::LegacyHarsh1 => "legacyHarsh1",
            Self::LegacyHarsh2 => "legacyHarsh2",
            Self::LegacyHarsh3 => "legacyHarsh3",
            Self::LegacyHarsh4 => "legacyHarsh4",
            Self::LegacyNormal1 => "legacyNormal1",
            Self::LegacyNormal2 => "legacyNormal2",
            Self::LegacyNormal3 => "legacyNormal3",
            Self::LegacyNormal4 => "legacyNormal4",
            Self::Morning => "morning",
            Self::Soft => "soft",
            Self::Sunrise => "sunrise",
            Self::Sunset => "sunset",
            Self::ThreePoints => "threePt",
            Self::TwoPoints => "twoPt",
        }
    }
}
impl FromStr for LightRigValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "balanced" => Ok(Self::Balanced),
            "brightRoom" => Ok(Self::BrightRoom),
            "chilly" => Ok(Self::Chilly),
            "contrasting" => Ok(Self::Contrasting),
            "flat" => Ok(Self::Flat),
            "flood" => Ok(Self::Flood),
            "freezing" => Ok(Self::Freezing),
            "glow" => Ok(Self::Glow),
            "harsh" => Ok(Self::Harsh),
            "legacyFlat1" => Ok(Self::LegacyFlat1),
            "legacyFlat2" => Ok(Self::LegacyFlat2),
            "legacyFlat3" => Ok(Self::LegacyFlat3),
            "legacyFlat4" => Ok(Self::LegacyFlat4),
            "legacyHarsh1" => Ok(Self::LegacyHarsh1),
            "legacyHarsh2" => Ok(Self::LegacyHarsh2),
            "legacyHarsh3" => Ok(Self::LegacyHarsh3),
            "legacyHarsh4" => Ok(Self::LegacyHarsh4),
            "legacyNormal1" => Ok(Self::LegacyNormal1),
            "legacyNormal2" => Ok(Self::LegacyNormal2),
            "legacyNormal3" => Ok(Self::LegacyNormal3),
            "legacyNormal4" => Ok(Self::LegacyNormal4),
            "morning" => Ok(Self::Morning),
            "soft" => Ok(Self::Soft),
            "sunrise" => Ok(Self::Sunrise),
            "sunset" => Ok(Self::Sunset),
            "threePt" => Ok(Self::ThreePoints),
            "twoPt" => Ok(Self::TwoPoints),
            _ => Err(()),
        }
    }
}
