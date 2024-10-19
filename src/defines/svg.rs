pub struct Svg {
    pub width: u64,
    pub height: u64,
}

pub fn new_svg(width: u64, height: u64) -> Svg {
    Svg { width, height }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_svg() {
        let svg = new_svg(640, 480);
        assert_eq!(svg.width, 640);
        assert_eq!(svg.height, 480);
    }
}
