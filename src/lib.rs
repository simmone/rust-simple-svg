use crate::defines::svg::Svg;

pub mod defines;

pub fn svg_out(width: u32, height: u32) -> u32 {
    width + height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = svg_out(2, 2);
        assert_eq!(result, 4);
    }
}
