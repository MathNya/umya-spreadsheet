use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum PresetMaterialTypeValues {
    Clear,
    DarkEdge,
    Flat,
    LegacyMatte,
    LegacyMetal,
    LegacyPlastic,
    LegacyWireframe,
    Matte,
    Metal,
    Plastic,
    Powder,
    SoftEdge,
    SoftMetal,
    TranslucentPowder,
    WarmMatte,
}
impl Default for PresetMaterialTypeValues {
    fn default() -> Self {
        Self::LegacyMatte
    }
}
impl EnumTrait for PresetMaterialTypeValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Clear => "clear",
            Self::DarkEdge => "dkEdge",
            Self::Flat => "flat",
            Self::LegacyMatte => "legacyMatte",
            Self::LegacyMetal => "legacyMetal",
            Self::LegacyPlastic => "legacyPlastic",
            Self::LegacyWireframe => "legacyWireframe",
            Self::Matte => "matte",
            Self::Metal => "metal",
            Self::Plastic => "plastic",
            Self::Powder => "powder",
            Self::SoftEdge => "softEdge",
            Self::SoftMetal => "softmetal",
            Self::TranslucentPowder => "translucentPowder",
            Self::WarmMatte => "warmMatte",
        }
    }
}
impl FromStr for PresetMaterialTypeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "clear" => Ok(Self::Clear),
            "dkEdge" => Ok(Self::DarkEdge),
            "flat" => Ok(Self::Flat),
            "legacyMatte" => Ok(Self::LegacyMatte),
            "legacyMetal" => Ok(Self::LegacyMetal),
            "legacyPlastic" => Ok(Self::LegacyPlastic),
            "legacyWireframe" => Ok(Self::LegacyWireframe),
            "matte" => Ok(Self::Matte),
            "metal" => Ok(Self::Metal),
            "plastic" => Ok(Self::Plastic),
            "powder" => Ok(Self::Powder),
            "softEdge" => Ok(Self::SoftEdge),
            "softmetal" => Ok(Self::SoftMetal),
            "translucentPowder" => Ok(Self::TranslucentPowder),
            "warmMatte" => Ok(Self::WarmMatte),
            _ => Err(()),
        }
    }
}
