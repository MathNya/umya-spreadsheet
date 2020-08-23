#[derive(Default, Debug)]
pub struct Hyperlink {
    url: String,
    tooltip: String,
}
impl Hyperlink {
    pub fn get_url(&self) -> &String {
        &self.url
    }
    pub fn is_internal(&self) -> bool {
        return match self.url.as_str().find("sheet://") {
            Some(value) => true,
            None => false,
        }
    }
}
