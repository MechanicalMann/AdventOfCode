use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_allergens(data: &Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for (allergen_list, text) in data {
        for allergen in allergen_list.split(", ") {
            let words: HashSet<_> = text.split(" ").map(|i| i.to_string()).collect();
            let ingr: HashSet<_> = match allergens.get(allergen) {
                Some(a) => a.intersection(&words).cloned().collect(),
                None => words,
            };
            allergens.insert(allergen.to_string(), ingr);
        }
    }

    loop {
        allergens = guess_allergens(&allergens);
        let guessed = allergens.values().filter(|a| a.len() > 1).count();
        // loop until all have been guessed (yes, this does rely on the data being guessable...)
        if guessed == 0 {
            break;
        }
    }

    allergens
}

fn guess_allergens(data: &HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>> {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let guessed: Vec<_> = data.iter().filter(|&(_, a)| a.len() == 1).collect();
    for (allergen, allergen_ingredients) in guessed {
        allergens.insert(
            allergen.to_string(),
            allergen_ingredients.iter().cloned().collect(),
        );
        for (key, ingredients) in data.iter().filter(|&(a, _)| a != allergen) {
            let ingr: HashSet<_> = ingredients
                .difference(allergen_ingredients)
                .cloned()
                .collect();
            allergens.insert(key.to_string(), ingr);
        }
    }
    allergens
}

fn get_safe_ingredients(
    data: &Vec<(String, String)>,
    allergens: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    let mut ingredients: HashSet<String> = HashSet::new();

    for (_, text) in data {
        let words: HashSet<_> = text.split(" ").map(|i| i.to_string()).collect();
        ingredients.extend(words);
    }

    let safe: HashSet<_> = ingredients
        .difference(&allergens.values().flatten().cloned().collect())
        .cloned()
        .collect();
    safe
}

fn load_data() -> Vec<(String, String)> {
    let datafile = "data/day21.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    lazy_static! {
        static ref ALLERGEN_RE: Regex = Regex::new(r"(?m)^(.+)\s\(contains (.+)\)$").unwrap();
    }

    let mut data: Vec<(String, String)> = vec![];
    for cap in ALLERGEN_RE.captures_iter(&read) {
        data.push((cap[2].to_string(), cap[1].to_string()));
    }
    data
}

pub fn part1() {
    let data = load_data();
    let allergens = get_allergens(&data);
    let safe = get_safe_ingredients(&data, &allergens);
    let mut count_safe = 0;
    for (_, ingredients) in &data {
        for i in &safe {
            count_safe += ingredients.split(" ").filter(|s| s == i).count();
        }
    }
    println!("Answer: {}", count_safe);
}
pub fn part2() {
    let data = load_data();
    let allergens = get_allergens(&data);
    let mut list: Vec<_> = allergens.iter().collect();
    list.sort_by(|&(ka, _), &(kb, _)| ka.cmp(kb));
    let values: Vec<_> = list.iter().map(|&(_, v)| v).flatten().cloned().collect();
    println!("Answer: {}", values.join(","));
}
