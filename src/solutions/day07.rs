use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Default)]
struct Bag {
    color: String,
    can_contain: Vec<(usize, String)>,
}
impl Bag {
    fn new(color: String) -> Bag {
        Bag {
            color,
            ..Default::default()
        }
    }

    fn contains(&mut self, bag: (usize, String)) {
        self.can_contain.push(bag);
    }
}

fn parse(input: &String) -> HashMap<String, Bag> {
    let mut map = HashMap::new();
    let outer = Regex::new(r"^(.+) bags contain (.+)\.$").unwrap();
    let inner = Regex::new(r"(\d+) ([^,.]+) bags?").unwrap();
    for line in input.lines() {
        let m = match outer.captures(line) {
            Some(cap) => cap,
            None => continue,
        };
        let node = map
            .entry(m[1].to_string())
            .or_insert(Bag::new(m[1].to_string()));
        for contained in inner.captures_iter(&m[2]) {
            let num = contained[1].parse::<usize>().unwrap();
            node.contains((num, contained[2].to_string()));
        }
    }
    map
}

fn get_nodes_containing(target: &String, nodes: &HashMap<String, Bag>) -> HashSet<String> {
    let mut bags: HashSet<String> = HashSet::new();
    for node in nodes.values() {
        for (_, color) in node.can_contain.iter() {
            if color == target {
                bags.insert(node.color.clone());
                bags.extend(get_nodes_containing(&node.color, nodes));
            }
        }
    }
    bags
}

fn count_children(target: &String, nodes: &HashMap<String, Bag>) -> usize {
    let mut total: usize = 0;
    let node = nodes.get(target).expect("Invalid ruleset!");
    for (num, color) in node.can_contain.iter() {
        total += num;
        total += count_children(&color, nodes) * num;
    }
    total
}

pub fn part1() {
    let datafile = "data/day7.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let nodes = parse(&read);
    println!(
        "Answer: {}",
        get_nodes_containing(&"shiny gold".to_string(), &nodes).len()
    );
}

pub fn part2() {
    let datafile = "data/day7.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let nodes = parse(&read);
    println!(
        "Answer: {}",
        count_children(&"shiny gold".to_string(), &nodes)
    );
}
