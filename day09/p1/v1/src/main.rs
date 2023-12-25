fn main() {
    println!("Hello, world!");
}

type Number = i32;
type VNumbers = Vec<Number>;

fn parse(input: &str) -> Number {
    let lines = input.lines();
    lines.map(parse_line).sum()
}

fn parse_line(line: &str) -> Number {
    let numbers: Result<Vec<Number>, _> = line
        .split_whitespace()
        .map(|c| c.parse::<Number>())
        .collect();
    let numbers = numbers.unwrap();
    let prediction = predict(&numbers);
    prediction
}

fn predict(numbers: &Vec<Number>) -> Number {
    dbg!(numbers);
    let mut all_diffs: Vec<VNumbers> = Vec::new();
    let mut do_loop = true;
    let mut current: &VNumbers = numbers;
    while do_loop {
        do_loop = false;
        let mut diffs: VNumbers = Vec::new();

        let mut iter = current.iter();
        let mut a = iter.next().unwrap();
        while let Some(b) = iter.next() {
            let d = b - a;
            diffs.push(d);
            if d != 0 {
                do_loop = true
            }
            a = b
        }

        all_diffs.push(diffs);
        current = &all_diffs.last().unwrap()
    }
    dbg!(all_diffs);
    todo!()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = parse(input);
        let expected = 114;
        assert_eq!(result, expected);
    }
}
