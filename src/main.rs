use std::{
    collections::HashMap,
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
    let input_path = env::var("aoc_2023_04_path").unwrap() + "/example.txt";
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
    let part2 = calculate_points_with_copies(cards);
    println!("{}", part2)
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
    let mut res = 0;
    let mut copies_v: HashMap<usize, usize> = HashMap::new();
    for card in cards {
        res += 1;
        let cop = copies_v.get(&card.id);
        match cop {
            Some(n) => {
                for _ in 0..*n {
                    for i in 1..card.matches_count + 1 {
                        if copies_v.contains_key(&(card.id + i)) {
                            let v = copies_v.get_mut(&(card.id + i)).unwrap();
                            *v += 1;
                            res += 1;
                        } else {
                            copies_v.insert(card.id + i, 1);
                            res += 1;
                        }
                    }
                }
            }
            None => {}
        };
        for i in 1..card.matches_count + 1 {
            if copies_v.contains_key(&(card.id + i)) {
                let v = copies_v.get_mut(&(card.id + i)).unwrap();
                *v += 1;
                res += 1;
            } else {
                copies_v.insert(card.id + i, 1);
                res += 1;
            }
        }
    }
    println!("{:?}", copies_v);
    return res;
}
