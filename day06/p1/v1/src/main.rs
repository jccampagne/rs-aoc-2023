fn main() {
    // using tests
}

type Number = i64;

#[derive(Debug)]
struct Time(Number);

#[derive(Debug)]
struct Distance(Number);

impl From<Number> for Distance {
    fn from(number: Number) -> Self {
        Distance(number)
    }
}
impl From<Number> for Time {
    fn from(number: Number) -> Self {
        Time(number)
    }
}

fn parse(input: &str) -> Number {
    let mut lines = input.lines();

    let times = parse_times(lines.next().unwrap());
    let dists = parse_distances(lines.next().unwrap());

    let res = compute(times, dists);

    return res;
}

fn parse_times(line: &str) -> Vec<Time> {
    parse_numbers::<Time>(line)
}

fn parse_distances(line: &str) -> Vec<Distance> {
    parse_numbers::<Distance>(line)
}

fn parse_numbers<T>(line: &str) -> Vec<T>
where
    T: From<Number>,
{
    let mut splits = line.split(":");
    let _ = splits.next();
    let numbers_part = splits.next().unwrap();
    let nums = numbers_part.split_whitespace();
    let mut res: Vec<T> = Vec::new();
    for n in nums {
        let r: Number = n.parse::<Number>().unwrap();
        let t: T = T::from(r);
        res.push(t);
    }
    return res;
}

fn compute(times: Vec<Time>, distances: Vec<Distance>) -> Number {
    let mut iter = std::iter::zip(times, distances);
    let mut res = 1;
    for it in iter {
        let r = compute_one(&it.0, &it.1);
        dbg!(&it, r);
        res *= r;
    }

    return res;
}

fn compute_one(Time(t): &Time, Distance(d): &Distance) -> Number {
    let a: Float = 1.0;
    let b: Float = *t as Float;
    let c: Float = *d as Float;
    let d = delta(a, b, c);
    let (x0, x1) = x0x1(a, b, d);

    let x0 = x0 + 1.0;
    let x1 = x1 - 1.0;

    let x0 = Float::floor(x0);
    let x1 = Float::ceil(x1);

    let n = (x1 - x0) as Number + 1;

    return n;
}

type Float = f64;

fn delta(a: Float, b: Float, c: Float) -> Float {
    b * b - 4.0 * a * c
}

fn x0x1(a: Float, b: Float, delta: Float) -> (Float, Float) {
    let sqrtd = Float::sqrt(delta);
    let x0 = (-b - sqrtd) / (2.0 * a);
    let x1 = (-b + sqrtd) / (2.0 * a);
    return (x0, x1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";

        let result = parse(input);
        dbg!(result);
        let expected = 4 * 8 * 9;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_example_1() {
        let input = "\
Time:      71530
Distance:  940200";
        let result = parse(input);
        dbg!(result);
        let expected = 4 * 8 * 9;

        assert_eq!(expected, result);
    }

}
