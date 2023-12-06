/*
 * This solution will take ages to implement.
 * It is much faster to program a two-pass algorithm
 */
fn main() {
    println!("Result: {}", solve(include_str!("../../inputs/day3.txt")));
}

#[derive(Debug, Clone, Copy)]
enum State {
    None,
    Dot,
    Symbol,
    Number(u32, bool) // the boolean checks if it is already added to the sum
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    let line_len = input.lines().next().unwrap().len();
    let mut prev_line = vec![State::None; line_len];

    let mut state = State::None;
    let mut prev_state = State::None;

    for (line_num, line) in input.lines().enumerate() {

        for (char_num, char) in line.chars().enumerate() {
            println!("{}, {}->{}) Current line: {}\n{}, {}->{}) Prev line: {:?}\n       ) prev_state: {:?}, state {:?}\n",
                line_num, char_num, char, line, line_num, char_num, char, prev_line, prev_state, state);
            
            let write_prev_state;
            (write_prev_state, state) = match char {
                '0'..='9' => {
                    // let match_left  = matches!(prev_line.get(char_num - 1).unwrap_or(&State::Dot), State::Symbol);
                    // let match_right = matches!(prev_line.get(char_num + 1).unwrap_or(&State::Dot), State::Symbol);
                    // let match_top = matches!(prev_line[char_num], State::Symbol);
                    // if match_left || match_top || match_right {
                    //     println!("Found: {} (symbol top)", todo!());
                    // }

                    if let State::Number(n, b) = state {
                        // New digit of previous number
                        let new_state = State::Number(10 * n + char.to_digit(10).unwrap(), b); 

                        // Also update the prev_state with the new found number
                        (new_state, new_state)
                    } else {
                        // First number
                        prev_state = state;
                        (prev_state, State::Number(char.to_digit(10).unwrap(), false))
                    }
                }

                '.' => {
                    // Check sequence: ...*123...
                    if let (State::Symbol, State::Number(n, false)) = (prev_state, state) {
                        println!("Found: {} (symbol before)", n);
                        sum += n;

                        // Update the state
                        state = State::Number(n, true);
                        // TODO: also update previous numbers in prev_line
                    }

                    prev_state = state;
                    (prev_state, State::Dot)
                }

                // Symbols
                _ => {
                    // Check sequence: ...123*...
                    if let State::Number(n, false) = state {
                        println!("Found: {} (symbol after)", n);
                        sum += n;

                        // Update the state
                        state = State::Number(n, true);
                        // TODO: also update previous numbers in prev_line
                    }

                    //                 ..123..
                    // Check sequence: ...*...

                    prev_state = state;
                    (prev_state, State::Symbol)
                }
            };

            // After checking everything, we can update the previous line
            // char_num is unsigned, cast it to i32 because the result may be -1
            if char_num as i32 - 1 >= 0 {
                prev_line[char_num - 1] = write_prev_state;
            }

            // Also write the state if we are at the end of line
            if char_num == line_len-1 {
                prev_line[char_num] = state;
            }
        }

    }

    sum
}

/*
fn update_state(state: State, prev_line: &mut Vec<State>, position: usize) -> State {
    assert!(matches!(state, State::Number(_, false)), "Invalid state!");

    let number = 
    let new_state = State::Number(state.0, true);

    let mut index_start = position;

    while let State::Number(number, _) = prev_line.get(index_start).unwrap_or(&State::Dot) {
        index_start -= 1;
    }

    new_state
}
*/


#[test]
fn test_same_line() {
    let test_case = "\
        11.....15/\n\
        ...$20.*20\n\
        .10#4.....\n\
        ...*23231.\n\
    ";
    assert_eq!(solve(test_case), 69);
}

#[test]
fn test_symbol_above() {
    let test_case = "\
    .....*.........\n\
    ....475........\n\
    ...............\n\
    ";
    assert_eq!(solve(test_case), 475);
}