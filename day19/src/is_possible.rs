pub(crate) fn is_possible(towels: &Vec<&str>, pattern: &str) -> bool {
    if towels.contains(&pattern) {
        return true;
    }

    towels.iter().filter(|towel| pattern.starts_with(**towel)).any(|towel| {
        let sub_pattern = pattern.trim_start_matches(towel);
        is_possible(towels, sub_pattern)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_towels() {
        let towels = vec![];
        let result = is_possible(&towels, "asdf");

        assert!(!result, "towels is empty, so no pattern should be possible");
    }

    #[test]
    fn test_identity() {
        let towels = vec!["asdf"];
        let result = is_possible(&towels, "asdf");

        assert!(result, "there is an exact match towel that it could've used");
    }

    #[test]
    fn test_combine_two_towels() {
        let towels = vec!["as", "df"];
        let result = is_possible(&towels, "asdf");

        assert!(result, "Combining the only two towels would produce the pattern");
    }
}
