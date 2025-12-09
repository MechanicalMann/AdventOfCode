use std::{collections::HashSet, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 08;
    const TITLE: &'static str = "UNKNOWN";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let boxes = self.input().get_lines_as::<IPoint3D>()?;
        let circuits = connect(&boxes, 1000);
        Ok(magnitude(&circuits))
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IPoint3D {
    x: isize,
    y: isize,
    z: isize,
}
impl FromStr for IPoint3D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let split = s.split(',').collect_vec();
        if split.len() != 3 {
            return Err(anyhow!("Invalid 3D point"));
        }
        let (x, y, z) = (split[0].parse()?, split[1].parse()?, split[2].parse()?);
        Ok(Self { x, y, z })
    }
}
impl IPoint3D {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: &IPoint3D) -> isize {
        let (a, b, c) = (self.x - other.x, self.y - other.y, self.z - other.z);
        (a.pow(2) + b.pow(2) + c.pow(2)).isqrt().abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connection {
    l: IPoint3D,
    r: IPoint3D,
    dist: isize,
}
impl Connection {
    fn new(l: IPoint3D, r: IPoint3D) -> Self {
        let dist = l.dist(&r);
        Self { l, r, dist }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Circuit {
    boxes: HashSet<IPoint3D>,
    connections: HashSet<Connection>,
}
impl Circuit {
    fn from<I>(connections: I) -> Self
    where
        I: IntoIterator<Item = Connection>,
    {
        let mut circuit = Self {
            boxes: HashSet::new(),
            connections: HashSet::new(),
        };
        connections.into_iter().for_each(|c| circuit.add(c));
        circuit
    }
    fn add(&mut self, conn: Connection) {
        self.boxes.insert(conn.l);
        self.boxes.insert(conn.r);
        self.connections.insert(conn);
    }

    fn contains(&self, c: &Connection) -> bool {
        self.boxes.contains(&c.l) || self.boxes.contains(&c.r)
    }

    fn len(&self) -> usize {
        self.boxes.len()
    }

    fn overlaps(&self, other: &Circuit) -> bool {
        self.boxes.intersection(&other.boxes).next().is_some()
    }

    fn merge(&mut self, other: &Circuit) {
        for &c in &other.connections {
            self.add(c);
        }
    }
}

fn connect(boxes: &Vec<IPoint3D>, num: usize) -> Vec<Circuit> {
    let mut circuits: Vec<Circuit> = vec![];
    let mut pairs = vec![];
    for i in 0..(boxes.len() - 1) {
        for j in (i + 1)..boxes.len() {
            pairs.push(Connection::new(boxes[i], boxes[j]));
        }
    }
    let mut sorted = pairs.iter().sorted_by(|l, r| Ord::cmp(&l.dist, &r.dist));
    for _ in 0..num {
        let Some(pair) = sorted.next() else {
            break;
        };
        if let Some(c) = circuits.iter_mut().find(|c| c.contains(pair)) {
            c.add(*pair);
        } else {
            let c = Circuit::from([*pair]);
            circuits.push(c);
        }
    }
    if circuits.len() <= 1 {
        return circuits;
    }
    loop {
        let mut merged = vec![];
        let mut did_merge = HashSet::new();
        for i in 0..circuits.len() {
            if did_merge.contains(&i) {
                continue;
            }
            let mut l = circuits[i].clone();
            for j in (i + 1)..circuits.len() {
                let r = &circuits[j];
                if l.overlaps(r) {
                    l.merge(r);
                    did_merge.insert(j);
                }
            }
            merged.push(l);
        }
        circuits = merged;
        if did_merge.len() == 0 {
            break;
        }
    }
    circuits
}

fn magnitude(circuits: &[Circuit]) -> usize {
    circuits
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "1,2,3".parse::<IPoint3D>()?;
        assert_eq!(1, test.x);
        assert_eq!(2, test.y);
        assert_eq!(3, test.z);
        Ok(())
    }

    #[test]
    fn should_get_distance() {
        let (a, b) = (IPoint3D::new(1, 2, 3), IPoint3D::new(4, 5, 6));
        assert_eq!(5, a.dist(&b));
    }

    #[test]
    fn should_get_circuits() {
        let boxes = vec![
            IPoint3D::new(1, 2, 3),
            IPoint3D::new(4, 5, 6),
            IPoint3D::new(11, 22, 33),
            IPoint3D::new(44, 55, 66),
        ];
        let circuits = connect(&boxes, 2);
        assert_eq!(1, circuits.len());
        assert_eq!(
            vec![Circuit::from([
                Connection::new(IPoint3D::new(1, 2, 3), IPoint3D::new(4, 5, 6)),
                Connection::new(IPoint3D::new(4, 5, 6), IPoint3D::new(11, 22, 33))
            ]),],
            circuits
        );
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let boxes = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<IPoint3D>().ok())
            .collect_vec();
        let circuits = connect(&boxes, 10);
        let magnitude = magnitude(&circuits);
        assert_eq!(40, magnitude);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let boxes = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<IPoint3D>().ok())
            .collect_vec();
        let circuits = connect(&boxes, usize::MAX);
        let last_connection = circuits.last().unwrap().connections.iter().last().unwrap();
        assert_eq!(1, circuits.len());
        assert_eq!(25272, last_connection.l.x * last_connection.r.x);
        Ok(())
    }
}
