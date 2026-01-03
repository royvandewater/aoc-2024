use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

#[derive(Clone, Debug)]
enum Block {
    // (id, size)
    Used(usize, usize),
    // (size)
    Free(usize),
}

impl Block {
    fn is_used(&self) -> bool {
        match self {
            Used(_, _) => true,
            Free(_) => false,
        }
    }

    fn has_id(&self, id: usize) -> bool {
        match self {
            Used(used_id, _) => *used_id == id,
            Free(_) => false,
        }
    }

    fn id_in(&self, ids: &HashSet<usize>) -> bool {
        match self {
            Used(id, _) => ids.contains(id),
            Free(_) => false,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Used(id, size) => f.write_fmt(format_args!("Used({}, {})", id, size)),
            Free(size) => f.write_fmt(format_args!("Free({})", size)),
        }
    }
}

use Block::*;

// (id, used_size, free_size)
type Chunk = (usize, usize, Option<usize>);

#[derive(Clone)]
pub struct DiskMap {
    items: VecDeque<Block>,
}

impl DiskMap {
    #[allow(dead_code)]
    fn new() -> DiskMap {
        DiskMap {
            items: VecDeque::new(),
        }
    }

    // Returns a new defragmented DiskMap, consumes the original
    pub fn to_defragged(&mut self) -> DiskMap {
        let mut processed_ids: HashSet<usize> = HashSet::new();

        let items = self.items.clone();
        let used_blocks = items.iter().rev().filter(|x| x.is_used()).map(|x| match x {
            Used(id, size) => (id, size),
            Free(_) => panic!("We should've filtered out all the free blocks"),
        });

        for (id, size) in used_blocks {
            processed_ids.insert(*id);
            self.insert_where_it_fits(*id, *size);
        }

        let last_used = self.last_used_block(&HashSet::new());
        match last_used {
            Some((id, _size)) => {
                let i = self.index_of(id);
                self.items.truncate(i + 1);
            }
            None => {}
        }

        self.items.iter().cloned().collect()
    }

    // returns the index and block of the last Used block that hasn't been processed yet.
    fn last_used_block(&self, already_processed_ids: &HashSet<usize>) -> Option<(usize, usize)> {
        let block = self
            .items
            .iter()
            .rev()
            .find(|x| x.is_used() && !x.id_in(already_processed_ids))?;

        match block {
            Free(_) => panic!("Block should not be free, we filtered those out!"),
            Used(id, size) => Some((*id, *size)),
        }
    }

    fn index_of(&self, id: usize) -> usize {
        self.items
            .iter()
            .enumerate()
            .find(|(_i, b)| b.has_id(id))
            .unwrap()
            .0
    }

    // Will find the first free space large enough to fit this if available,
    // will leave it where it is if there's nothing.
    fn insert_where_it_fits(&mut self, id: usize, size: usize) -> Option<()> {
        let original_i = self.index_of(id);
        let (i, free_size) = self.first_large_enough_free(original_i, size)?;

        match free_size {
            free_size if free_size == size => {
                self.items.remove(original_i);
                self.items.insert(original_i, Free(size));
                self.items.insert(i, Used(id, size));
                self.items.remove(i + 1);
            }
            free_size => {
                self.items.remove(original_i);
                self.items.insert(original_i, Free(size));
                self.items.insert(i, Used(id, size));
                self.items.insert(i + 1, Free(free_size - size));
                self.items.remove(i + 2);
            }
        }

        Some(())
    }

    fn first_large_enough_free(&self, max_i: usize, target_size: usize) -> Option<(usize, usize)> {
        self.items.iter().enumerate().find_map(|x| match x {
            (i, Free(free_size)) if i < max_i && target_size <= *free_size => Some((i, *free_size)),
            _ => None,
        })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.clone().collect::<Vec<Option<usize>>>().into_iter() {
            match x {
                Some(x) => f.write_fmt(format_args!("{}", x))?,
                None => f.write_str(".")?,
            };
        }

        Ok(())
    }
}

impl FromStr for DiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.trim()
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(parse_chunk)
            .collect::<DiskMap>())
    }
}

fn parse_chunk((id, c): (usize, &[usize])) -> (usize, usize, Option<usize>) {
    let mut c: VecDeque<usize> = c.to_vec().into();
    let used = c.pop_front().unwrap();
    let free = match c.pop_front() {
        None => None,
        Some(size) if size == 0 => None,
        Some(size) => Some(size),
    };

    (id, used, free)
}

impl Iterator for DiskMap {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.items.pop_front()? {
            Used(id, size) => {
                if size > 1 {
                    self.items.push_front(Used(id, size - 1));
                }
                Some(Some(id))
            }
            Free(size) => {
                if size > 1 {
                    self.items.push_front(Free(size - 1));
                }
                Some(None)
            }
        }
    }
}

impl FromIterator<Block> for DiskMap {
    fn from_iter<T: IntoIterator<Item = Block>>(iter: T) -> Self {
        DiskMap {
            items: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Chunk> for DiskMap {
    fn from_iter<T: IntoIterator<Item = Chunk>>(iter: T) -> Self {
        iter.into_iter().flat_map(chunk_to_block).collect()
    }
}

fn chunk_to_block((id, used, free): Chunk) -> Vec<Block> {
    match (id, used, free) {
        (id, used, None) => vec![Used(id, used)],
        (id, used, Some(free)) => vec![Used(id, used), Free(free)],
    }
}

#[allow(dead_code)]
fn disk_to_string(v: &Vec<Block>) -> String {
    v.iter()
        .flat_map(expand_block)
        .map(|c| match c {
            Some(x) => format!("{}", x),
            None => ".".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn expand_block(block: &Block) -> Vec<Option<usize>> {
    match block {
        Used(id, size) => vec![Some(*id); *size],
        Free(size) => vec![None; *size],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_empty() {
        let sut = DiskMap::new().to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![])
    }

    #[test]
    fn test_when_one_item_id_0() {
        let sut = DiskMap::from_iter([Used(0, 1)]).to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![Some(0)])
    }

    #[test]
    fn test_when_one_item_id_2() {
        let sut = DiskMap::from_iter([Used(2, 1)]).to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![Some(2)])
    }

    #[test]
    fn test_when_one_item_id_2_size_3() {
        let sut = DiskMap::from_iter([Used(2, 3)]).to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![Some(2), Some(2), Some(2)])
    }

    #[test]
    fn test_when_two_items_with_gap() {
        // 1.2
        let sut = DiskMap::from_iter([Used(1, 1), Free(1), Used(2, 1)]).to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![Some(1), Some(2)])
    }

    #[ignore]
    #[test]
    fn test_when_three_items_with_gaps() {
        // 1.2.3
        let sut = DiskMap::from_iter([Used(1, 1), Free(1), Used(2, 1), Free(1), Used(3, 1)])
            .to_defragged();
        let items: Vec<Option<usize>> = sut.collect();

        assert_eq!(items, vec![Some(1), Some(3), Some(2)])
    }

    #[test]
    fn test_parse_example() {
        let input = "2333133121414131402";
        let disk_map: DiskMap = input.parse().unwrap();
        let output = format!("{}", disk_map);

        assert_eq!(output, "00...111...2...333.44.5555.6666.777.888899");
    }
}
