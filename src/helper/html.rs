use structs::RichText;
use html_parser::Dom;

pub fn html_to_richtext(html: &str)->Result<RichText> {
    let dom = Dom::parse(html);
}
