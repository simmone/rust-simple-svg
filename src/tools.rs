pub mod precision {
    pub fn svg_round(n: f64) -> String {
        let pown = 10.0_f64.powf(4.0);
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
      assert_eq!(svg_round(30.0), "30");
      assert_eq!(svg_round(30.0000), "30");
      assert_eq!(svg_round(30.0001), "30.0001");
      assert_eq!(svg_round(30.00001), "30");
      assert_eq!(svg_round(30.00005), "30.0001");
      assert_eq!(svg_round(30.123), "30.123");
      assert_eq!(svg_round(30.1234), "30.1234");
      assert_eq!(svg_round(30.12344), "30.1234");
      assert_eq!(svg_round(30.12345), "30.1235");
    }
}
