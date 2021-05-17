#[derive(Default, Debug)]
pub struct Layout {
    layout_target: String,
    x_mode: String,
    y_mode: String,
    x_pos: f64,
    y_pos: f64,
    width: i32,
    height: i32,
    show_legend_key: bool,
    show_val: bool,
    show_cat_name: bool,
    show_ser_name: bool,
    show_percent: bool,
    show_bubble_size: bool,
    show_leader_lines: bool,
}
impl Layout {
    pub fn get_layout_target(&self)-> &str {
        &self.layout_target
    }

    pub fn set_layout_target<S: Into<String>>(&mut self, value:S)-> &mut Layout {
        self.layout_target = value.into();
        self
    }

    pub fn get_x_mode(&self)-> &str {
        &self.x_mode
    }

    pub fn set_x_mode<S: Into<String>>(&mut self, value:S)-> &mut Layout {
        self.x_mode = value.into();
        self
    }

    pub fn get_y_mode(&self)-> &str {
        &self.y_mode
    }

    pub fn set_y_mode<S: Into<String>>(&mut self, value:S)-> &mut Layout {
        self.y_mode = value.into();
        self
    }

    pub fn get_x_pos(&self)-> &f64 {
        &self.x_pos
    }

    pub fn set_x_pos(&mut self, value:f64)-> &mut Layout {
        self.x_pos = value;
        self
    }

    pub fn get_y_pos(&self)-> &f64 {
        &self.y_pos
    }

    pub fn set_y_pos(&mut self, value:f64)-> &mut Layout {
        self.y_pos = value;
        self
    }

    pub fn get_width(&self)-> &i32 {
        &self.width
    }

    pub fn set_width(&mut self, value:i32)-> &mut Layout {
        self.width = value;
        self
    }

    pub fn get_height(&self)-> &i32 {
        &self.height
    }

    pub fn set_height(&mut self, value:i32)-> &mut Layout {
        self.height = value;
        self
    }

    pub fn get_show_legend_key(&self)-> &bool {
        &self.show_legend_key
    }

    pub fn set_show_legend_key(&mut self, value:bool)-> &mut Layout {
        self.show_legend_key = value;
        self
    }
    
    pub fn get_show_val(&self)-> &bool {
        &self.show_val
    }

    pub fn set_show_val(&mut self, value:bool)-> &mut Layout {
        self.show_val = value;
        self
    }

    pub fn get_show_cat_name(&self)-> &bool {
        &self.show_cat_name
    }

    pub fn set_show_cat_name(&mut self, value:bool)-> &mut Layout {
        self.show_cat_name = value;
        self
    }

    pub fn get_show_ser_name(&self)-> &bool {
        &self.show_ser_name
    }

    pub fn set_show_ser_name(&mut self, value:bool)-> &mut Layout {
        self.show_ser_name = value;
        self
    }

    pub fn get_show_percent(&self)-> &bool {
        &self.show_percent
    }

    pub fn set_show_percent(&mut self, value:bool)-> &mut Layout {
        self.show_percent = value;
        self
    }

    pub fn get_show_bubble_size(&self)-> &bool {
        &self.show_bubble_size
    }

    pub fn set_show_bubble_size(&mut self, value:bool)-> &mut Layout {
        self.show_bubble_size = value;
        self
    }

    pub fn get_show_leader_lines(&self)-> &bool {
        &self.show_leader_lines
    }

    pub fn set_show_leader_lines(&mut self, value:bool)-> &mut Layout {
        self.show_leader_lines = value;
        self
    }
}
