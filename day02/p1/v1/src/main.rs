use std::{collections::HashMap, env, fs};

//The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
const LIMITS: [(&str, Number); 3] = [("red", 12), ("green", 13), ("blue", 14)];

type LimitMap = HashMap<String, i128>;

type GameId = i128;
type Number = i128;

struct Parser {
    limits: HashMap<String, i128>,
}

impl Parser {
    pub fn compute(&self, input: &str) -> Number {
        let result = input.lines().filter_map(|line| self.parse_line(line)).sum();
        return result;
    }

    fn parse_line(&self, line: &str) -> Option<GameId> {
        let mut parts = line.split(":");
        let part_game_id = parts.next();
        let part_draws = parts.next();
        let Some(part_game_id) = part_game_id else {
            panic!("could not parse game id part {part_game_id:?}");
        };
        let mut part_game_id = part_game_id.split(" ");
        let _ = part_game_id.next();
        let Some(game_id) = part_game_id.next() else {
            panic!("could not parse");
        };
        let Ok(game_id) = game_id.parse::<GameId>() else {
            panic!("could not parse game id");
        };
        let Some(part_draws) = part_draws else {
            panic!("bad input");
        };
        let draws = part_draws.split(";");
        let mut is_bad = 0;
        for draw in draws {
            // dbg!(draw);
            let number_color_list = draw.split(",");
            for nc in number_color_list {
                let nc0 = nc.trim();
                let mut nc1 = nc0.split(" ");
                let Some(number) = nc1.next() else {
                    panic!("bad input");
                };
                let Ok(number) = number.parse::<Number>() else {
                    panic!("could not parse number");
                };
                let Some(color) = nc1.next() else {
                    panic!("bad input");
                };
                dbg!((number, color));
                let Some(max_number) = self.limits.get(color) else {
                    panic!("color not found in limits");
                };
                if number > *max_number {
                    is_bad += 1;
                }
            }
        }
        if is_bad > 0 {
            return None;
        } else {
            return Some(game_id);
        }
    }

    fn from<const N: usize>(list: [(&str, Number); N]) -> Self {
        let mut h: LimitMap = HashMap::new();
        for (k, v) in list {
            h.insert(k.into(), v);
        }
        Parser { limits: h }
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let contents = fs::read_to_string(input_file).unwrap();

    let parser = Parser::from(LIMITS);
    let result = parser.compute(&contents);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected1 = 8;
        let parser = Parser::from(LIMITS);

        let result = parser.compute(input);

        assert_eq!(expected1, result)
    }
}
