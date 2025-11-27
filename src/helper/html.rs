use std::collections::HashMap;

use html_parser::{
    Dom,
    Node,
};
use phf::phf_map;

use crate::structs::{
    Color,
    Font,
    RichText,
    TextElement,
    UnderlineValues,
    VerticalAlignmentRunValues,
};

/// Generate rich text from html.
/// # Arguments
/// * `html` - HTML String.
/// # Return value
/// * `Result<RichText, html_parser::Error>`
/// # Examples
/// ```
/// let html = r##"<font color="red">test</font><br><font class="test" color="#48D1CC">TE<b>S</b>T<br/>TEST</font>"##;
/// let richtext = umya_spreadsheet::helper::html::html_to_richtext(html).unwrap();
///
/// let mut book = umya_spreadsheet::new_file();
/// let mut sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// sheet.get_cell_mut("A1").set_rich_text(richtext);
/// // Enable line breaks.
/// sheet
///     .get_cell_mut("A1")
///     .get_style_mut()
///     .get_alignment_mut()
///     .set_wrap_text(true);
/// ```
#[inline]
pub fn html_to_richtext(html: &str) -> Result<RichText, html_parser::Error> {
    html_to_richtext_custom(html, &DataAnalysis::default())
}

/// Use here for custom html parsing.
/// # Arguments
/// * `html` - HTML String.
/// * `method` - struct for analysis.
/// # Return value
/// * `Result<RichText, html_parser::Error>`
#[inline]
pub fn html_to_richtext_custom(
    html: &str,
    method: &dyn AnalysisMethod,
) -> Result<RichText, html_parser::Error> {
    let dom = Dom::parse(html)?;
    let data = read_node(&dom.children, &Vec::new());
    let result = make_rich_text(&data, method);
    Ok(result)
}

#[allow(clippy::field_reassign_with_default)]
fn read_node(node_list: &Vec<Node>, parent_element: &[HfdElement]) -> Vec<HtmlFlatData> {
    let mut result: Vec<HtmlFlatData> = Vec::new();

    if node_list.is_empty() {
        return result;
    }

    let mut data = HtmlFlatData::default();
    data.element.extend_from_slice(parent_element);

    for node in node_list {
        match node {
            Node::Text(text) => {
                data.text = format!("{}{}", data.text, text);
            }
            Node::Element(element) => {
                if &element.name == "br" {
                    data.text = format!("{}{}", data.text, "\n");
                    continue;
                }
                if !data.text.is_empty() {
                    result.push(data);
                    data = HtmlFlatData::default();
                    data.element.extend_from_slice(parent_element);
                }

                let mut elm: HfdElement = HfdElement::default();
                elm.name.clone_from(&element.name);

                elm.attributes = element
                    .attributes
                    .iter()
                    .map(|(name, value)| {
                        (
                            name.clone(),
                            value.as_ref().map(ToString::to_string).unwrap_or_default(),
                        )
                    })
                    .collect();

                elm.classes.clone_from(&element.classes);
                data.element.push(elm);

                let mut children = read_node(&element.children, &data.element);
                result.append(&mut children);

                data = HtmlFlatData::default();
                data.element.extend_from_slice(parent_element);
            }
            Node::Comment(_) => {}
        }
    }
    if !data.text.is_empty() {
        result.push(data);
    }
    result
}

fn make_rich_text(html_flat_data_list: &[HtmlFlatData], method: &dyn AnalysisMethod) -> RichText {
    let mut result = RichText::default();

    for html_flat_data in html_flat_data_list {
        let font_name: Option<&str> = method.font_name(html_flat_data);
        let size: Option<f64> = method.size(html_flat_data);
        let color: Option<String> = method.color(html_flat_data);
        let is_bold: bool = method.is_bold(html_flat_data);
        let is_italic: bool = method.is_italic(html_flat_data);
        let is_underline: bool = method.is_underline(html_flat_data);
        let is_superscript: bool = method.is_superscript(html_flat_data);
        let is_subscript: bool = method.is_subscript(html_flat_data);
        let is_strikethrough: bool = method.is_strikethrough(html_flat_data);

        let mut text_element = TextElement::default();
        let mut font = Font::default();

        if let Some(v) = font_name {
            font.set_name(v);
        }

        if let Some(v) = size {
            font.set_size(v);
        }

        if let Some(v) = color {
            let argb = v;
            let mut clr = Color::default();
            clr.set_argb_str(argb);
            font.set_color(clr);
        }

        if is_bold {
            font.set_bold(is_bold);
        }
        if is_italic {
            font.set_italic(is_italic);
        }
        if is_underline {
            font.font_underline_mut().set_val(UnderlineValues::Single);
        }
        if is_superscript {
            font.vertical_text_alignment_mut()
                .set_val(VerticalAlignmentRunValues::Superscript);
        }
        if is_subscript {
            font.vertical_text_alignment_mut()
                .set_val(VerticalAlignmentRunValues::Subscript);
        }
        if is_strikethrough {
            font.set_strikethrough(is_strikethrough);
        }

        text_element.set_text(&html_flat_data.text);
        text_element.set_run_properties(font);
        result.add_rich_text_elements(text_element);
    }
    result
}

#[derive(Clone, Default, Debug)]
pub struct HtmlFlatData {
    text:    String,
    element: Vec<HfdElement>,
}

#[derive(Clone, Default, Debug)]
pub struct HfdElement {
    name:       String,
    attributes: HashMap<String, String>,
    classes:    Vec<String>,
}
impl HfdElement {
    #[inline]
    #[must_use]
    pub fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    #[inline]
    #[must_use]
    pub fn get_by_name_and_attribute(&self, name: &str, attribute: &str) -> Option<&str> {
        self.attributes
            .get(attribute)
            .and_then(|v| (self.name == name).then_some(v))
            .map(String::as_str)
    }

    #[inline]
    #[must_use]
    pub fn contains_class(&self, class: &str) -> bool {
        self.classes.contains(&class.to_string())
    }
}

pub trait AnalysisMethod {
    fn is_tag(&self, html_flat_data: &HtmlFlatData, tag: &str) -> bool;
    fn font_name<'a>(&'a self, html_flat_data: &'a HtmlFlatData) -> Option<&'a str>;
    fn size(&self, html_flat_data: &HtmlFlatData) -> Option<f64>;
    fn color(&self, html_flat_data: &HtmlFlatData) -> Option<String>;
    fn is_bold(&self, html_flat_data: &HtmlFlatData) -> bool;
    fn is_italic(&self, html_flat_data: &HtmlFlatData) -> bool;
    fn is_underline(&self, html_flat_data: &HtmlFlatData) -> bool;
    fn is_superscript(&self, html_flat_data: &HtmlFlatData) -> bool;
    fn is_subscript(&self, html_flat_data: &HtmlFlatData) -> bool;
    fn is_strikethrough(&self, html_flat_data: &HtmlFlatData) -> bool;
}

#[derive(Clone, Default, Debug)]
struct DataAnalysis {}
impl AnalysisMethod for DataAnalysis {
    #[inline]
    fn font_name<'a>(&'a self, html_flat_data: &'a HtmlFlatData) -> Option<&'a str> {
        html_flat_data
            .element
            .iter()
            .find_map(|element| element.get_by_name_and_attribute("font", "face"))
    }

    #[inline]
    fn size(&self, html_flat_data: &HtmlFlatData) -> Option<f64> {
        html_flat_data.element.iter().find_map(|element| {
            element
                .get_by_name_and_attribute("font", "size")
                .and_then(|v| v.parse::<f64>().ok())
        })
    }

    fn color(&self, html_flat_data: &HtmlFlatData) -> Option<String> {
        html_flat_data
            .element
            .iter()
            .filter_map(|element| element.get_by_name_and_attribute("font", "color"))
            .find_map(|v| {
                let color = v.trim_start_matches('#').to_uppercase();
                COLOR_MAP
                    .get(&color)
                    .map_or(Some(color), |v| Some((*v).to_string()))
            })
    }

    #[inline]
    fn is_tag(&self, html_flat_data: &HtmlFlatData, tag: &str) -> bool {
        html_flat_data
            .element
            .iter()
            .any(|element| element.has_name(tag))
    }

    #[inline]
    fn is_bold(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "b") || self.is_tag(html_flat_data, "strong")
    }

    #[inline]
    fn is_italic(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "i") || self.is_tag(html_flat_data, "em")
    }

    #[inline]
    fn is_underline(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "u") || self.is_tag(html_flat_data, "ins")
    }

    #[inline]
    fn is_superscript(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "sup")
    }

    #[inline]
    fn is_subscript(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "sub")
    }

    #[inline]
    fn is_strikethrough(&self, html_flat_data: &HtmlFlatData) -> bool {
        self.is_tag(html_flat_data, "del")
    }
}

static COLOR_MAP: phf::Map<&str, &str> = phf_map! {
    "ALICEBLUE" => "F0F8FF",
    "ANTIQUEWHITE" => "FAEBD7",
    "ANTIQUEWHITE1" => "FFEFDB",
    "ANTIQUEWHITE2" => "EEDFCC",
    "ANTIQUEWHITE3" => "CDC0B0",
    "ANTIQUEWHITE4" => "8B8378",
    "AQUA" => "00FFFF",
    "AQUAMARINE1" => "7FFFD4",
    "AQUAMARINE2" => "76EEC6",
    "AQUAMARINE4" => "458B74",
    "AZURE1" => "F0FFFF",
    "AZURE2" => "E0EEEE",
    "AZURE3" => "C1CDCD",
    "AZURE4" => "838B8B",
    "BEIGE" => "F5F5DC",
    "BISQUE1" => "FFE4C4",
    "BISQUE2" => "EED5B7",
    "BISQUE3" => "CDB79E",
    "BISQUE4" => "8B7D6B",
    "BLACK" => "000000",
    "BLANCHEDALMOND" => "FFEBCD",
    "BLUE" => "0000FF",
    "BLUE1" => "0000FF",
    "BLUE2" => "0000EE",
    "BLUE4" => "00008B",
    "BLUEVIOLET" => "8A2BE2",
    "BROWN" => "A52A2A",
    "BROWN1" => "FF4040",
    "BROWN2" => "EE3B3B",
    "BROWN3" => "CD3333",
    "BROWN4" => "8B2323",
    "BURLYWOOD" => "DEB887",
    "BURLYWOOD1" => "FFD39B",
    "BURLYWOOD2" => "EEC591",
    "BURLYWOOD3" => "CDAA7D",
    "BURLYWOOD4" => "8B7355",
    "CADETBLUE" => "5F9EA0",
    "CADETBLUE1" => "98F5FF",
    "CADETBLUE2" => "8EE5EE",
    "CADETBLUE3" => "7AC5CD",
    "CADETBLUE4" => "53868B",
    "CHARTREUSE1" => "7FFF00",
    "CHARTREUSE2" => "76EE00",
    "CHARTREUSE3" => "66CD00",
    "CHARTREUSE4" => "458B00",
    "CHOCOLATE" => "D2691E",
    "CHOCOLATE1" => "FF7F24",
    "CHOCOLATE2" => "EE7621",
    "CHOCOLATE3" => "CD661D",
    "CORAL" => "FF7F50",
    "CORAL1" => "FF7256",
    "CORAL2" => "EE6A50",
    "CORAL3" => "CD5B45",
    "CORAL4" => "8B3E2F",
    "CORNFLOWERBLUE" => "6495ED",
    "CORNSILK1" => "FFF8DC",
    "CORNSILK2" => "EEE8CD",
    "CORNSILK3" => "CDC8B1",
    "CORNSILK4" => "8B8878",
    "CYAN1" => "00FFFF",
    "CYAN2" => "00EEEE",
    "CYAN3" => "00CDCD",
    "CYAN4" => "008B8B",
    "DARKGOLDENROD" => "B8860B",
    "DARKGOLDENROD1" => "FFB90F",
    "DARKGOLDENROD2" => "EEAD0E",
    "DARKGOLDENROD3" => "CD950C",
    "DARKGOLDENROD4" => "8B6508",
    "DARKGREEN" => "006400",
    "DARKKHAKI" => "BDB76B",
    "DARKOLIVEGREEN" => "556B2F",
    "DARKOLIVEGREEN1" => "CAFF70",
    "DARKOLIVEGREEN2" => "BCEE68",
    "DARKOLIVEGREEN3" => "A2CD5A",
    "DARKOLIVEGREEN4" => "6E8B3D",
    "DARKORANGE" => "FF8C00",
    "DARKORANGE1" => "FF7F00",
    "DARKORANGE2" => "EE7600",
    "DARKORANGE3" => "CD6600",
    "DARKORANGE4" => "8B4500",
    "DARKORCHID" => "9932CC",
    "DARKORCHID1" => "BF3EFF",
    "DARKORCHID2" => "B23AEE",
    "DARKORCHID3" => "9A32CD",
    "DARKORCHID4" => "68228B",
    "DARKSALMON" => "E9967A",
    "DARKSEAGREEN" => "8FBC8F",
    "DARKSEAGREEN1" => "C1FFC1",
    "DARKSEAGREEN2" => "B4EEB4",
    "DARKSEAGREEN3" => "9BCD9B",
    "DARKSEAGREEN4" => "698B69",
    "DARKSLATEBLUE" => "483D8B",
    "DARKSLATEGRAY" => "2F4F4F",
    "DARKSLATEGRAY1" => "97FFFF",
    "DARKSLATEGRAY2" => "8DEEEE",
    "DARKSLATEGRAY3" => "79CDCD",
    "DARKSLATEGRAY4" => "528B8B",
    "DARKTURQUOISE" => "00CED1",
    "DARKVIOLET" => "9400D3",
    "DEEPPINK1" => "FF1493",
    "DEEPPINK2" => "EE1289",
    "DEEPPINK3" => "CD1076",
    "DEEPPINK4" => "8B0A50",
    "DEEPSKYBLUE1" => "00BFFF",
    "DEEPSKYBLUE2" => "00B2EE",
    "DEEPSKYBLUE3" => "009ACD",
    "DEEPSKYBLUE4" => "00688B",
    "DIMGRAY" => "696969",
    "DODGERBLUE1" => "1E90FF",
    "DODGERBLUE2" => "1C86EE",
    "DODGERBLUE3" => "1874CD",
    "DODGERBLUE4" => "104E8B",
    "FIREBRICK" => "B22222",
    "FIREBRICK1" => "FF3030",
    "FIREBRICK2" => "EE2C2C",
    "FIREBRICK3" => "CD2626",
    "FIREBRICK4" => "8B1A1A",
    "FLORALWHITE" => "FFFAF0",
    "FORESTGREEN" => "228B22",
    "FUCHSIA" => "FF00FF",
    "GAINSBORO" => "DCDCDC",
    "GHOSTWHITE" => "F8F8FF",
    "GOLD1" => "FFD700",
    "GOLD2" => "EEC900",
    "GOLD3" => "CDAD00",
    "GOLD4" => "8B7500",
    "GOLDENROD" => "DAA520",
    "GOLDENROD1" => "FFC125",
    "GOLDENROD2" => "EEB422",
    "GOLDENROD3" => "CD9B1D",
    "GOLDENROD4" => "8B6914",
    "GRAY" => "BEBEBE",
    "GRAY1" => "030303",
    "GRAY10" => "1A1A1A",
    "GRAY11" => "1C1C1C",
    "GRAY12" => "1F1F1F",
    "GRAY13" => "212121",
    "GRAY14" => "242424",
    "GRAY15" => "262626",
    "GRAY16" => "292929",
    "GRAY17" => "2B2B2B",
    "GRAY18" => "2E2E2E",
    "GRAY19" => "303030",
    "GRAY2" => "050505",
    "GRAY20" => "333333",
    "GRAY21" => "363636",
    "GRAY22" => "383838",
    "GRAY23" => "3B3B3B",
    "GRAY24" => "3D3D3D",
    "GRAY25" => "404040",
    "GRAY26" => "424242",
    "GRAY27" => "454545",
    "GRAY28" => "474747",
    "GRAY29" => "4A4A4A",
    "GRAY3" => "080808",
    "GRAY30" => "4D4D4D",
    "GRAY31" => "4F4F4F",
    "GRAY32" => "525252",
    "GRAY33" => "545454",
    "GRAY34" => "575757",
    "GRAY35" => "595959",
    "GRAY36" => "5C5C5C",
    "GRAY37" => "5E5E5E",
    "GRAY38" => "616161",
    "GRAY39" => "636363",
    "GRAY4" => "0A0A0A",
    "GRAY40" => "666666",
    "GRAY41" => "696969",
    "GRAY42" => "6B6B6B",
    "GRAY43" => "6E6E6E",
    "GRAY44" => "707070",
    "GRAY45" => "737373",
    "GRAY46" => "757575",
    "GRAY47" => "787878",
    "GRAY48" => "7A7A7A",
    "GRAY49" => "7D7D7D",
    "GRAY5" => "0D0D0D",
    "GRAY50" => "7F7F7F",
    "GRAY51" => "828282",
    "GRAY52" => "858585",
    "GRAY53" => "878787",
    "GRAY54" => "8A8A8A",
    "GRAY55" => "8C8C8C",
    "GRAY56" => "8F8F8F",
    "GRAY57" => "919191",
    "GRAY58" => "949494",
    "GRAY59" => "969696",
    "GRAY6" => "0F0F0F",
    "GRAY60" => "999999",
    "GRAY61" => "9C9C9C",
    "GRAY62" => "9E9E9E",
    "GRAY63" => "A1A1A1",
    "GRAY64" => "A3A3A3",
    "GRAY65" => "A6A6A6",
    "GRAY66" => "A8A8A8",
    "GRAY67" => "ABABAB",
    "GRAY68" => "ADADAD",
    "GRAY69" => "B0B0B0",
    "GRAY7" => "121212",
    "GRAY70" => "B3B3B3",
    "GRAY71" => "B5B5B5",
    "GRAY72" => "B8B8B8",
    "GRAY73" => "BABABA",
    "GRAY74" => "BDBDBD",
    "GRAY75" => "BFBFBF",
    "GRAY76" => "C2C2C2",
    "GRAY77" => "C4C4C4",
    "GRAY78" => "C7C7C7",
    "GRAY79" => "C9C9C9",
    "GRAY8" => "141414",
    "GRAY80" => "CCCCCC",
    "GRAY81" => "CFCFCF",
    "GRAY82" => "D1D1D1",
    "GRAY83" => "D4D4D4",
    "GRAY84" => "D6D6D6",
    "GRAY85" => "D9D9D9",
    "GRAY86" => "DBDBDB",
    "GRAY87" => "DEDEDE",
    "GRAY88" => "E0E0E0",
    "GRAY89" => "E3E3E3",
    "GRAY9" => "171717",
    "GRAY90" => "E5E5E5",
    "GRAY91" => "E8E8E8",
    "GRAY92" => "EBEBEB",
    "GRAY93" => "EDEDED",
    "GRAY94" => "F0F0F0",
    "GRAY95" => "F2F2F2",
    "GRAY97" => "F7F7F7",
    "GRAY98" => "FAFAFA",
    "GRAY99" => "FCFCFC",
    "GREEN" => "00FF00",
    "GREEN1" => "00FF00",
    "GREEN2" => "00EE00",
    "GREEN3" => "00CD00",
    "GREEN4" => "008B00",
    "GREENYELLOW" => "ADFF2F",
    "HONEYDEW1" => "F0FFF0",
    "HONEYDEW2" => "E0EEE0",
    "HONEYDEW3" => "C1CDC1",
    "HONEYDEW4" => "838B83",
    "HOTPINK" => "FF69B4",
    "HOTPINK1" => "FF6EB4",
    "HOTPINK2" => "EE6AA7",
    "HOTPINK3" => "CD6090",
    "HOTPINK4" => "8B3A62",
    "INDIANRED" => "CD5C5C",
    "INDIANRED1" => "FF6A6A",
    "INDIANRED2" => "EE6363",
    "INDIANRED3" => "CD5555",
    "INDIANRED4" => "8B3A3A",
    "IVORY1" => "FFFFF0",
    "IVORY2" => "EEEEE0",
    "IVORY3" => "CDCDC1",
    "IVORY4" => "8B8B83",
    "KHAKI" => "F0E68C",
    "KHAKI1" => "FFF68F",
    "KHAKI2" => "EEE685",
    "KHAKI3" => "CDC673",
    "KHAKI4" => "8B864E",
    "LAVENDER" => "E6E6FA",
    "LAVENDERBLUSH1" => "FFF0F5",
    "LAVENDERBLUSH2" => "EEE0E5",
    "LAVENDERBLUSH3" => "CDC1C5",
    "LAVENDERBLUSH4" => "8B8386",
    "LAWNGREEN" => "7CFC00",
    "LEMONCHIFFON1" => "FFFACD",
    "LEMONCHIFFON2" => "EEE9BF",
    "LEMONCHIFFON3" => "CDC9A5",
    "LEMONCHIFFON4" => "8B8970",
    "LIGHT" => "EEDD82",
    "LIGHTBLUE" => "ADD8E6",
    "LIGHTBLUE1" => "BFEFFF",
    "LIGHTBLUE2" => "B2DFEE",
    "LIGHTBLUE3" => "9AC0CD",
    "LIGHTBLUE4" => "68838B",
    "LIGHTCORAL" => "F08080",
    "LIGHTCYAN1" => "E0FFFF",
    "LIGHTCYAN2" => "D1EEEE",
    "LIGHTCYAN3" => "B4CDCD",
    "LIGHTCYAN4" => "7A8B8B",
    "LIGHTGOLDENROD1" => "FFEC8B",
    "LIGHTGOLDENROD2" => "EEDC82",
    "LIGHTGOLDENROD3" => "CDBE70",
    "LIGHTGOLDENROD4" => "8B814C",
    "LIGHTGOLDENRODYELLOW" => "FAFAD2",
    "LIGHTGRAY" => "D3D3D3",
    "LIGHTPINK" => "FFB6C1",
    "LIGHTPINK1" => "FFAEB9",
    "LIGHTPINK2" => "EEA2AD",
    "LIGHTPINK3" => "CD8C95",
    "LIGHTPINK4" => "8B5F65",
    "LIGHTSALMON1" => "FFA07A",
    "LIGHTSALMON2" => "EE9572",
    "LIGHTSALMON3" => "CD8162",
    "LIGHTSALMON4" => "8B5742",
    "LIGHTSEAGREEN" => "20B2AA",
    "LIGHTSKYBLUE" => "87CEFA",
    "LIGHTSKYBLUE1" => "B0E2FF",
    "LIGHTSKYBLUE2" => "A4D3EE",
    "LIGHTSKYBLUE3" => "8DB6CD",
    "LIGHTSKYBLUE4" => "607B8B",
    "LIGHTSLATEBLUE" => "8470FF",
    "LIGHTSLATEGRAY" => "778899",
    "LIGHTSTEELBLUE" => "B0C4DE",
    "LIGHTSTEELBLUE1" => "CAE1FF",
    "LIGHTSTEELBLUE2" => "BCD2EE",
    "LIGHTSTEELBLUE3" => "A2B5CD",
    "LIGHTSTEELBLUE4" => "6E7B8B",
    "LIGHTYELLOW1" => "FFFFE0",
    "LIGHTYELLOW2" => "EEEED1",
    "LIGHTYELLOW3" => "CDCDB4",
    "LIGHTYELLOW4" => "8B8B7A",
    "LIME" => "00FF00",
    "LIMEGREEN" => "32CD32",
    "LINEN" => "FAF0E6",
    "MAGENTA" => "FF00FF",
    "MAGENTA2" => "EE00EE",
    "MAGENTA3" => "CD00CD",
    "MAGENTA4" => "8B008B",
    "MAROON" => "B03060",
    "MAROON1" => "FF34B3",
    "MAROON2" => "EE30A7",
    "MAROON3" => "CD2990",
    "MAROON4" => "8B1C62",
    "MEDIUM" => "66CDAA",
    "MEDIUMAQUAMARINE" => "66CDAA",
    "MEDIUMBLUE" => "0000CD",
    "MEDIUMORCHID" => "BA55D3",
    "MEDIUMORCHID1" => "E066FF",
    "MEDIUMORCHID2" => "D15FEE",
    "MEDIUMORCHID3" => "B452CD",
    "MEDIUMORCHID4" => "7A378B",
    "MEDIUMPURPLE" => "9370DB",
    "MEDIUMPURPLE1" => "AB82FF",
    "MEDIUMPURPLE2" => "9F79EE",
    "MEDIUMPURPLE3" => "8968CD",
    "MEDIUMPURPLE4" => "5D478B",
    "MEDIUMSEAGREEN" => "3CB371",
    "MEDIUMSLATEBLUE" => "7B68EE",
    "MEDIUMSPRINGGREEN" => "00FA9A",
    "MEDIUMTURQUOISE" => "48D1CC",
    "MEDIUMVIOLETRED" => "C71585",
    "MIDNIGHTBLUE" => "191970",
    "MINTCREAM" => "F5FFFA",
    "MISTYROSE1" => "FFE4E1",
    "MISTYROSE2" => "EED5D2",
    "MISTYROSE3" => "CDB7B5",
    "MISTYROSE4" => "8B7D7B",
    "MOCCASIN" => "FFE4B5",
    "NAVAJOWHITE1" => "FFDEAD",
    "NAVAJOWHITE2" => "EECFA1",
    "NAVAJOWHITE3" => "CDB38B",
    "NAVAJOWHITE4" => "8B795E",
    "NAVY" => "000080",
    "NAVYBLUE" => "000080",
    "OLDLACE" => "FDF5E6",
    "OLIVE" => "808000",
    "OLIVEDRAB" => "6B8E23",
    "OLIVEDRAB1" => "C0FF3E",
    "OLIVEDRAB2" => "B3EE3A",
    "OLIVEDRAB4" => "698B22",
    "ORANGE" => "FFA500",
    "ORANGE1" => "FFA500",
    "ORANGE2" => "EE9A00",
    "ORANGE3" => "CD8500",
    "ORANGE4" => "8B5A00",
    "ORANGERED1" => "FF4500",
    "ORANGERED2" => "EE4000",
    "ORANGERED3" => "CD3700",
    "ORANGERED4" => "8B2500",
    "ORCHID" => "DA70D6",
    "ORCHID1" => "FF83FA",
    "ORCHID2" => "EE7AE9",
    "ORCHID3" => "CD69C9",
    "ORCHID4" => "8B4789",
    "PALE" => "DB7093",
    "PALEGOLDENROD" => "EEE8AA",
    "PALEGREEN" => "98FB98",
    "PALEGREEN1" => "9AFF9A",
    "PALEGREEN2" => "90EE90",
    "PALEGREEN3" => "7CCD7C",
    "PALEGREEN4" => "548B54",
    "PALETURQUOISE" => "AFEEEE",
    "PALETURQUOISE1" => "BBFFFF",
    "PALETURQUOISE2" => "AEEEEE",
    "PALETURQUOISE3" => "96CDCD",
    "PALETURQUOISE4" => "668B8B",
    "PALEVIOLETRED" => "DB7093",
    "PALEVIOLETRED1" => "FF82AB",
    "PALEVIOLETRED2" => "EE799F",
    "PALEVIOLETRED3" => "CD6889",
    "PALEVIOLETRED4" => "8B475D",
    "PAPAYAWHIP" => "FFEFD5",
    "PEACHPUFF1" => "FFDAB9",
    "PEACHPUFF2" => "EECBAD",
    "PEACHPUFF3" => "CDAF95",
    "PEACHPUFF4" => "8B7765",
    "PINK" => "FFC0CB",
    "PINK1" => "FFB5C5",
    "PINK2" => "EEA9B8",
    "PINK3" => "CD919E",
    "PINK4" => "8B636C",
    "PLUM" => "DDA0DD",
    "PLUM1" => "FFBBFF",
    "PLUM2" => "EEAEEE",
    "PLUM3" => "CD96CD",
    "PLUM4" => "8B668B",
    "POWDERBLUE" => "B0E0E6",
    "PURPLE" => "A020F0",
    "REBECCAPURPLE" => "663399",
    "PURPLE1" => "9B30FF",
    "PURPLE2" => "912CEE",
    "PURPLE3" => "7D26CD",
    "PURPLE4" => "551A8B",
    "RED" => "FF0000",
    "RED1" => "FF0000",
    "RED2" => "EE0000",
    "RED3" => "CD0000",
    "RED4" => "8B0000",
    "ROSYBROWN" => "BC8F8F",
    "ROSYBROWN1" => "FFC1C1",
    "ROSYBROWN2" => "EEB4B4",
    "ROSYBROWN3" => "CD9B9B",
    "ROSYBROWN4" => "8B6969",
    "ROYALBLUE" => "4169E1",
    "ROYALBLUE1" => "4876FF",
    "ROYALBLUE2" => "436EEE",
    "ROYALBLUE3" => "3A5FCD",
    "ROYALBLUE4" => "27408B",
    "SADDLEBROWN" => "8B4513",
    "SALMON" => "FA8072",
    "SALMON1" => "FF8C69",
    "SALMON2" => "EE8262",
    "SALMON3" => "CD7054",
    "SALMON4" => "8B4C39",
    "SANDYBROWN" => "F4A460",
    "SEAGREEN1" => "54FF9F",
    "SEAGREEN2" => "4EEE94",
    "SEAGREEN3" => "43CD80",
    "SEAGREEN4" => "2E8B57",
    "SEASHELL1" => "FFF5EE",
    "SEASHELL2" => "EEE5DE",
    "SEASHELL3" => "CDC5BF",
    "SEASHELL4" => "8B8682",
    "SIENNA" => "A0522D",
    "SIENNA1" => "FF8247",
    "SIENNA2" => "EE7942",
    "SIENNA3" => "CD6839",
    "SIENNA4" => "8B4726",
    "SILVER" => "C0C0C0",
    "SKYBLUE" => "87CEEB",
    "SKYBLUE1" => "87CEFF",
    "SKYBLUE2" => "7EC0EE",
    "SKYBLUE3" => "6CA6CD",
    "SKYBLUE4" => "4A708B",
    "SLATEBLUE" => "6A5ACD",
    "SLATEBLUE1" => "836FFF",
    "SLATEBLUE2" => "7A67EE",
    "SLATEBLUE3" => "6959CD",
    "SLATEBLUE4" => "473C8B",
    "SLATEGRAY" => "708090",
    "SLATEGRAY1" => "C6E2FF",
    "SLATEGRAY2" => "B9D3EE",
    "SLATEGRAY3" => "9FB6CD",
    "SLATEGRAY4" => "6C7B8B",
    "SNOW1" => "FFFAFA",
    "SNOW2" => "EEE9E9",
    "SNOW3" => "CDC9C9",
    "SNOW4" => "8B8989",
    "SPRINGGREEN1" => "00FF7F",
    "SPRINGGREEN2" => "00EE76",
    "SPRINGGREEN3" => "00CD66",
    "SPRINGGREEN4" => "008B45",
    "STEELBLUE" => "4682B4",
    "STEELBLUE1" => "63B8FF",
    "STEELBLUE2" => "5CACEE",
    "STEELBLUE3" => "4F94CD",
    "STEELBLUE4" => "36648B",
    "TAN" => "D2B48C",
    "TAN1" => "FFA54F",
    "TAN2" => "EE9A49",
    "TAN3" => "CD853F",
    "TAN4" => "8B5A2B",
    "TEAL" => "008080",
    "THISTLE" => "D8BFD8",
    "THISTLE1" => "FFE1FF",
    "THISTLE2" => "EED2EE",
    "THISTLE3" => "CDB5CD",
    "THISTLE4" => "8B7B8B",
    "TOMATO1" => "FF6347",
    "TOMATO2" => "EE5C42",
    "TOMATO3" => "CD4F39",
    "TOMATO4" => "8B3626",
    "TURQUOISE" => "40E0D0",
    "TURQUOISE1" => "00F5FF",
    "TURQUOISE2" => "00E5EE",
    "TURQUOISE3" => "00C5CD",
    "TURQUOISE4" => "00868B",
    "VIOLET" => "EE82EE",
    "VIOLETRED" => "D02090",
    "VIOLETRED1" => "FF3E96",
    "VIOLETRED2" => "EE3A8C",
    "VIOLETRED3" => "CD3278",
    "VIOLETRED4" => "8B2252",
    "WHEAT" => "F5DEB3",
    "WHEAT1" => "FFE7BA",
    "WHEAT2" => "EED8AE",
    "WHEAT3" => "CDBA96",
    "WHEAT4" => "8B7E66",
    "WHITE" => "FFFFFF",
    "WHITESMOKE" => "F5F5F5",
    "YELLOW" => "FFFF00",
    "YELLOW1" => "FFFF00",
    "YELLOW2" => "EEEE00",
    "YELLOW3" => "CDCD00",
    "YELLOW4" => "8B8B00",
    "YELLOWGREEN" => "9ACD32",
};

#[test]
fn convert_test() {
    let html = r#"<font color="red">test</font><br><font class="test" color="green">TE<b>S</b>T<br/>TEST</font>"#;
    let _unused = html_to_richtext(html).unwrap();
}
