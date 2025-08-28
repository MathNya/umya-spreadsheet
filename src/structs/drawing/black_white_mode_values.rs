use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum BlackWhiteModeValues {
    Auto,
    Black,
    BlackGray,
    BlackWhite,
    Color,
    Gray,
    GrayWhite,
    Hidden,
    InvGray,
    LightGray,
    White,
}
impl Default for BlackWhiteModeValues {
    #[inline]
    fn default() -> Self {
        Self::Auto
    }
}
impl EnumTrait for BlackWhiteModeValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Auto => "auto",
            Self::Black => "black",
            Self::BlackGray => "blackGray",
            Self::BlackWhite => "blackWhite",
            Self::Color => "clr",
            Self::Gray => "gray",
            Self::GrayWhite => "grayWhite",
            Self::Hidden => "hidden",
            Self::InvGray => "invGray",
            Self::LightGray => "ltGray",
            Self::White => "white",
        }
    }
}
impl FromStr for BlackWhiteModeValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "auto" => Ok(Self::Auto),
            "black" => Ok(Self::Black),
            "blackGray" => Ok(Self::BlackGray),
            "blackWhite" => Ok(Self::BlackWhite),
            "clr" => Ok(Self::Color),
            "gray" => Ok(Self::Gray),
            "grayWhite" => Ok(Self::GrayWhite),
            "hidden" => Ok(Self::Hidden),
            "invGray" => Ok(Self::InvGray),
            "ltGray" => Ok(Self::LightGray),
            "white" => Ok(Self::White),
            _ => Err(()),
        }
    }
}
