pub struct ViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewBox {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        ViewBox {
            min_x,
            min_y,
            width,
            height,
        }
    }
}
