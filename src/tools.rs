pub mod precision {
    pub fn svg_round(n: f64, precision: usize) -> String {
        let pown = 10.0_f64.powf(precision as f64);
        let roundn = (n * pown).round()/pown;
        if roundn == roundn.round() {
            format!("{}", roundn.round())
        } else {
            format!("{}", roundn)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::precision::svg_round;

    #[test]
    fn check_svg_round() {
        assert_eq!(svg_round(30.0, 4), "30");
        assert_eq!(svg_round(30.0000, 4), "30");
        assert_eq!(svg_round(30.0001, 4), "30.0001");
        assert_eq!(svg_round(30.00001, 4), "30");
        assert_eq!(svg_round(30.00005, 4), "30.0001");
        assert_eq!(svg_round(30.123, 4), "30.123");
        assert_eq!(svg_round(30.1234, 4), "30.1234");
        assert_eq!(svg_round(30.12344, 4), "30.1234");
        assert_eq!(svg_round(30.12345, 4), "30.1235");

        assert_eq!(svg_round(30.5555, 0), "31");
        assert_eq!(svg_round(30.5555, 1), "30.6");
        assert_eq!(svg_round(30.5555, 2), "30.56");
        assert_eq!(svg_round(30.5555, 3), "30.556");
    }
}
