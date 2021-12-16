fn get_syntax_error_score(programs: &Vec<String>) -> usize {
    let mut score = 0;

    for program in programs {
        let mut stack = Vec::new();
        for ch in program.chars() {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                stack.push(ch);
            } else {
                if let Some(opening_ch) = stack.pop() {
                    if ch == ')' && opening_ch != '(' {
                        score += 3;
                    } else if ch == ']' && opening_ch != '[' {
                        score += 57;
                    } else if ch == '}' && opening_ch != '{' {
                        score += 1197;
                    } else if ch == '>' && opening_ch != '<' {
                        score += 25137;
                    }
                }
            }
        }
    }

    score
}

fn get_autocomplete_score(programs: &Vec<String>) -> usize {
    let mut scores = Vec::new();

    for program in programs {
        let mut stack = Vec::new();
        let mut is_valid = true;
        for ch in program.chars() {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                stack.push(ch);
            } else {
                if let Some(opening_ch) = stack.pop() {
                    if opening_ch == '(' && ch != ')' {
                        is_valid = false;
                        break;
                    } else if opening_ch == '[' && ch != ']' {
                        is_valid = false;
                        break;
                    } else if opening_ch == '{' && ch != '}' {
                        is_valid = false;
                        break;
                    } else if opening_ch == '<' && ch != '>' {
                        is_valid = false;
                        break;
                    }
                }
            }
        }

        if is_valid {
            let score = stack.iter().rev().fold(0, |accum, ch| {
                let score = match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                accum * 5 + score
            });
            scores.push(score);
        }
    }

    scores.sort();
    scores[(scores.len() / 2)]
}

fn main() {
    let input = include_str!("../input.txt");

    let programs: Vec<String> = input.lines().map(String::from).collect();

    println!("part 1: {}", get_syntax_error_score(&programs));
    println!("part 2: {}", get_autocomplete_score(&programs));
}
