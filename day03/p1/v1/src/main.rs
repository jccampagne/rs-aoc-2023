use regex::Regex;
use std::{env, fs};

#[derive(Debug)]
struct Grid {
    content: String,
    height: i64,
    width: i64,
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let Some(first_line) = lines.next() else {
            panic!("bad input");
        };

        let expected_width_plus_one = first_line.len();
        let mut problem = false;
        let mut content = first_line.to_owned();
        let mut nb_lines = 0;
        while let Some(line) = lines.next() {
            let line_len = line.len();
            if line_len != expected_width_plus_one {
                problem = true;
                break;
            }
            nb_lines += 1;
            content = content + &line[0..line_len];
        }
        if problem {
            panic!("bad input");
        }
        Grid {
            content: content,
            height: nb_lines,
            width: expected_width_plus_one as i64,
        }
    }

    pub fn line(&self, number: i64) -> Option<&str> {
        let start = (number * self.width) as usize;
        let end = ((number + 1) * self.width) as usize;

        return self.content.get(start..end);
    }

    pub fn peek(&self, i_char: i64, j_line: i64) -> Option<&str> {
        let p = (j_line * self.width + i_char) as usize;
        let s = &self.content.get(p..p + 1);
        return *s;
    }
}

#[derive(Debug)]
struct NumberMatch {
    number: i64,
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct CandidatePositionIterator<'a> {
    // num_line: i64,
    // num_start: i64,
    // num_end: i64,
    grid: &'a Grid,

    j_line: i64,
    i_char: i64,

    // j_min: i64,
    j_max: i64,
    i_min: i64,
    i_max: i64,
}

impl<'a> CandidatePositionIterator<'a> {
    fn new(line: i64, start: i64, end: i64, grid: &'a Grid) -> Self {
        let j_min = line - 1;
        let j_max = line + 1;
        let i_min = start - 1;
        let i_max = end ;

        CandidatePositionIterator {
            // num_line: line,
            // num_start: start,
            // num_end: end,
            grid,

            j_line: j_min,
            i_char: i_min,

            // j_min,
            j_max,

            i_min,
            i_max,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    line: i64,
    char: i64,
}

impl<'a> Iterator for CandidatePositionIterator<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        // end
        if self.j_line > self.j_max {
            return None;
        }
        if self.j_line > self.grid.height as i64 {
            return None;
        };

        // new line
        if self.i_char >= self.i_max {
            self.i_char = self.i_min;
            self.j_line += 1;
            return self.next();
        }

        // out of grid bounds checks
        if self.i_char > self.grid.width as i64 {
            self.i_char = self.i_min;
            self.j_line += 1;
            return self.next();
        };
        if self.i_char < 0 {
            self.i_char += 1;
            return self.next();
        };
        if self.j_line < 0 {
            self.j_line += 1;
            self.i_char = self.i_min;
            return self.next();
        };

        let result = Some(Position {
            line: self.j_line,
            char: self.i_char,
        });
        // dbg!(&result);
        self.i_char += 1;
        return result;
    }
}

fn find_numbers_one_line(line: &str) -> Vec<NumberMatch> {
    let re = Regex::new(r"\d+").unwrap();
    let capture_matches = re.captures_iter(line);
    let mut result: Vec<NumberMatch> = Vec::new();
    for captures in capture_matches {
        for m in captures.iter() {
            if let Some(m) = m {
                let Ok(n) = m.as_str().parse::<i64>() else {
                    panic!("oops");
                };
                let r: NumberMatch = NumberMatch {
                    number: n,
                    start: m.start() as i64,
                    end: m.end() as i64 + 1,
                };
                result.push(r);
            } else {
                dbg!("ooops");
            };
        }
    }
    return result;
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let contents = fs::read_to_string(input_file).unwrap();
    let result = parse(&contents);
    dbg!(result);
}

type Number = i64;

fn parse(input: &str) -> Number {
    let g = Grid::from_input(input);
    let mut nline = 0;
    let mut sum = 0;
    for line in input.lines() {
        let numbers = find_numbers_one_line(line);
        for n in numbers {
            let pit = CandidatePositionIterator::new(nline, n.start, n.end, &g);
            for p in pit {
                match g.peek(p.char, p.line) {
                    Some(s) => {
                        if s == "." {
                            continue;
                        }
                        if ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&s) {
                            continue;
                        }
                        // dbg!(s, n.number);
                        sum += n.number;
                        break;
                    }

                    None => continue,
                }
            }
        }
        nline += 1;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_1() {
        dbg!(find_numbers_one_line("..3345...631..6"));
    }

    #[test]
    fn test_generator() {
        //                           012  012
        let grid = Grid::from_input("...\n...\n");
        let it = CandidatePositionIterator::new(1, 1, 3, &grid);
        let c: Vec<Position> = it.collect();
        let expected = Vec::from([
            Position { line: 0, char: 0 },
            Position { line: 0, char: 1 },
            Position { line: 0, char: 2 },
            Position { line: 1, char: 0 },
            Position { line: 1, char: 1 },
            Position { line: 1, char: 2 },
        ]);

        assert_eq!(expected, c);
    }

    #[test]
    fn test_generator_2() {
        let grid = Grid::from_input("...\n...\n...\n...\n...\n...\n...\n");
        let it = CandidatePositionIterator::new(1, 1, 2, &grid);
        let c: Vec<Position> = it.collect();
        let expected = Vec::from([
            Position { line: 0, char: 0 },
            Position { line: 0, char: 1 },
            Position { line: 1, char: 0 },
            Position { line: 1, char: 1 },
            Position { line: 2, char: 0 },
            Position { line: 2, char: 1 },
        ]);

        assert_eq!(expected, c);
    }

    #[test]
    fn test_generator_3() {
        let grid = Grid::from_input(".....\n.....\n.....\n.....\n.....\n.....\n.....\n");
        let it = CandidatePositionIterator::new(1, 1, 2, &grid);
        let c: Vec<Position> = it.collect();
        let expected = Vec::from([
            Position { line: 0, char: 0 },
            Position { line: 0, char: 1 },
            Position { line: 1, char: 0 },
            Position { line: 1, char: 1 },
            Position { line: 2, char: 0 },
            Position { line: 2, char: 1 },
        ]);

        assert_eq!(expected, c);
    }

    #[test]
    fn test_example0() {
        let input = "12345
67890";
        let g = Grid::from_input(input);
        assert_eq!(Some("12345"), g.line(0));
        assert_eq!(Some("67890"), g.line(1));
        assert_eq!(None, g.line(2));
    }

    #[test]
    fn test_example1() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let g = Grid::from_input(input);
        assert_eq!(Some("467..114.."), g.line(0));
        assert_eq!(Some("...*......"), g.line(1));
        assert_eq!(Some(".664.598.."), g.line(9));

        let expected = 4361;
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example2() {
        let input = "\
........954......104.......52......70..............206.806........708..........................217...............................440........
.......@...................*.............................*.664..............677................@....459.........687.........................
..................378.....398........548..495..........983....*................*..282.................*...........$.248.....409.......165...
";

        // "
        // ........954................52..........................806.....................................217..........................................
        // .......@...................*.............................*.664..............677................@....459.........687.........................
        // ..........................398..........................983....*................*......................*...........$.........................
        // "

        let s1 = 954 + 52 + 806 + 217;
        let s2 = 664 + 677 + 459 + 687;
        let s3 = 398 + 983;
        let expected = s1 + s2 + s3;

        let result = parse(input);
        assert_eq!(expected, result);
    }


    #[test]
    fn test_example3() {
        let input = "\
789.-836..
.....#....";
        let expected = 836;
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example4() {
        let input = "\
789.-.....
.....#....";
        let expected = 0;
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
