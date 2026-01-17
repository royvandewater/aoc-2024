use crate::xy::XY;

pub(crate) fn find_end(input: &str) -> Option<XY> {
    input.trim().lines().enumerate().find_map(|(y, line)| {
        line.trim()
            .chars()
            .enumerate()
            .find_map(move |(x, c)| match c == 'E' {
                true => Some(XY::new(x, y)),
                false => None,
            })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert!(find_end("").is_none());
    }

    #[test]
    fn test_base_case() {
        let result = find_end("E").unwrap();

        assert_eq!(result, XY::new(0, 0));
    }
}
