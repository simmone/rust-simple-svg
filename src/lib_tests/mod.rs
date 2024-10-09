use crate::defines::svg::*;

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::svg_out;

    #[test]
    fn void_svg_out_test() {
        let svg = svg_out(build_svg(2, 3));
        
        let contents = include_str!("../showcase/basic/empty.svg");
        
        assert_eq!(svg, contents);
    }
}
