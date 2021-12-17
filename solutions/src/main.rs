use anyhow::Result;
use itertools::Itertools;
use prettytable::{cell, color, format::Alignment, row, Attr, Cell, Row, Table};
use solver::{Measure, Solver};

mod days;
mod input;
mod solver;

fn main() -> Result<()> {
    if Some(String::from("perf")) == env::args().nth(1) {
        perf()?;
    }
    days::day01::Solution::new().solve()?;
    days::day02::Solution::new().solve()?;
    days::day03::Solution::new().solve()?;
    days::day04::Solution::new().solve()?;
    days::day05::Solution::new().solve()?;
    days::day06::Solution::new().solve()?;
    days::day07::Solution::new().solve()?;
    days::day08::Solution::new().solve()?;
    days::day09::Solution::new().solve()?;
    days::day10::Solution::new().solve()?;
    days::day11::Solution::new().solve()?;
    days::day12::Solution::new().solve()?;
    days::day13::Solution::new().solve()?;
    days::day14::Solution::new().solve()?;
    days::day15::Solution::new().solve()?;
    days::day16::Solution::new().solve()?;
    Ok(())
}

const TITLES: [&str; 25] = [
    "Sonar Sweep",
    "Dive!",
    "Binary Diagnostic",
    "Giant Squid",
    "Hydrothermal Venture",
    "Lanternfish",
    "The Treachery of Whales",
    "Seven Segment Search",
    "Smoke Basin",
    "Syntax Scoring",
    "Dumbo Octopus",
    "Passage Pathing",
    "Transparent Origami",
    "Extended Polymerization",
    "Chiton",
    "Packet Decoder",
    "???",
    "???",
    "???",
    "???",
    "???",
    "???",
    "???",
    "???",
    "???",
];

fn perf() -> Result<()> {
    let measures: Vec<_> = vec![
        Measure::get(days::day01::Solution::new()),
        Measure::get(days::day02::Solution::new()),
        Measure::get(days::day03::Solution::new()),
        Measure::get(days::day04::Solution::new()),
        Measure::get(days::day05::Solution::new()),
        Measure::get(days::day06::Solution::new()),
        Measure::get(days::day07::Solution::new()),
        Measure::get(days::day08::Solution::new()),
        Measure::get(days::day09::Solution::new()),
        Measure::get(days::day10::Solution::new()),
        Measure::get(days::day11::Solution::new()),
        Measure::get(days::day12::Solution::new()),
        Measure::get(days::day13::Solution::new()),
        Measure::get(days::day14::Solution::new()),
        Measure::get(days::day15::Solution::new()),
        Measure::get(days::day16::Solution::new()),
    ];
    let mut results = vec![];
    for m in measures {
        results.push((m.describe_part_one(), m.time_part_one()?));
        results.push((m.describe_part_two(), m.time_part_two()?));
    }

    // Get some rough stats
    let len = results.len();
    let (i1, i2, i3) = (len / 2, (len / 4) * 2, (len * 90) / 100);
    let ranked = results
        .iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, d)| d.as_secs_f32())
        .collect_vec();
    let (p1, p2, p3) = (ranked[i1], ranked[i2], ranked[i3]);

    let get_quartile_color = |d: f32| match d {
        _ if d < p1 => color::GREEN,
        _ if d >= p1 && d < p2 => color::BRIGHT_GREEN,
        _ if d >= p2 && d < p3 => color::YELLOW,
        _ => color::BRIGHT_RED,
    };

    let total = ranked.iter().sum::<f32>();
    let mean = total / (len as f32);
    let median = ranked[i2];

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Year", "Day", "Part", "Title", "Time"]);

    for (key, res) in results {
        let s = key.split(' ').collect_vec();
        let (day, part) = (s[1], s[3]);
        let title = TITLES[(day.parse::<usize>().unwrap() - 1)];
        let dur = res.as_secs_f32();
        table.add_row(Row::new(vec![
            Cell::new("2021"),
            Cell::new(day),
            Cell::new(part),
            Cell::new(title),
            Cell::new(&format!("{:0.04}s", dur))
                .with_style(Attr::ForegroundColor(get_quartile_color(dur))),
        ]));
    }

    table.add_row(Row::new(vec![Cell::new("").with_hspan(5)]));

    table.add_row(Row::new(vec![
        Cell::new_align("Total", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&format!("{:0.04}s", total))
            .with_style(Attr::ForegroundColor(get_quartile_color(total))),
    ]));
    table.add_row(Row::new(vec![
        Cell::new_align("Average", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&format!("{:0.04}s", mean))
            .with_style(Attr::ForegroundColor(get_quartile_color(mean))),
    ]));
    table.add_row(Row::new(vec![
        Cell::new_align("Median", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&format!("{:0.04}s", median))
            .with_style(Attr::ForegroundColor(get_quartile_color(median))),
    ]));

    table.printstd();

    Ok(())
}
