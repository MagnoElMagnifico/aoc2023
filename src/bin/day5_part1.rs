use sscanf::sscanf;

fn main() {
    println!("Result: {}", solve(include_str!("../../inputs/day5.txt")));
}

fn read_seeds(first_line: &str) -> Vec<u64> {
    let input = sscanf!(first_line, "seeds: {str}").unwrap();

    let mut numbers = vec![];
    let mut current = 0;

    for char in input.chars() {
        if char.is_digit(10) {
            current = 10*current + char.to_digit(10).unwrap() as u64;
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


fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = read_seeds(lines.next().unwrap());
    let mut uses = vec![0; seeds.len()];

    for line in lines {
        match line.chars().next() {
            Some('0' ..= '9') =>  {
                let (output, source, step) = sscanf!(line, "{u64} {u64} {u64}").unwrap();

                for (i, seed) in seeds.iter_mut().enumerate() {
                    if uses[i] > 0 {
                        continue;
                    }


                    if *seed >= source && *seed <= source+step {
                        *seed = output + *seed-source;
                        uses[i] = 1;
                    }
                }
            }
            _ => {
                println!();
                for n in uses.iter_mut() {
                    *n = 0;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

#[test]
fn test_example() {
    let test_case = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
    ";
    assert_eq!(solve(test_case), 35);
}
