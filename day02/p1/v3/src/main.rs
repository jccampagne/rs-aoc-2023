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

        let game_id = parse_game_id(part_game_id);

        if self.check_all_draws_are_ok(part_draws) {
            return Some(game_id);
        } else {
            return None;
        }
    }

    fn check_all_draws_are_ok(&self, part_draws: Option<&str>) -> bool {
        let Some(part_draws) = part_draws else {
            panic!("bad input");
        };
        let draws = part_draws.split(";");
        let first_bad_draw = draws
            .filter(|x| self.check_one_draw_is_above_limit(x))
            .next();
        match first_bad_draw {
            Some(_) => return false,
            None => return true,
        }
    }

    fn check_one_draw_is_above_limit(&self, draw: &str) -> bool {
        let number_color_list = draw.split(",");
        for number_color in number_color_list {
            let number_color_trimmed = number_color.trim();
            let mut number_color_parts = number_color_trimmed.split(" ");
            let Some(number) = number_color_parts.next() else {
                panic!("bad input");
            };
            let Ok(number) = number.parse::<Number>() else {
                panic!("could not parse number");
            };
            let Some(color) = number_color_parts.next() else {
                panic!("bad input");
            };
            let Some(max_number) = self.limits.get(color) else {
                panic!("color not found in limits");
            };
            if number > *max_number {
                return true;
            }
        }
        return false;
    }

    fn from<const N: usize>(list: [(&str, Number); N]) -> Self {
        let mut h: LimitMap = HashMap::new();
        for (k, v) in list {
            h.insert(k.into(), v);
        }
        Parser { limits: h }
    }
}

fn parse_game_id(part_game_id: Option<&str>) -> GameId {
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
    game_id
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
