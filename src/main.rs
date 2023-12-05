use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<u64>,
    your_numbers: Vec<u64>,
}
fn main() {
    let mut part1_result = 0;
    let input_path = env::var("aoc_2023_04_path").unwrap() + "/example.txt";
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let card = parse_line_to_card(line, i + 1);
        let card_points = calculate_card_points(&card);
        part1_result += card_points;
        println!("{:?}", card);
    }

    println!("{}", part1_result);
}
fn parse_line_to_card(line: String, card_id: usize) -> Card {
    let mut card = Card {
        id: card_id,
        winning_numbers: vec![],
        your_numbers: vec![],
    };
    let line: String = line.chars().skip_while(|x| *x != ':').skip(2).collect();
    let line: Vec<&str> = line.split(|x| x == '|').collect();
    let winning_numbers_unparsed = *line.get(0).unwrap();
    let your_numbers_unparsed = *line.get(1).unwrap();
    let mut number_in_string = String::new();
    for (j, c) in winning_numbers_unparsed.char_indices() {
        if c.is_ascii_digit() {
            number_in_string.push(c);
        }
        if (c == ' ' || j == winning_numbers_unparsed.len() - 1) && !number_in_string.is_empty() {
            card.winning_numbers
                .push(u64::from_str_radix(&number_in_string, 10).unwrap());
            number_in_string.clear();
        }
    }
    for (j, c) in your_numbers_unparsed.char_indices() {
        if c.is_ascii_digit() {
            number_in_string.push(c);
        }
        if (c == ' ' || j == your_numbers_unparsed.len() - 1) && !number_in_string.is_empty() {
            card.your_numbers
                .push(u64::from_str_radix(&number_in_string, 10).unwrap());
            number_in_string.clear();
        }
    }

    return card;
}
fn calculate_card_points(card: &Card) -> u32 {
    let mut your_winning_cards_amount = 1u32;
    for c in &card.your_numbers {
        if card.winning_numbers.contains(&c) {
            your_winning_cards_amount *= 2;
        }
    }
    return your_winning_cards_amount / 2;
}
