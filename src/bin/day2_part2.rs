use sscanf::sscanf;

fn main() {
    println!("Result:  {}", solve(include_str!("../../inputs/day2.txt")));
}

fn solve<'a>(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let (_, results) = sscanf!(line, "Game {}: {}", u32, str).unwrap();

        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

        for selection in results.split(';') {
            for case in selection.split(',') {
                let (_, num, color) = sscanf!(case, "{:/\\s*?/}{} {}", str, u32, str).unwrap();

                match color {
                    "red" if num > max_red => max_red = num,
                    "green" if num > max_green => max_green = num,
                    "blue" if num > max_blue => max_blue = num,
                    _ => continue,
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }

    sum
}

#[test]
fn test() {
    let test_case = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";
    assert_eq!(solve(test_case), 2286);
}

