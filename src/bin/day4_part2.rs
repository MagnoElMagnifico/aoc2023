use std::collections::VecDeque;

use sscanf::sscanf;

fn main() {
    println!("Result: {}", solve(include_str!("../../inputs/day4.txt")));
}

fn sscanf_numbers(input: &str) -> Vec<u32> {
    let mut numbers = vec![];
    let mut current = 0;

    for char in input.chars() {
        if char.is_digit(10) {
            current = 10*current + char.to_digit(10).unwrap();
        } else if char.is_whitespace() && current != 0 {
            numbers.push(current);
            current = 0;
        }
    }

    if current != 0 {
        numbers.push(current);
    }

    numbers
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;
    let mut queue = VecDeque::new();

    for line in input.lines() {
        let (_, card_num, str_numbers, str_winners) = sscanf!(line, "Card{str:/\\s*/}{u32}:{str}|{str}").unwrap();
        let mut points = 0_u32;

        for number in sscanf_numbers(str_numbers) {
            for winner in sscanf_numbers(str_winners) {
                if number == winner {
                    points += 1;
                }
            }
        }

        // Process previous duplicated cards and add 1 for the original card
        let num_total_cards = 1 + queue.pop_front().unwrap_or((0, 0)).1;
        sum += num_total_cards;

        // Add new copies
        for i in 1..=points {
            let mut found = false;

            for (str_card, value) in queue.iter_mut() {
                if *str_card == card_num+i {
                    *value += num_total_cards;
                    found = true;
                    break;
                }
            } 

            if !found {
                queue.push_back((card_num+i, num_total_cards));
            }
        }
    }

    sum
}

#[test]
fn test_example() {
    let test_case = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
    ";
    assert_eq!(solve(test_case), 30);
}