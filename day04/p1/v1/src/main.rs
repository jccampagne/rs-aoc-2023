use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

type Number = i32;

fn parse_line(line: &str) -> Number {
    println!("========= line = {line}");
    let mut parts = line.split(":");
    let _game_id = parts.next();
    let num_part = parts.next().unwrap().trim();
    let mut numbers = num_part.split(" ");
    let mut winning = HashSet::new();
    while let Some(n) = numbers.next() {
        if n == "|" { break;}
        winning.insert(n);
    }
    let mut score : i32 = 0;
    while let Some(n) = numbers.next() {
        if winning.contains(n) {
            if score == 0 {
                score = 1
            } else {
                score *= 2
            }
            println!("winning {n:?}");
        }
    }
    println!("score = {score}");
    return score;
}

fn parse(input: &str) -> Number {
    let result = input.lines().map(parse_line).sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = parse(input);
        let expected = 13;

        assert_eq!(result, expected)
    }
}
