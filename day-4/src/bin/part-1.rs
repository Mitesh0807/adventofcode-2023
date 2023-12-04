use std::fs::File;
use std::io::Read;
fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut values_cards: Vec<u32> = Vec::new();
        let card = line.split(":").collect::<Vec<&str>>();
        let cards = card[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = cards[0].split_whitespace().collect::<Vec<&str>>();
        let player_cards = cards[1].split_whitespace().collect::<Vec<&str>>();
        for card in player_cards {
            if winning_numbers.contains(&card) {
                values_cards.push(card.parse::<u32>().unwrap());
            }
        }
        let sum_value = values_cards.into_iter().enumerate().fold(0, |acc, (i, _)| {
            //  dbg!(i, acc, curr);
            if i == 0 {
                acc + 1
            } else {
                acc * 2
            }
        });
        total += sum_value
    }
    println!("{}", total);
}
