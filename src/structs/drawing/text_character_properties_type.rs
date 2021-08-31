use super::TextCapsValues;
use super::SolidFill;
use super::Outline;
use super::LatinFont;
use super::EastAsianFont;
use super::GradientFill;
use super::NoFill;
use super::EffectList;

pub trait TextCharacterPropertiesType {
    fn get_text(&self) -> &str;

    fn set_text<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_kumimoji(&self) -> &Option<String>;

    fn set_kumimoji<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_language(&self) -> &Option<String>;

    fn set_language<S: Into<String>>(&mut self, value:S) -> &mut Self;
    
    fn get_alternative_language(&self) -> &Option<String>;

    fn set_alternative_language<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_bold(&self) -> &Option<String>;

    fn set_bold<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_sz(&self) -> &Option<String>;

    fn set_sz<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_italic(&self) -> &Option<String>;

    fn set_italic<S: Into<String>>(&mut self, value:S) -> &mut Self;

    fn get_capital(&self) -> &TextCapsValues;

    fn set_capital(&mut self, value:TextCapsValues) -> &mut Self;

    fn get_spacing(&self) -> &i32;

    fn set_spacing(&mut self, value:i32) -> &mut Self;

    fn get_solid_fill(&self) -> &Option<SolidFill>;

    fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill>;
    
    fn set_solid_fill(&mut self, value:SolidFill) -> &mut Self;

    fn get_outline(&self) -> &Option<Outline>;

    fn get_outline_mut(&mut self) -> &mut Option<Outline>;

    fn set_outline(&mut self, value:Outline) -> &mut Self;

    fn get_latin_font(&self) -> &Option<LatinFont>;

    fn get_latin_font_mut(&mut self) -> &mut Option<LatinFont>;
    
    fn set_latin_font(&mut self, value:LatinFont) -> &mut Self;

    fn get_east_asian_font(&self) -> &Option<EastAsianFont>;

    fn get_east_asian_font_mut(&mut self) -> &mut Option<EastAsianFont>;
    
    fn set_east_asian_font(&mut self, value:EastAsianFont) -> &mut Self;

    fn get_gradient_fill(&self) -> &Option<GradientFill>;

    fn get_gradient_fill_mut(&mut self) -> &mut Option<GradientFill>;
    
    fn set_gradient_fill(&mut self, value:GradientFill) -> &mut Self;

    fn get_no_fill(&self) -> &Option<NoFill>;

    fn get_no_fill_mut(&mut self) -> &mut Option<NoFill>;
    
    fn set_no_fill(&mut self, value:NoFill) -> &mut Self;

    fn get_effect_list(&self) -> &Option<EffectList>;

    fn get_effect_list_mut(&mut self) -> &mut Option<EffectList>;
    
    fn set_effect_list(&mut self, value:EffectList) -> &mut Self;
}
