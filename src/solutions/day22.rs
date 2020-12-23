use std::collections::{HashMap, HashSet};
use std::fs;

fn play_combat(decks: (Vec<u8>, Vec<u8>)) -> (usize, usize) {
    let (mut player_one, mut player_two) = decks;
    loop {
        if player_one.len() == 0 || player_two.len() == 0 {
            break;
        }
        let (card_one, card_two) = (player_one.remove(0), player_two.remove(0));
        if card_one > card_two {
            player_one.push(card_one);
            player_one.push(card_two);
        } else if card_two > card_one {
            player_two.push(card_two);
            player_two.push(card_one);
        } else {
            // ties are undefined behavior, return them to their respective decks
            player_one.push(card_one);
            player_two.push(card_two);
        }
    }
    (score_game(&player_one), score_game(&player_two))
}

fn score_game(deck: &Vec<u8>) -> usize {
    let cards = deck.len();
    if cards == 0 {
        return 0;
    }
    deck.iter()
        .enumerate()
        .map(|(i, v)| *v as usize * (cards - i))
        .sum()
}

fn play_recursive_combat(
    player_one: &mut Vec<u8>,
    player_two: &mut Vec<u8>,
    history: &mut HashMap<(Vec<u8>, Vec<u8>), (usize, usize)>,
) -> (usize, usize) {
    let mut prev_states: HashSet<(Vec<u8>, Vec<u8>)> = HashSet::new();
    let mut score: Option<(usize, usize)> = None;
    loop {
        let game_state = (
            player_one.iter().cloned().collect(),
            player_two.iter().cloned().collect(),
        );
        if let Some(s) = history.get(&game_state) {
            score = Some(*s);
            break;
        }
        if !prev_states.insert(game_state) || player_one.len() == 0 || player_two.len() == 0 {
            score = Some((score_game(&player_one), score_game(&player_two)));
        }
        if let Some(_) = score {
            break;
        }
        let (card_one, card_two) = (player_one.remove(0), player_two.remove(0));
        let (remaining_one, remaining_two) = (player_one.len() as u8, player_two.len() as u8);
        if remaining_one >= card_one && remaining_two >= card_two {
            let state = (
                player_one.iter().cloned().collect(),
                player_two.iter().cloned().collect(),
            );
            let score = match history.get(&state) {
                Some(s) => *s,
                None => {
                    let s = play_recursive_combat(
                        &mut player_one.iter().cloned().collect(),
                        &mut player_two.iter().cloned().collect(),
                        history,
                    );
                    history.insert(state, s.clone());
                    s
                }
            };
            let (score_one, score_two) = score;
            if score_one > score_two {
                player_one.push(card_one);
                player_one.push(card_two);
            } else {
                player_two.push(card_two);
                player_two.push(card_one);
            }
        } else if card_one > card_two {
            player_one.push(card_one);
            player_one.push(card_two);
        } else if card_two > card_one {
            player_two.push(card_two);
            player_two.push(card_one);
        }
    }
    score.unwrap()
}

fn load_data() -> (Vec<u8>, Vec<u8>) {
    let datafile = "data/day22.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let mut decks = read.split("\n\n");
    let player_one = build_deck(decks.next().unwrap());
    let player_two = build_deck(decks.next().unwrap());
    (player_one, player_two)
}

fn build_deck(s: &str) -> Vec<u8> {
    let mut lines = s.lines();
    lines.next(); // Skip the Player header
    lines.map(|l| l.parse::<u8>().unwrap()).collect()
}

pub fn part1() {
    println!("Time for crab.");
    let game = load_data();
    let (score_one, score_two) = play_combat(game);
    println!("\n== Post-game results ==");
    if score_one > score_two {
        println!("Player 1 wins!");
        println!("Score: {}", score_one);
    } else {
        println!("Player 2 wins!");
        println!("Score: {}", score_two);
    }
}
pub fn part2() {
    println!("\nTime for crab: rematch.");
    let mut game_state = load_data();
    let (score_one, score_two) =
        play_recursive_combat(&mut game_state.0, &mut game_state.1, &mut HashMap::new());
    println!("\n== Post-game results ==");
    if score_one > score_two {
        println!("Player 1 wins!");
        println!("Score: {}", score_one);
    } else {
        println!("Player 2 wins!");
        println!("Score: {}", score_two);
    }
}
