use memoize::memoize;

#[memoize(Ignore: towels)]
pub(crate) fn number_of_possibilities(towels: &Vec<&str>, pattern: String) -> usize {
    let exact_matches = towels.iter().filter(|t| **t == pattern).count();

    let sub_matches = towels
        .iter()
        .filter_map(|towel| {
            let sub_pattern = pattern.strip_prefix(towel)?;
            Some(number_of_possibilities(towels, sub_pattern.to_string()))
        })
        .sum::<usize>();

    exact_matches + sub_matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_towels() {
        let towels = vec![];
        let result = number_of_possibilities(&towels, "asdf".to_string());

        assert_eq!(result, 0, "towels is empty, so no pattern should be possible");
    }

    #[test]
    fn test_identity() {
        let towels = vec!["asdf"];
        let result = number_of_possibilities(&towels, "asdf".to_string());

        assert_eq!(result, 1, "there is an exact match towel that it can used");
    }

    #[test]
    fn test_combine_two_towels() {
        let towels = vec!["as", "df"];
        let result = number_of_possibilities(&towels, "asdf".to_string());

        assert_eq!(
            result, 1,
            "Combining the only two towels is the only way to produce the pattern"
        );
    }

    #[test]
    fn test_brwrr() {
        let towels = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let result = number_of_possibilities(&towels, "brwrr".to_string());

        assert_eq!(result, 2);
    }
}
