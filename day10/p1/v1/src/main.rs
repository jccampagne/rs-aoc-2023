use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let input = fs::read_to_string(input_file).unwrap();
    let result = parse(&input);
    dbg!(result);
}

type Number = i32;

const CNS: char = '|'; //  is a vertical pipe connecting north and south.
const CEW: char = '-'; //  is a horizontal pipe connecting east and west.
const CNE: char = 'L'; //  is a 90-degree bend connecting north and east.
const CNW: char = 'J'; //  is a 90-degree bend connecting north and west.
const CSW: char = '7'; //  is a 90-degree bend connecting south and west.
const CSE: char = 'F'; //  is a 90-degree bend connecting south and east.
const CGR: char = '.'; //  is ground; there is no pipe in this tile.
const CST: char = 'S'; //  is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum MapChar {
    CNS = b'|', //  is a vertical pipe connecting north and south.
    CEW = b'-', //  is a horizontal pipe connecting east and west.
    CNE = b'L', //  is a 90-degree bend connecting north and east.
    CNW = b'J', //  is a 90-degree bend connecting north and west.
    CSW = b'7', //  is a 90-degree bend connecting south and west.
    CSE = b'F', //  is a 90-degree bend connecting south and east.
    CGR = b'.', //  is ground; there is no pipe in this tile.
    CST = b'S', //  is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
}

type Ind = usize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Coord(Ind, Ind);

fn parse(input: &str) -> Number {
    let mut start: Option<Coord> = None;
    let lines = input.lines();
    let mut all_connections: Vec<Conn> = Vec::new();
    for (i_line, line) in lines.enumerate() {
        for (i_col, c) in line.chars().enumerate() {
            eprintln!("=====================================");
            dbg!(i_line, i_col, c);

            let c = match c {
                CNS => Ok(MapChar::CNS),
                CEW => Ok(MapChar::CEW),
                CNE => Ok(MapChar::CNE),
                CNW => Ok(MapChar::CNW),
                CSW => Ok(MapChar::CSW),
                CSE => Ok(MapChar::CSE),
                CGR => Ok(MapChar::CGR),
                CST => Ok(MapChar::CST),
                _ => Err(format!("oops char {}", c)),
            };
            let Ok(c) = c else {
                let e = "";
                std::panic::panic_any(e);
            };
            match c {
                MapChar::CGR => {}
                MapChar::CST => {
                    if let Some(_) = start {
                        panic!("more than 1 start?");
                    }
                    start = Some(Coord(i_line, i_col));
                }
                _ => {
                    let Some(conns) = connections(i_line, i_col, c) else {
                        panic!("should not happen")
                    };
                    all_connections.extend(conns);
                }
            }
        }
    }

    let Some(start) = start else {
        panic!("no start");
    };



    todo!()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Conn(Coord, Coord);

impl Coord {
    fn conn(&self) -> Connector {
        Connector { coord: self }
    }
}

#[derive(Debug)]
struct Connector<'a> {
    coord: &'a Coord,
}

impl<'a> Connector<'a> {
    fn north_south(self) -> [Conn; 2] {
        dbg!(&self);
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i - 1, j);
        let s = Coord(i + 1, j);
        [Conn(n, x.clone()), Conn(x, s)]
    }

    fn east_west(self) -> [Conn; 2] {
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i, j - 1);
        let s = Coord(i, j + 1);
        [Conn(n, x.clone()), Conn(x, s)]
    }

    fn north_east(self) -> [Conn; 2] {
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i - 1, j);
        let s = Coord(i, j + 1);
        [Conn(n, x.clone()), Conn(x, s)]
    }

    fn north_west(self) -> [Conn; 2] {
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i - 1, j);
        let s = Coord(i, j - 1);
        [Conn(n, x.clone()), Conn(x, s)]
    }

    fn south_east(self) -> [Conn; 2] {
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i + 1, j);
        let s = Coord(i, j + 1);
        [Conn(n, x.clone()), Conn(x, s)]
    }

    fn south_west(self) -> [Conn; 2] {
        dbg!(&self);
        let i = self.coord.0;
        let j = self.coord.1;
        let x = Coord(i, j);
        let n = Coord(i + 1, j);
        let s = Coord(i, j - 1);
        [Conn(n, x.clone()), Conn(x, s)]
    }
}

fn connections(i: Ind, j: Ind, c: MapChar) -> Option<[Conn; 2]> {
    let x = Coord(i, j);
    match c {
        MapChar::CNS => Some(x.conn().north_south()),
        MapChar::CEW => Some(x.conn().east_west()),
        MapChar::CNE => Some(x.conn().north_east()),
        MapChar::CNW => Some(x.conn().north_west()),
        MapChar::CSW => Some(x.conn().south_west()),
        MapChar::CSE => Some(x.conn().south_east()),
        MapChar::CGR => None,
        MapChar::CST => None,
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let result = parse(input);
        let expected = 8;
        assert_eq!(result, expected);
    }
}
