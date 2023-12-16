extern crate anyhow;

use anyhow::Result;
use clap::{arg, command, Parser, Subcommand};
use itertools::Itertools;
use prettytable::{color, format::Alignment, row, Attr, Cell, Row, Table};
use solver::{Measure, Solver};

mod common;
mod days;
mod input;
mod solver;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Perf {
        #[arg(short, long)]
        fine: bool,
        #[arg(short, long, default_value_t = 10)]
        iterations: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Perf { fine, iterations }) => perf(fine, iterations)?,
        _ => solve()?,
    }
    Ok(())
}

fn solve() -> Result<()> {
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
    Ok(())
}

fn perf(fine: bool, iterations: u8) -> Result<()> {
    let fmt_func = match fine {
        true => format_fine,
        false => format_rough,
    };
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
    ];
    let count = measures.len();

    println!("Generating performance statistics...");
    println!("{count} solutions, {iterations} runs each.\n");

    let mut results = vec![];
    for (i, m) in measures.iter().enumerate() {
        print!("\rProcessing... {}/{count}", i + 1);
        results.push((
            m.title(),
            m.describe_part_one(),
            m.time_part_one(iterations)?,
        ));
        results.push((
            m.title(),
            m.describe_part_two(),
            m.time_part_two(iterations)?,
        ));
    }
    println!("\nDone.");

    // Get some rough stats
    let len = results.len();
    let (i1, i2, i3) = (len / 2, (len / 4) * 2, (len * 90) / 100);
    let ranked = results
        .iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, _, d)| d.as_secs_f32())
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

    table.set_titles(row!["Year", "Day", "Title", "Part", "Time"]);

    for (title, key, res) in results {
        let s = key.split(' ').collect_vec();
        let (day, part) = (s[1], s[3]);
        let dur = res.as_secs_f32();
        table.add_row(Row::new(vec![
            Cell::new("2023"),
            Cell::new(day),
            Cell::new(&title),
            Cell::new(part),
            Cell::new(&fmt_func(&dur)).with_style(Attr::ForegroundColor(get_quartile_color(dur))),
        ]));
    }

    table.add_row(Row::new(vec![Cell::new("").with_hspan(5)]));

    table.add_row(Row::new(vec![
        Cell::new_align("Total", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&fmt_func(&total)).with_style(Attr::ForegroundColor(get_quartile_color(total))),
    ]));
    table.add_row(Row::new(vec![
        Cell::new_align("Average", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&fmt_func(&mean)).with_style(Attr::ForegroundColor(get_quartile_color(mean))),
    ]));
    table.add_row(Row::new(vec![
        Cell::new_align("Median", Alignment::RIGHT)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new(&fmt_func(&median)).with_style(Attr::ForegroundColor(get_quartile_color(median))),
    ]));

    table.printstd();

    Ok(())
}

fn format_rough(dur: &f32) -> String {
    format!("{dur:0.03}s")
}

fn format_fine(dur: &f32) -> String {
    format!("{dur:0.06}s")
}
