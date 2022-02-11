use structs::Alignment;
use structs::Borders;
use structs::Fill;
use structs::Font;
use structs::NumberingFormat;
use structs::UInt32Value;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Style {
    font: Option<Font>,
    fill: Option<Fill>,
    borders: Option<Borders>,
    alignment: Option<Alignment>,
    numbering_format: Option<NumberingFormat>,
    format_id: UInt32Value,
}
impl Style {
    pub fn get_font(&self) -> &Option<Font> {
        &self.font
    }

    pub fn get_font_mut(&mut self) -> &mut Font {
        match &self.font {
            Some(_) => return self.font.as_mut().unwrap(),
            None => {}
        }
        self.set_font(Font::get_defalut_value());
        self.font.as_mut().unwrap()
    }

    pub fn set_font(&mut self, value: Font) -> &mut Self {
        self.font = Some(value);
        self
    }

    pub(crate) fn set_font_crate(&mut self, value: Option<Font>) -> &mut Self {
        self.font = value;
        self
    }

    pub fn get_fill(&self) -> &Option<Fill> {
        &self.fill
    }

    pub fn get_fill_mut(&mut self) -> &mut Fill {
        match &self.fill {
            Some(_) => return self.fill.as_mut().unwrap(),
            None => {}
        }
        self.set_fill(Fill::get_defalut_value());
        self.fill.as_mut().unwrap()
    }

    pub fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub(crate) fn set_fill_crate(&mut self, value: Option<Fill>) -> &mut Self {
        self.fill = value;
        self
    }

    pub fn get_borders(&self) -> &Option<Borders> {
        &self.borders
    }

    pub fn get_borders_mut(&mut self) -> &mut Borders {
        match &self.borders {
            Some(_) => return self.borders.as_mut().unwrap(),
            None => {}
        }
        self.set_borders(Borders::get_defalut_value());
        self.borders.as_mut().unwrap()
    }

    pub fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders = Some(value);
        self
    }

    pub(crate) fn set_borders_crate(&mut self, value: Option<Borders>) -> &mut Self {
        self.borders = value;
        self
    }

    pub fn get_alignment(&self) -> &Option<Alignment> {
        &self.alignment
    }

    pub fn get_alignment_mut(&mut self) -> &mut Alignment {
        match &self.alignment {
            Some(_) => return self.alignment.as_mut().unwrap(),
            None => {}
        }
        self.set_alignment(Alignment::default());
        self.alignment.as_mut().unwrap()
    }

    pub fn set_alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    pub(crate) fn set_alignment_crate(&mut self, value: Option<Alignment>) -> &mut Self {
        self.alignment = value;
        self
    }

    pub fn get_numbering_format(&self) -> &Option<NumberingFormat> {
        &self.numbering_format
    }

    pub fn get_numbering_format_mut(&mut self) -> &mut NumberingFormat {
        match &self.numbering_format {
            Some(_) => return self.numbering_format.as_mut().unwrap(),
            None => {}
        }
        self.set_numbering_format(NumberingFormat::default());
        self.numbering_format.as_mut().unwrap()
    }

    pub(crate) fn _get_numbering_format_mut_crate(&mut self) -> &mut Option<NumberingFormat> {
        &mut self.numbering_format
    }

    pub fn set_numbering_format(&mut self, value: NumberingFormat) -> &mut Self {
        self.numbering_format = Some(value);
        self
    }

    pub fn get_number_format(&self) -> &Option<NumberingFormat> {
        &self.get_numbering_format()
    }

    pub fn get_number_format_mut(&mut self) -> &mut NumberingFormat {
        self.get_numbering_format_mut()
    }

    pub fn set_number_format(&mut self, value: NumberingFormat) -> &mut Self {
        self.set_numbering_format(value)
    }

    pub fn get_format_id(&self) -> &u32 {
        self.format_id.get_value()
    }

    pub fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    pub(crate) fn _get_defalut_value() -> Self {
        let def = Self::default();
        def
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::compute(format!(
                "{}{}{}{}{}",
                match &self.font {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.borders {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.alignment {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.numbering_format {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
            ))
        )
    }

    pub(crate) fn _is_empty(&self) -> bool {
        self.get_hash_code() == Self::_get_defalut_value().get_hash_code()
    }
}
