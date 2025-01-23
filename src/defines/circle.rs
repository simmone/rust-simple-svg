#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle {
            radius,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        format!("    <circle id=\"{}\" r=\"{}\" />\n", shape_id, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_circle() {
        let circle = Circle::new(30.0);

        assert_eq!(circle.radius, 30.0);
    }

    #[test]
    fn check_format() {
        let circle = Circle::new(30.0);

        assert_eq!(
            Circle::format(&circle, "1".to_string()),
            "    <circle id=\"1\" r=\"30\" />\n"
        );
    }
}
