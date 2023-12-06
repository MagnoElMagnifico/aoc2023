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

    for line in input.lines() {
        let (_, _, str_numbers, str_winners) = sscanf!(line, "Card{str:/\\s*/}{i32}:{str}|{str}").unwrap();
        let mut points = 0_u32;

        for number in sscanf_numbers(str_numbers) {
            for winner in sscanf_numbers(str_winners) {
                if number == winner {
                    points += 1;
                }
            }
        }

        if points == 0 {
            continue;
        }

        sum += 1 << (points - 1);
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
    assert_eq!(solve(test_case), 13);
}

#[test]
fn test_example_line_by_line() {
    assert_eq!(solve("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
    assert_eq!(solve("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
    assert_eq!(solve("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
    assert_eq!(solve("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
    assert_eq!(solve("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
    assert_eq!(solve("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
}