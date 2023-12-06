use std::{iter::zip, cmp::max};

fn main() {
    println!("Result: {}", solve(include_str!("../../inputs/day3.txt")));
}

fn get_number(line: &mut [char], hint: usize) -> u32 {
    let mut pos = hint+1;

    while pos > 0 && line.get(pos-1).unwrap_or(&'.').is_digit(10) {
        pos -= 1;
    }

    let mut number = 0;
    while line.get(pos).unwrap_or(&'.').is_digit(10) {
        number = 10*number + line[pos].to_digit(10).unwrap();
        line[pos] = '.';
        pos += 1;
    }

    number
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    let num_lines = input.lines().count() as i32;
    let cpl = input.lines().next().unwrap().len() as i32;

    let mut matrix: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(num_lines * cpl, matrix.len() as i32);

    for x in 0..num_lines {
        'y: for y in 0..cpl {
            if !matches!(matrix[(x*cpl + y) as usize], '0'..='9' | '.') {
                // Check surroundings of the symbol
                let x_checks = [x*cpl, x*cpl, (x-1)*cpl, (x-1)*cpl, (x-1)*cpl, (x+1)*cpl, (x+1)*cpl, (x+1)*cpl];
                let y_checks = [y-1, y+1, y-1, y, y+1, y-1, y, y+1];

                let mut adj_count = 0;
                let mut mult = 1;

                for (x, y) in zip(x_checks, y_checks) {
                    if adj_count > 2 {
                        continue 'y;
                    }

                    if matrix.get((x + y) as usize).unwrap_or(&'.').is_digit(10) {
                        mult *= get_number(&mut matrix[max(0, x) as usize .. (x+cpl) as usize], y as usize);
                        adj_count += 1;
                    }
                }

                if adj_count == 2 {
                    sum += mult;
                }
            }
        }
    }

    sum
}

#[test]
fn test_example() {
    let test_case = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\n\
    ";
    assert_eq!(solve(test_case), 467835);
}
