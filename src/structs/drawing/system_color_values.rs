use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum SystemColorValues {
    ActiveBorder,
    ActiveCaption,
    ApplicationWorkspace,
    Background,
    ButtonFace,
    ButtonHighlight,
    ButtonShadow,
    ButtonText,
    CaptionText,
    GradientActiveCaption,
    GradientInactiveCaption,
    GrayText,
    Highlight,
    HighlightText,
    HotLight,
    InactiveBorder,
    InactiveCaption,
    InactiveCaptionText,
    InfoBack,
    InfoText,
    Menu,
    MenuBar,
    MenuHighlight,
    MenuText,
    ScrollBar,
    ThreeDDarkShadow,
    ThreeDLight,
    Window,
    WindowFrame,
    WindowText,
}
impl Default for SystemColorValues {
    fn default() -> Self {
        Self::ScrollBar
    }
}
impl EnumTrait for SystemColorValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::ActiveBorder => "activeBorder",
            Self::ActiveCaption => "activeCaption",
            Self::ApplicationWorkspace => "appWorkspace",
            Self::Background => "background",
            Self::ButtonFace => "btnFace",
            Self::ButtonHighlight => "btnHighlight",
            Self::ButtonShadow => "btnShadow",
            Self::ButtonText => "btnText",
            Self::CaptionText => "captionText",
            Self::GradientActiveCaption => "gradientActiveCaption",
            Self::GradientInactiveCaption => "gradientInactiveCaption",
            Self::GrayText => "grayText",
            Self::Highlight => "highlight",
            Self::HighlightText => "highlightText",
            Self::HotLight => "hotLight",
            Self::InactiveBorder => "inactiveBorder",
            Self::InactiveCaption => "inactiveCaption",
            Self::InactiveCaptionText => "inactiveCaptionText",
            Self::InfoBack => "infoBk",
            Self::InfoText => "infoText",
            Self::Menu => "menu",
            Self::MenuBar => "menuBar",
            Self::MenuHighlight => "menuHighlight",
            Self::MenuText => "menuText",
            Self::ScrollBar => "scrollBar",
            Self::ThreeDDarkShadow => "3dDkShadow",
            Self::ThreeDLight => "3dLight",
            Self::Window => "window",
            Self::WindowFrame => "windowFrame",
            Self::WindowText => "windowText",
        }
    }
}
impl FromStr for SystemColorValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "activeBorder" => Ok(Self::ActiveBorder),
            "activeCaption" => Ok(Self::ActiveCaption),
            "appWorkspace" => Ok(Self::ApplicationWorkspace),
            "background" => Ok(Self::Background),
            "btnFace" => Ok(Self::ButtonFace),
            "btnHighlight" => Ok(Self::ButtonHighlight),
            "btnShadow" => Ok(Self::ButtonShadow),
            "btnText" => Ok(Self::ButtonText),
            "captionText" => Ok(Self::CaptionText),
            "gradientActiveCaption" => Ok(Self::GradientActiveCaption),
            "gradientInactiveCaption" => Ok(Self::GradientInactiveCaption),
            "grayText" => Ok(Self::GrayText),
            "highlight" => Ok(Self::Highlight),
            "highlightText" => Ok(Self::HighlightText),
            "hotLight" => Ok(Self::HotLight),
            "inactiveBorder" => Ok(Self::InactiveBorder),
            "inactiveCaption" => Ok(Self::InactiveCaption),
            "inactiveCaptionText" => Ok(Self::InactiveCaptionText),
            "infoBk" => Ok(Self::InfoBack),
            "infoText" => Ok(Self::InfoText),
            "menu" => Ok(Self::Menu),
            "menuBar" => Ok(Self::MenuBar),
            "menuHighlight" => Ok(Self::MenuHighlight),
            "menuText" => Ok(Self::MenuText),
            "scrollBar" => Ok(Self::ScrollBar),
            "3dDkShadow" => Ok(Self::ThreeDDarkShadow),
            "3dLight" => Ok(Self::ThreeDLight),
            "window" => Ok(Self::Window),
            "windowFrame" => Ok(Self::WindowFrame),
            "windowText" => Ok(Self::WindowText),
            _ => Err(()),
        }
    }
}
