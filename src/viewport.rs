pub struct Viewport {
    pub center_x: f64,
    pub center_y: f64,
    pub scale: f64,
}

const SCALE_STEP: f64 = 0.95;
const OFFSET_STEP: f64 = 0.05;

impl Viewport {
    pub fn new(center_x: f64, center_y: f64, scale: f64) -> Self {
        Self {
            center_x,
            center_y,
            scale,
        }
    }

    pub fn zoom_in(&mut self, step: Option<f64>) {
        self.scale *= step.unwrap_or(SCALE_STEP);
    }

    pub fn zoom_out(&mut self, step: Option<f64>) {
        self.scale /= step.unwrap_or(SCALE_STEP);
    }

    pub fn shift_left(&mut self, step: Option<f64>) {
        self.center_x -= self.scale * step.unwrap_or(OFFSET_STEP);
    }
    
    pub fn shift_right(&mut self, step: Option<f64>) {
        self.center_x += self.scale * step.unwrap_or(OFFSET_STEP);
    }

    pub fn shift_up(&mut self, step: Option<f64>) {
        self.center_y += self.scale * step.unwrap_or(OFFSET_STEP);
    }

    pub fn shift_down(&mut self, step: Option<f64>) {
        self.center_y -= self.scale * step.unwrap_or(OFFSET_STEP);
    }

    pub fn zoom_reset(&mut self) {
        self.scale = 1.0;
    }
}
