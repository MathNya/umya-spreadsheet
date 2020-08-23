use super::header_footer_drawing::HeaderFooterDrawing;

#[derive(Default, Debug)]
pub struct HeaderFooter {
    odd_header: String,
    odd_footer: String,
    even_header: String,
    even_footer: String,
    first_header: String,
    first_footer: String,
    different_odd_even: bool,
    different_first: bool,
    scale_with_document: bool,
    align_with_margins: bool,
    header_footer_images: Vec<HeaderFooterDrawing>,
}
impl HeaderFooter {
    pub fn get_header_footer_images(&self)-> &Vec<HeaderFooterDrawing> {
        &self.header_footer_images
    }
}