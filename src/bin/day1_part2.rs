const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let input = include_str!("../../inputs/day1.txt");
    let result = solve(&mut input.lines());

    println!("Result: {}", result);
}

fn solve<'a>(lines: &mut impl Iterator<Item = &'a str>) -> u32 {
    // For the sake of learning, let's do it manually

    let mut sum = 0;

    for line in lines {
        // 0: Does not match
        // n: n characters match
        //    if n == NUMBERS[i] => the number i is found
        let mut matches = [0; NUMBERS.len()];

        //// Search the first number ////
        let mut first_num = 0;
        'line: for char in line.chars() {
            // Matches a digit
            if let Some(num) = char.to_digit(10) {
                first_num = num;
                break 'line;
            }

            // Matches a character inside NUMBERS
            for (num, num_str) in NUMBERS.iter().enumerate() {
                // NOTE: Doesn't work when using non ASCII characters
                let num_str_chars = num_str.as_bytes();
                let num_str_first_char = num_str_chars[0] as char;
                let num_str_desired_char = num_str_chars[matches[num]] as char;

                if char == num_str_desired_char {
                    // If the desired char matches, add 1 to the counter
                    matches[num] += 1;
                } else if char == num_str_first_char {
                    // In the case that the char is the first of the number,
                    // it means that the secuence might start again, so we cannot
                    // leave this to 0.
                    matches[num] = 1;
                } else {
                    // Otherwise, reset the counter
                    matches[num] = 0;
                }

                // If all the characters match, then we found a number
                if matches[num] == num_str.len() {
                    first_num = num as u32;
                    break 'line;
                }
            }
        }

        // Reset matches
        matches = [0; NUMBERS.len()];

        //// Search the last number ////
        let mut last_num = 0;
        'line: for char in line.chars().rev() {
            if let Some(num) = char.to_digit(10) {
                last_num = num;
                break 'line;
            }

            // Repeats the same as before, but in reverse
            for (num, num_str) in NUMBERS.iter().enumerate() {
                let num_str_chars = num_str.as_bytes();
                let num_str_last_char = num_str_chars[num_str_chars.len() - 1] as char;
                let num_str_desired_char = num_str_chars[num_str.len() - matches[num] - 1] as char;

                if char == num_str_desired_char {
                    matches[num] += 1;
                } else if char == num_str_last_char {
                    matches[num] = 1;
                } else {
                    matches[num] = 0;
                }

                if matches[num] == num_str.len() {
                    last_num = num as u32;
                    break 'line;
                }
            }
        }

        println!("{} ===> ({}, {})", line, first_num, last_num);

        // Calculate the final number
        sum += 10 * first_num + last_num;
    }

    sum
}

#[test]
fn test() {
    let test_case = "\
        two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\
    ";
    assert_eq!(solve(&mut test_case.lines()), 281);
}

#[test]
fn test2() {
    let test_case = "zvssix1dqb22five";
    assert_eq!(solve(&mut test_case.lines()), 65);
}
