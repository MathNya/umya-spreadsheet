#[derive(Default, Debug)]
pub struct PageSetup {
    paper_size: i32,
    orientation: String,
    scale: i32,
    fit_to_page: bool,
    fit_to_height: i32,
    fit_to_width: i32,
    columns_to_repeat_at_left: Vec<String>,
    rows_to_repeat_at_top: Vec<i32>,
    horizontal_centered: bool,
    vertical_centered: bool,
    print_area: String,
    first_page_number: i32,
}
impl PageSetup {
    pub fn get_paper_size(&self)-> &i32 {
        &self.paper_size
    }
    pub fn set_paper_size(&mut self, value:i32) {
        self.paper_size = value;
    }
    pub fn get_fit_to_page(&self)-> &bool {
        &self.fit_to_page
    }
    pub fn set_fit_to_page(&mut self, value:bool) {
        self.fit_to_page = value;
    }
}