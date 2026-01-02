use std::collections::VecDeque;

#[allow(dead_code)]
pub fn disk_to_string(v: &Vec<Option<usize>>) -> String {
    v.iter()
        .map(|c| match c {
            Some(x) => format!("{}", x),
            None => ".".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub struct CompactDiskMap {
    items: VecDeque<Option<usize>>,
}

impl CompactDiskMap {
    fn remove_from_end_until_value(&mut self) -> Option<usize> {
        match self.items.pop_back()? {
            Some(x) => Some(x),
            None => self.remove_from_end_until_value(),
        }
    }
}

impl Iterator for CompactDiskMap {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.items.pop_front()? {
            Some(x) => Some(x),
            None => self.remove_from_end_until_value(),
        }
    }
}

impl FromIterator<Option<usize>> for CompactDiskMap {
    fn from_iter<T: IntoIterator<Item = Option<usize>>>(iter: T) -> Self {
        CompactDiskMap {
            items: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_empty() {
        let sut = CompactDiskMap::from_iter([]);
        let items: Vec<usize> = sut.collect();

        assert_eq!(items, vec![])
    }

    #[test]
    fn test_when_one_item() {
        let sut = CompactDiskMap::from_iter([Some(1)]);
        let items: Vec<usize> = sut.collect();

        assert_eq!(items, vec![1])
    }

    #[test]
    fn test_when_two_items_with_gap() {
        let sut = CompactDiskMap::from_iter([Some(1), None, Some(2)]);
        let items: Vec<usize> = sut.collect();

        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn test_when_three_items_with_gaps() {
        let sut = CompactDiskMap::from_iter([Some(1), None, Some(2), None, Some(3)]);
        let items: Vec<usize> = sut.collect();

        assert_eq!(items, vec![1, 3, 2])
    }
}
