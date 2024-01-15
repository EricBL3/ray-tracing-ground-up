#[derive(Clone)]
pub struct ViewPlane {
    pub horizontal_res: u32,
    pub vertical_res: u32,
    pub pixel_size: f64,
    pub gamma: f32,
    pub inv_gamma: f32,
    pub show_out_of_gamut: bool,
}

impl Default for ViewPlane {
    fn default() -> Self {
        Self {
            horizontal_res: 0,
            vertical_res: 0,
            pixel_size: 1.0,
            gamma: 1.0,
            inv_gamma: 1.0,
            show_out_of_gamut: false,
        }
    }
}

impl ViewPlane {
    pub fn set_hres(&mut self, h_res: u32) {
        self.horizontal_res = h_res;
    }

    pub fn set_vres(&mut self, v_res: u32) {
        self.vertical_res = v_res;
    }

    pub fn set_pixel_size(&mut self, p_size: f64) {
        self.pixel_size = p_size;
    }

    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
        self.inv_gamma = 1.0 / gamma;
    }

    pub fn set_gamut_display(&mut self, show: bool) {
        self.show_out_of_gamut = show;
    }
}
