use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum ObjectValues {
    AuditingLine,
    AuditingRectangle,
    Button,
    Checkbox,
    Dialog,
    Drop,
    Edit,
    Group,
    GroupBox,
    Label,
    List,
    Movie,
    Note,
    Picture,
    Radio,
    Rectangle,
    Scroll,
    Shape,
    Spin,
}
impl Default for ObjectValues {
    fn default() -> Self {
        Self::Button
    }
}
impl EnumTrait for ObjectValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::AuditingLine => "LineA",
            Self::AuditingRectangle => "RectA",
            Self::Button => "Button",
            Self::Checkbox => "Checkbox",
            Self::Dialog => "Dialog",
            Self::Drop => "Drop",
            Self::Edit => "Edit",
            Self::Group => "Group",
            Self::GroupBox => "GBox",
            Self::Label => "Label",
            Self::List => "List",
            Self::Movie => "Movie",
            Self::Note => "Note",
            Self::Picture => "Pict",
            Self::Radio => "Radio",
            Self::Rectangle => "Rect",
            Self::Scroll => "Scroll",
            Self::Shape => "Shape",
            Self::Spin => "Spin",
        }
    }
}
impl FromStr for ObjectValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "LineA" => Ok(Self::AuditingLine),
            "RectA" => Ok(Self::AuditingRectangle),
            "Button" => Ok(Self::Button),
            "Checkbox" => Ok(Self::Checkbox),
            "Dialog" => Ok(Self::Dialog),
            "Drop" => Ok(Self::Drop),
            "Edit" => Ok(Self::Edit),
            "Group" => Ok(Self::Group),
            "GBox" => Ok(Self::GroupBox),
            "Label" => Ok(Self::Label),
            "List" => Ok(Self::List),
            "Movie" => Ok(Self::Movie),
            "Note" => Ok(Self::Note),
            "Pict" => Ok(Self::Picture),
            "Radio" => Ok(Self::Radio),
            "Rect" => Ok(Self::Rectangle),
            "Scroll" => Ok(Self::Scroll),
            "Shape" => Ok(Self::Shape),
            "Spin" => Ok(Self::Spin),
            _ => Err(()),
        }
    }
}
