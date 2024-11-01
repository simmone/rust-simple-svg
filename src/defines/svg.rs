pub struct Svg {
    pub width: u32,
    pub height: u32,
    pub widget_id_count: u32,
}

pub fn new_svg(width: u32, height: u32) -> Svg {
    Svg {
        width,
        height,
        widget_id_count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_svg() {
        let svg = new_svg(640, 480);
        assert_eq!(svg.width, 640);
        assert_eq!(svg.height, 480);
        assert_eq!(svg.widget_id_count, 0);
    }
}
