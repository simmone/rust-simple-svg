use super::*;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn svg_out_test() {
        let result = svg_out(2, 2);
        assert_eq!(result, "hello world");
    }
}
