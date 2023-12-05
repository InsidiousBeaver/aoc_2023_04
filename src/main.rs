use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u64>,
    your_numbers: Vec<u64>,
    matches_count: usize,
}
fn main() {
    let mut part1_result = 0;
    let mut cards: Vec<Card> = vec![];
    let input_path = env::var("aoc_2023_04_path").unwrap() + "/input.txt";
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let card = parse_line_to_card(line, i + 1);
        let card_points = calculate_card_points(&card);
        cards.push(card);
        part1_result += card_points;
    }
    // println!("{:?}", cards);
    let part2_result = calculate_points_with_copies(cards);
    println!("part1 result:{}", part1_result);
    println!("part2 result:{}", part2_result)
}
fn parse_line_to_card(line: String, card_id: usize) -> Card {
    let mut card = Card {
        id: card_id,
        winning_numbers: vec![],
        your_numbers: vec![],
        matches_count: 0,
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
    card.matches_count = card_matches(&card);

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
fn card_matches(card: &Card) -> usize {
    let mut count = 0;
    for num in &card.winning_numbers {
        if card.your_numbers.contains(&num) {
            count += 1;
        }
    }
    return count;
}
fn calculate_points_with_copies(cards: Vec<Card>) -> usize {
    let mut result = 0;
    let mut copies_ar: Vec<usize> = Vec::with_capacity(cards.len());
    for _ in 0..cards.len() {
        copies_ar.push(0);
    }
    for card in cards {
        result += 1;
        let copies_count = copies_ar.get(card.id - 1).unwrap();
        for _ in 0..*copies_count {
            for i in 1..card.matches_count + 1 {
                copies_ar[card.id - 1 + i] += 1;
                result += 1;
            }
        }
        for i in 1..card.matches_count + 1 {
            copies_ar[card.id - 1 + i] += 1;
            result += 1;
        }
    }
    return result;
}
