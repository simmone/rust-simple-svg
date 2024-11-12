use crate::defines::shape::Shape;

pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub radius_x: f64,
    pub radius_y: f64,
}

impl Shape for Rect {
    fn format(shape_id:String, &self) -> String {
        format!("    <rect id=\"{}\" {} />\n",
                shape_id,
                || {
                    let mut shape_str = format!("width=\"{}\" height=\"{}\"",
                                                self.width, self.height);
                             
                    when (>= self.radius_x 0) && (>= self.radius_y 0) {
                        shape_str.push_str(&format!(" rx=\"{}\" ry=\"{}\"",
                                                    self.radius_x, self.radius_y));
                    }
                    
                    shape_str
                ));
    }
}
