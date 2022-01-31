use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum ClipboardFormatValues {
    Bitmap,
    Picture,
    PictureOld,
    PicturePrint,
    PictureScreen,
}
impl Default for ClipboardFormatValues {
    fn default() -> Self {
        Self::PictureOld
    }
}
impl EnumTrait for ClipboardFormatValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bitmap => "Bitmap",
            Self::Picture => "Pict",
            Self::PictureOld => "PictOld",
            Self::PicturePrint => "PictPrint",
            Self::PictureScreen => "PictScreen",
        }
    }
}
impl FromStr for ClipboardFormatValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Bitmap" => Ok(Self::Bitmap),
            "Pict" => Ok(Self::Picture),
            "PictOld" => Ok(Self::PictureOld),
            "PictPrint" => Ok(Self::PicturePrint),
            "PictScreen" => Ok(Self::PictureScreen),
            _ => Err(()),
        }
    }
}
