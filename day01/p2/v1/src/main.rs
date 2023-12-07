use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let contents = fs::read_to_string(input_file).unwrap();
    let result = compute(&contents);
    dbg!(result);
}

type Digit = i128;
type Number = i128;

static DIGITS: [(&str, Digit); 19] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

enum BidirRange {
    Fwd(std::ops::Range<usize>),
    Rev(std::iter::Rev<std::ops::Range<usize>>),
}

impl Iterator for BidirRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BidirRange::Fwd(fr) => fr.next(),
            BidirRange::Rev(rr) => rr.next(),
        }
    }
}

pub enum Range {
    Forward(std::ops::Range<usize>),
    Backwards(std::iter::Rev<std::ops::Range<usize>>),
}

impl Iterator for Range {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            Range::Forward(range) => range.next(),
            Range::Backwards(range) => range.next(),
        }
    }
}

fn parse_chars(line: &str, range: BidirRange) -> Option<Number> {
    for offset in range {
        for (ds, di) in DIGITS {
            let sub_line = line.get(offset..);
            if let Some(sub_line) = sub_line {
                if sub_line.starts_with(ds) {
                    return Some(di);
                }
            }
        }
    }
    return None;
}

fn parse_chars_fwd(line: &str) -> Option<Number> {
    let range = BidirRange::Fwd(0..line.len());
    parse_chars(line, range)
}

fn parse_chars_rev(line: &str) -> Option<Number> {
    let range = BidirRange::Rev((0..line.len()).rev());
    parse_chars(line, range)
}

fn parse_line(line: &str) -> Option<Number> {
    let first = parse_chars_fwd(line);
    let last = parse_chars_rev(line);
    match (first, last) {
        (None, None) => todo!(),
        (None, Some(_)) => todo!(),
        (Some(first), None) => return Some(first * 10 + first),
        (Some(first), Some(last)) => return Some(first * 10 + last),
    }
}

fn compute(input: &str) -> i128 {
    let result: Number = input.lines().filter_map(parse_line).sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let expected1 = 12 + 38 + 15 + 77;

        let result = compute(input1.into());
        assert_eq!(expected1, result);
    }

    #[test]
    fn test_example2() {
        let input1 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let expected1 = 29 + 83 + 13 + 24 + 42 + 14 + 76;
        let result = compute(input1.into());
        assert_eq!(expected1, result);
    }
}
