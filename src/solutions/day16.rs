use anyhow::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Field {
    name: String,
    valid_ranges: Vec<(usize, usize)>,
}
impl Field {
    fn is_valid(&self, value: &usize) -> bool {
        let (lba, uba) = &self.valid_ranges[0];
        let (lbb, ubb) = &self.valid_ranges[1];
        (value >= lba && value <= uba) || (value >= lbb && value <= ubb)
    }
}

enum ValidationResult {
    Valid,
    Invalid(usize),
}

struct PuzzleInput {
    fields: Vec<Field>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}
impl PuzzleInput {
    fn get_error_rate_nearby(&self) -> usize {
        let mut error_rate = 0;
        for ticket in &self.nearby_tickets {
            match self.validate_ticket(ticket) {
                ValidationResult::Invalid(err) => error_rate += err,
                ValidationResult::Valid => continue,
            }
        }
        error_rate
    }

    fn validate_ticket(&self, ticket: &Vec<usize>) -> ValidationResult {
        let mut error_rate = 0;
        for value in ticket {
            let mut is_valid = false;
            for field in &self.fields {
                is_valid = field.is_valid(value);
                if is_valid {
                    break;
                }
            }
            if !is_valid {
                error_rate += value;
            }
        }
        match error_rate {
            0 => ValidationResult::Valid,
            _ => ValidationResult::Invalid(error_rate),
        }
    }

    fn guess_fields(&self) -> Vec<(usize, &'_ Field)> {
        let valid_tickets: &Vec<_> = &self
            .nearby_tickets
            .iter()
            .filter(|&x| matches!(self.validate_ticket(x), ValidationResult::Valid))
            .collect();
        let mut remaining_fields: Vec<_> = self.fields.iter().collect();
        let mut failsafe = 0;
        let mut guesses: HashMap<usize, &Field> = HashMap::new();
        println!("Guessing for {} fields...", remaining_fields.len());
        loop {
            if remaining_fields.len() == 0 {
                break;
            }
            failsafe = remaining_fields.len();
            let mut possibilities: HashMap<usize, Vec<&Field>> = HashMap::new();
            for &field in remaining_fields.iter() {
                for i in 0..self.fields.len() {
                    if guesses.contains_key(&i) {
                        continue;
                    }
                    let mut is_possible = true;
                    for ticket in valid_tickets {
                        if !field.is_valid(&ticket[i]) {
                            is_possible = false;
                            break;
                        }
                    }
                    if is_possible {
                        let plist = possibilities.entry(i).or_insert(vec![]);
                        plist.push(field);
                    }
                }
            }
            for (pos, f) in possibilities.iter().filter(|(_, f)| f.len() == 1) {
                if guesses.contains_key(pos) {
                    panic!("Two fields seem to match position {}", pos);
                }
                let field = f[0];
                guesses.insert(*pos, field);
                if let Some(idx) = remaining_fields.iter().position(|x| x.name == field.name) {
                    remaining_fields.remove(idx);
                }
                println!("Guessed position {} is {}", pos, field.name);
            }
            if remaining_fields.len() == failsafe {
                // println!("Current possibilities: {:#?}", possibilities);
                panic!("Unable to guess!");
            }
        }
        let mut result: Vec<(usize, &Field)> = vec![];
        for guess in guesses {
            result.push(guess);
        }
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }
}

fn load_data() -> Result<PuzzleInput> {
    let datafile = "data/day16.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let mut lines = read.lines();

    let mut fields: Vec<Field> = vec![];
    let my_ticket: Vec<usize>;
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    let field_matcher = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$")?;

    // parse field descriptors, making a lot of assumptions about the input data
    loop {
        let line = lines.next().unwrap();
        let m = match field_matcher.captures(line) {
            Some(c) => c,
            None => break,
        };
        let range_low: (usize, usize) = (m[2].parse()?, m[3].parse()?);
        let range_hi: (usize, usize) = (m[4].parse()?, m[5].parse()?);
        fields.push(Field {
            name: m[1].to_string(),
            valid_ranges: vec![range_low, range_hi],
        })
    }
    lines.next();

    // parse my ticket
    my_ticket = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    lines.next();
    lines.next();

    // parse nearby tickets
    for l in lines {
        nearby_tickets.push(l.split(",").map(|x| x.parse::<usize>().unwrap()).collect());
    }

    Ok(PuzzleInput {
        fields,
        my_ticket,
        nearby_tickets,
    })
}

pub fn part1() {
    let data = load_data().unwrap();
    let error_rate = data.get_error_rate_nearby();
    println!("Answer: {}", error_rate);
}

pub fn part2() {
    let data = load_data().unwrap();
    let guesses = data.guess_fields();
    let mut answer = 1;
    for (pos, _) in guesses
        .iter()
        .filter(|(_, f)| f.name.starts_with("departure"))
    {
        answer *= data.my_ticket[*pos];
    }
    println!("{:?}", answer);
}
