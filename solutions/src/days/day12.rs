use crate::input::AdventInput;
use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const DAY: u8 = 12;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let input = AdventInput::for_day(DAY).get()?;
        let map = Map::from(input.as_str());
        Ok(map.get_path_count())
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let input = AdventInput::for_day(DAY).get()?;
        let mut map = Map::from(input.as_str());
        map.allow_small_pass = true;
        Ok(map.get_path_count())
    }
}

const START: &'static str = "start";
const END: &'static str = "end";

struct Map<'a> {
    caves: HashMap<&'a str, HashSet<&'a str>>,
    allow_small_pass: bool,
}
impl<'a> From<&'a str> for Map<'a> {
    fn from(input: &'a str) -> Self {
        let mut map = Map::new();
        input
            .lines()
            .map(|l| l.split('-').collect::<Vec<_>>())
            .for_each(|p| map.add_path(p[0], p[1]));
        map
    }
}
impl<'a> Map<'a> {
    fn new() -> Self {
        Map {
            caves: HashMap::new(),
            allow_small_pass: false,
        }
    }

    fn add_path(&mut self, from: &'a str, to: &'a str) {
        if to != START {
            let forward = self.caves.entry(from).or_default();
            forward.insert(to);
        }
        if to != END && from != START {
            let backward = self.caves.entry(to).or_default();
            backward.insert(from);
        }
    }

    fn get_path_count(&self) -> usize {
        let mut paths = 0;
        for path in &self.caves[START] {
            let start_counts = HashMap::new();
            paths += self.paths_from(path, &start_counts);
        }
        paths
    }

    fn paths_from(&self, start: &'a str, counts: &HashMap<&'a str, usize>) -> usize {
        // Only count paths that reach the end
        if start == END {
            return 1;
        }

        let mut smalls = counts.clone();
        if start.chars().all(|c| c.is_lowercase()) {
            if self.allow_small_pass {
                if let Some(&cnt) = smalls.get(start) {
                    if cnt > 0 && smalls.values().any(|&v| v == 2) {
                        return 0;
                    }
                }
            } else {
                if smalls.contains_key(start) {
                    return 0;
                }
            }
            *smalls.entry(start).or_insert(0) += 1;
        }

        self.caves[start]
            .iter()
            .map(|&c| self.paths_from(c, &smalls))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "start-end";
        let map: Map = input.into();
        assert_eq!(vec![&"start"], map.caves.keys().collect::<Vec<_>>());
        assert_eq!(&"end", map.caves["start"].iter().next().unwrap())
    }

    #[test]
    fn should_add_paths() {
        let mut map = Map::new();
        map.add_path(&"a", &"b");
        assert_eq!(2, map.caves.len());
        assert_eq!(vec![&"a", &"b"], map.caves.keys().collect::<Vec<_>>());
        assert_eq!(&"b", map.caves["a"].iter().next().unwrap());
        assert_eq!(&"a", map.caves["b"].iter().next().unwrap());
    }

    #[test]
    fn should_add_multiple() {
        let input = "start-a
start-b
a-b
a-end
b-end";
        let map: Map = input.into();
        assert_eq!(3, map.caves.len());
        assert_eq!(
            vec![&"a", &"b", &START],
            map.caves.keys().sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![&"a", &"b"],
            map.caves[START].iter().sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![&"b", &END],
            map.caves["a"].iter().sorted().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![&"a", &END],
            map.caves["b"].iter().sorted().collect::<Vec<_>>()
        );
    }

    #[test]
    fn should_count_paths() {
        let input = "start-a
start-b
a-b
a-end
b-end";
        let map: Map = input.into();
        let actual = map.get_path_count();
        assert_eq!(4, actual);
    }

    #[test]
    fn should_solve_part1_example1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let map: Map = input.into();
        let actual = map.get_path_count();
        assert_eq!(10, actual);
    }

    #[test]
    fn should_solve_part1_example2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let map: Map = input.into();
        let actual = map.get_path_count();
        assert_eq!(19, actual);
    }

    #[test]
    fn should_solve_part1_example3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let map: Map = input.into();
        let actual = map.get_path_count();
        assert_eq!(226, actual);
    }

    #[test]
    fn should_solve_part2_example() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let mut map: Map = input.into();
        map.allow_small_pass = true;
        let actual = map.get_path_count();
        assert_eq!(36, actual);
    }
}
