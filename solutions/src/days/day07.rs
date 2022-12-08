use std::collections::HashMap;

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

const TOTAL_SIZE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 7;
    const TITLE: &'static str = "No Space Left On Device";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let data = self.input().get()?;
        let mut fs = build_directory_tree(&data)?;
        Ok(sum_sizes_lte(&mut fs, 100000))
    }

    fn part_two(&self) -> Result<usize> {
        let data = self.input().get()?;
        let mut fs = build_directory_tree(&data)?;
        Ok(minimum_to_delete(&mut fs, SPACE_NEEDED))
    }
}

fn build_directory_tree(instructions: &str) -> Result<Filesystem> {
    let mut wd = "/";
    let mut dirs = HashMap::from([("/".to_string(), vec![])]);
    let mut dir_history: Vec<&str> = vec!["/"];
    let mut current_dir: &mut Vec<Entry> = dirs.get_mut("/").unwrap();
    for l in instructions.lines() {
        if l.starts_with("$ ls") {
            continue;
        } else if l.starts_with("$ cd") {
            let mut target = &l[5..];
            if target == ".." {
                target = dir_history.pop().expect("No parent directory");
                if dir_history.len() == 0 {
                    dir_history.push("/");
                }
            } else if wd != "/" {
                dir_history.push(&wd);
            }
            let pathspec = match target {
                "/" => "/".to_string(),
                _ => format!("/{}", dir_history.iter().skip(1).chain([&target]).join("/")),
            };
            current_dir = dirs.entry(pathspec).or_insert(vec![]);
            wd = target;
        } else {
            let tokens = l.split(' ').collect_vec();
            if tokens[0] == "dir" {
                current_dir.push(Entry::Directory(tokens[1].to_string()));
            } else if tokens[0].chars().next().unwrap().is_numeric() {
                current_dir.push(Entry::File(
                    tokens[1].to_string(),
                    tokens[0].parse::<usize>().expect("Invalid file size"),
                ));
            } else {
                return Err(anyhow!("Invalid directory listing"));
            }
        }
    }
    Ok(Filesystem {
        dirs,
        wd: wd.to_string(),
        dir_sizes: HashMap::new(),
    })
}

fn sum_sizes_lte(fs: &mut Filesystem, max_filesize: usize) -> usize {
    let sizes = fs.get_dir_sizes();
    sizes.values().filter(|&&s| s <= max_filesize).sum()
}

fn minimum_to_delete(fs: &mut Filesystem, min_space_needed: usize) -> usize {
    let sizes = fs.get_dir_sizes();
    let total_used = sizes.get("/").unwrap();
    let free = TOTAL_SIZE - total_used;
    if free >= min_space_needed {
        return 0;
    }
    let target = min_space_needed - free;
    sizes
        .values()
        .filter(|&&s| s >= target)
        .min()
        .expect("No directory to delete")
        .to_owned()
}

#[derive(Debug, PartialEq)]
enum Entry {
    File(String, usize),
    Directory(String),
}

#[derive(Debug, PartialEq)]
struct Filesystem {
    dirs: HashMap<String, Vec<Entry>>,
    wd: String,
    dir_sizes: HashMap<String, usize>,
}
impl Filesystem {
    fn get_size(&self, dir: &str) -> usize {
        let entries = self
            .dirs
            .get(dir)
            .expect(&format!("Invalid directory: {}", dir));
        let base_path = match dir {
            "/" => "/".to_string(),
            _ => format!("{}/", dir),
        };
        entries
            .iter()
            .map(|e| match e {
                Entry::Directory(d) => {
                    let path = &format!("{}{}", base_path, d);
                    match self.dir_sizes.get(path) {
                        Some(size) => *size,
                        None => self.get_size(path),
                    }
                }
                Entry::File(_, s) => *s,
            })
            .sum()
    }

    fn get_dir_sizes(&mut self) -> &HashMap<String, usize> {
        let mut sizes = HashMap::new();
        for d in self.dirs.keys() {
            sizes.insert(d.to_string(), self.get_size(d));
        }
        self.dir_sizes = sizes;
        &self.dir_sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn should_parse_files() {
        let instructions = "123 abc.txt
456 def.tar";
        let mut expected = HashMap::new();
        expected.insert(
            "/".to_string(),
            vec![
                Entry::File("abc.txt".to_string(), 123),
                Entry::File("def.tar".to_string(), 456),
            ],
        );

        let fs = build_directory_tree(instructions).unwrap();
        assert_eq!(expected, fs.dirs);
    }

    #[test]
    fn should_parse_dirs() {
        let instructions = "dir a
dir b
dir c";
        let mut expected = HashMap::new();
        expected.insert(
            "/".to_string(),
            vec![
                Entry::Directory("a".to_string()),
                Entry::Directory("b".to_string()),
                Entry::Directory("c".to_string()),
            ],
        );

        let fs = build_directory_tree(instructions).unwrap();
        assert_eq!(expected, fs.dirs);
    }

    #[test]
    fn should_move_dirs() {
        let instructions = "dir a
$ cd a
dir b
$ cd b
dir c
$ cd c";
        let mut expected = HashMap::new();
        expected.insert("/".to_string(), vec![Entry::Directory("a".to_string())]);
        expected.insert("/a".to_string(), vec![Entry::Directory("b".to_string())]);
        expected.insert("/a/b".to_string(), vec![Entry::Directory("c".to_string())]);
        expected.insert("/a/b/c".to_string(), vec![]);

        let fs = build_directory_tree(instructions).unwrap();
        assert_eq!(expected, fs.dirs);
        assert_eq!("c", fs.wd);
    }

    #[test]
    fn should_move_up() {
        let instructions = "$ cd /
dir a
$ cd a
dir b
$ cd b
dir c
$ cd c
$ cd ..";

        let fs = build_directory_tree(instructions).unwrap();
        assert_eq!("b", fs.wd);
    }

    #[test]
    fn should_build_tree() {
        let instructions = "dir a
dir b
$ cd a
$ ls
123 abc.txt
$ cd ..
$ cd b
$ ls
456 def.tar";
        let mut expected = HashMap::new();
        expected.insert(
            "/".to_string(),
            vec![
                Entry::Directory("a".to_string()),
                Entry::Directory("b".to_string()),
            ],
        );
        expected.insert(
            "/a".to_string(),
            vec![Entry::File("abc.txt".to_string(), 123)],
        );
        expected.insert(
            "/b".to_string(),
            vec![Entry::File("def.tar".to_string(), 456)],
        );

        let fs = build_directory_tree(instructions).unwrap();
        assert_eq!(expected, fs.dirs);
    }

    #[test]
    fn should_get_size() {
        let instructions = "123 abc.txt
dir a
$ cd a
$ ls
456 def.tar";
        let fs = build_directory_tree(instructions).unwrap();
        let size = fs.get_size("/");
        assert_eq!(579, size);
    }

    #[test]
    fn should_solve_part_1() {
        let mut fs = build_directory_tree(EXAMPLE_INPUT).unwrap();
        let actual = sum_sizes_lte(&mut fs, 100000);
        assert_eq!(95437, actual);
    }

    #[test]
    fn should_solve_part_2() {
        let mut fs = build_directory_tree(EXAMPLE_INPUT).unwrap();
        let actual = minimum_to_delete(&mut fs, SPACE_NEEDED);
        assert_eq!(24933642, actual);
    }
}
