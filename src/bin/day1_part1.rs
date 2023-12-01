fn main() {
    /*
    use std::io::{stdin, stdout, BufReader, BufRead, Write};
    use std::fs::File;

    let mut filename = String::new();

    print!("Input filename: ");
    // No newline at the end, so a flush is needed
    stdout().flush()?;

    stdin().read_line(&mut filename)?;

    let file = File::open(filename.trim())?;
    let mut reader = BufReader::new(file);
    let result = solve(&mut reader.lines());

    // This doesn't work because the test function requires
    // Iterator<Item = &str> but the real case requires
    // Iterator<Item = io::Result<String>>.
    //
    // Right now, I don't know how to solve this.
    */

    let input = include_str!("../../inputs/day1-part1.txt");
    let result = solve(&mut input.lines());

    println!("Result: {}", result);
}

// ZERO COST ABSTRACTIONS
// dyn: dynamic dispatch
//    - returns a fat pointer (two pointers): 
//        - one to the data of the struct
//        - other to the vtable (pointers to all its functions)
//    - this might be slower since there are two pointer dereferences.
// impl: static dispatch
//    - the compiler creates several functions, one for each type
//    - if the function is used with a lot of diferent types, it will be reapeated across the
//    executable. In this case, it might be better to use dynamic dispatch instead.
fn solve<'a>(lines: &mut impl Iterator<Item = &'a str>) -> u32 {
    let mut sum = 0;

    for line in lines {
        // Search the first digit
        for char in line.chars() {
            if let Some(n) = char.to_digit(10) {
                sum += 10 * n;
                break;
            }
        }

        // Search the last digit
        for char in line.chars().rev() {
            if let Some(n) = char.to_digit(10) {
                sum += n;
                break;
            }
        }
    }

    sum
}

#[test]
fn test() {
    let test_case = "\
        1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet\
    ";
    assert_eq!(solve(&mut test_case.lines()), 142);
}
