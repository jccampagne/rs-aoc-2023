use std::{cmp::Ordering, env, fs, path::Display};

type Number = i64;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}
#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Hand {
    FiveOfAKind {
        five: Card,
    },
    FourOfAKind {
        four: Card,
        one: Card,
    },
    FullHouse {
        three: Card,
        two: Card,
    },
    ThreeOfAKind {
        three: Card,
        one_a: Card,
        one_b: Card,
    },
    TwoPairs {
        two_a: Card,
        two_b: Card,
        one: Card,
    },
    OnePair {
        two: Card,
        one_a: Card,
        one_b: Card,
        one_c: Card,
    },
    HighCard {
        one_a: Card,
        one_b: Card,
        one_c: Card,
        one_d: Card,
        one_e: Card,
    },
}

#[rustfmt::skip]
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind { five: five_a }, Hand::FiveOfAKind { five: five_b }) => five_a.cmp(five_b),
            (Hand::FiveOfAKind { .. }, _ )                                           => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FourOfAKind { .. },        Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FourOfAKind { four: s_four, one: s_one },
             Hand::FourOfAKind { four: o_four, one: o_one })
                 => ( s_four, s_one).cmp(
                    &(o_four, o_one)),
            (Hand::FourOfAKind { .. }, _)                               => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FullHouse { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { three: s_three, two: s_two },
             Hand::FullHouse { three: o_three, two: o_two })
                 => (s_three, s_two).cmp(&(o_three, o_two)),
            (Hand::FullHouse { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::ThreeOfAKind { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FullHouse { .. })   => Ordering::Less,
            (Hand::ThreeOfAKind { three: s_three, one_a: s_one_a, one_b: s_one_b},
             Hand::ThreeOfAKind { three: o_three, one_a: o_one_a, one_b: o_one_b })
               => (s_three, s_one_a, s_one_b).cmp(
                 &(o_three, o_one_a, o_one_b)),
            (Hand::ThreeOfAKind { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::TwoPairs { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::TwoPairs { two_a: s_two_a, two_b: s_two_b, one: s_one},
             Hand::TwoPairs { two_a: o_two_a, two_b: o_two_b, one:o_one })
                => (s_two_a, s_two_b, s_one).cmp(
                  &(o_two_a, o_two_b, o_one)),
            (Hand::TwoPairs { .. }, _) => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::OnePair { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FullHouse { .. })   => Ordering::Less,
            (Hand::OnePair { .. }, Hand::ThreeOfAKind {..})  => Ordering::Less,
            (Hand::OnePair { .. }, Hand::TwoPairs { .. })    => Ordering::Less,
            (Hand::OnePair { two: s_two, one_a: s_one_a, one_b: s_one_b, one_c: s_one_c},
             Hand::OnePair { two: o_two, one_a: o_one_a, one_b: o_one_b, one_c: o_one_c})
                =>(s_two, s_one_a, s_one_b, s_one_c).cmp(
                 &(o_two, o_one_a, o_one_b, o_one_c)),
            (Hand::OnePair { .. }, Hand::HighCard {..})      => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::HighCard { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::TwoPairs     { .. }) => Ordering::Less,
            (Hand::HighCard { one_a: s_one_a, one_b: s_one_b, one_c: s_one_c, one_d: s_one_d, one_e: s_one_e },
             Hand::HighCard { one_a: o_one_a, one_b: o_one_b, one_c: o_one_c, one_d: o_one_d, one_e: o_one_e })
                => (s_one_a, s_one_b, s_one_c, s_one_d, s_one_e).cmp(
                  &(o_one_a, o_one_b, o_one_c, o_one_d, o_one_e)),
            (Hand::HighCard {..}, _) => Ordering::Less
            // ---------------------------------------------------------------
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().unwrap();
    let contents = fs::read_to_string(input_file).unwrap();

    let result = parse_input(&contents);
    dbg!(result);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct MyError(String);

type Quantity = u8;
type Couple = (Quantity, Card);

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError(String);

impl TryFrom<char> for Card {
    type Error = ParseCardError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            'T' => Ok(Card::CT),
            'J' => Ok(Card::CJ),
            'Q' => Ok(Card::CQ),
            'K' => Ok(Card::CK),
            'A' => Ok(Card::CA),
            _ => Err(ParseCardError(c.into())),
        }
    }
}

fn parse_input(input: &str) -> Result<Number, MyError> {
    // let result: Vec<Result<ParsedLine, MyError>> = input.lines().map(parse_line).collect();
    let result_0: Result<Vec<ParsedLine>, MyError> = input.lines().map(parse_line).collect();
    let Ok(mut result_2) = result_0 else {
        return Err(result_0.err().unwrap());
    };
    fn comp(lhs: &ParsedLine, rhs: &ParsedLine) -> Ordering {
        let r = lhs.hand.cmp(&rhs.hand);
        r
    }
    result_2.sort_by(comp);
    dbg!(&result_2);
    let result = result_2
        .iter()
        .enumerate()
        .map(|(i, pl)| {
            dbg!(i, pl);
            (pl.bid as Number) * ((i + 1) as Number)
        })
        .sum();

    return Ok(result);
}

fn parse_hand(input: &str) -> Result<Hand, MyError> {
    let mut couples: Vec<Couple> = Vec::new();
    let mut the_chars = input.chars().collect::<Vec<char>>();
    the_chars.sort();
    if the_chars.is_empty() {
        return Err(MyError("empty 1".into()));
    }
    let mut the_chars = the_chars.into_iter();

    let mut current_char = the_chars.next().unwrap();
    let try_from = Card::try_from(current_char);
    let Ok(mut current_card) = try_from else {
        return Err(MyError(format!("try from error {:?}", try_from.err())));
    };
    let mut i: Quantity = 1;
    let mut total: Quantity = 1;
    while let Some(next_char) = the_chars.next() {
        total += 1;
        if current_char == next_char {
            i += 1;
        } else {
            let couple = (i, current_card);
            couples.push(couple);

            let next_card: Result<Card, ParseCardError> = Card::try_from(next_char);
            let Ok(next_card) = next_card else {
                return Result::Err(MyError(format!(
                    "oops could not parse card {:?}",
                    next_card
                )));
            };

            // reset
            current_char = next_char;
            current_card = next_card;
            i = 1;
        }
    }

    let current_card: Result<Card, ParseCardError> = Card::try_from(current_char);
    let Ok(current_card) = current_card else {
        return Result::Err(MyError("oops".into()));
    };
    let couple = (i, current_card);
    couples.push(couple);

    let hand = couples_to_hand(couples);

    if total != 5 {
        return Err(MyError(format!(
            "wrong number of cards {} - {:?}",
            total, hand
        )));
    }

    return hand;
}

fn couples_to_hand(mut couples: Vec<Couple>) -> Result<Hand, MyError> {
    couples.sort();
    let couples = couples.into_iter().rev().collect::<Vec<Couple>>();

    match (
        couples.get(0),
        couples.get(1),
        couples.get(2),
        couples.get(3),
        couples.get(4),
    ) {
        (Some((5, c5)), None, None, None, None) => Ok(Hand::FiveOfAKind { five: *c5 }),
        (Some((4, c4)), Some((1, c1)), None, None, None) => Ok(Hand::FourOfAKind {
            four: *c4,
            one: *c1,
        }),
        (Some((3, c3)), Some((2, c2)), None, None, None) => Ok(Hand::FullHouse {
            three: *c3,
            two: *c2,
        }),
        (Some((3, c3)), Some((1, c1)), Some((1, d1)), None, None) => Ok(Hand::ThreeOfAKind {
            three: *c3,
            one_a: *c1,
            one_b: *d1,
        }),
        (Some((2, c2)), Some((2, d2)), Some((1, d1)), None, None) => Ok(Hand::TwoPairs {
            two_a: *c2,
            two_b: *d2,
            one: *d1,
        }),
        (Some((2, c2)), Some((1, c1)), Some((1, d1)), Some((1, e1)), None) => Ok(Hand::OnePair {
            two: *c2,
            one_a: *c1,
            one_b: *d1,
            one_c: *e1,
        }),
        (Some((1, c1)), Some((1, d1)), Some((1, e1)), Some((1, f1)), Some((1, g1))) => {
            Ok(Hand::HighCard {
                one_a: *c1,
                one_b: *d1,
                one_c: *e1,
                one_d: *f1,
                one_e: *g1,
            })
        }
        x => Err(MyError(format!("ooops: {:?}", x))),
    }
}

type Bid = Number;

#[derive(Debug, PartialEq)]
struct ParsedLine {
    hand: Hand,
    bid: Bid,
}

fn parse_line(input: &str) -> Result<ParsedLine, MyError> {
    let iter = &mut input.split(" ");

    let Some(hand_str) = iter.next() else {
        return Err(MyError("oops".into()));
    };
    let hand = parse_hand(hand_str);
    let Ok(hand) = hand else {
        return Err(hand.err().unwrap());
    };

    let bid_str = iter.next().unwrap();
    let bid = bid_str.parse::<Number>().unwrap();

    Ok(ParsedLine { hand, bid })
}
impl AsRef<Hand> for Hand {
    fn as_ref(&self) -> &Hand {
        return self;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;

    #[test]
    fn test_parse_hand() {
        let testcases = [
            (
                "1",
                Err(MyError("try from error Some(ParseCardError(\"1\"))".into())),
            ),
            ("A", Err(MyError("ooops".into()))),
            ("33333", Ok(Hand::FiveOfAKind { five: C3 })),
            ("333333333", Err(MyError("ooops".into()))),
            (
                "67345",
                Ok(Hand::HighCard {
                    one_a: C7,
                    one_b: C6,
                    one_c: C5,
                    one_d: C4,
                    one_e: C3,
                }),
            ),
            (
                "33332",
                Ok(Hand::FourOfAKind {
                    four: C3,
                    one: Card::C2,
                }),
            ),
        ];

        for (input, expected) in testcases {
            let result = parse_hand(input);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_parse_line() {
        let input = "32T3K 765";
        let expected = Ok(ParsedLine {
            hand: Hand::OnePair {
                two: C3,
                one_a: CK,
                one_b: CT,
                one_c: C2,
            },
            bid: 765,
        });
        let result = parse_line(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_ordering() {
        assert!(parse_hand("22222").unwrap() < parse_hand("AAAAA").unwrap());
        assert!(parse_hand("22222") < parse_hand("AAAAA"));
        assert!(parse_hand("22222") < parse_hand("33333"));
        assert!(parse_hand("22333") == parse_hand("33322"));
        assert!(parse_hand("22333") > parse_hand("33222"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
    }

    #[test]
    fn test_ordering_2() {
        let list: [Hand; 8] = [
            "22222", "KKKKK", "22K22", "3K3K3", "3K3K2", "44KAT", "33567", "2345K",
        ]
        .map(|input| parse_hand(input).unwrap());

        let sorted: Vec<&Hand> = list.iter().collect::<Vec<&Hand>>();

        let expected = [
            &Hand::FiveOfAKind { five: C2 },
            &Hand::FiveOfAKind { five: CK },
            &Hand::FourOfAKind { four: C2, one: CK },
            &Hand::FullHouse { three: C3, two: CK },
            &Hand::TwoPairs {
                two_a: CK,
                two_b: C3,
                one: C2,
            },
            &Hand::OnePair {
                two: C4,
                one_a: CA,
                one_b: CK,
                one_c: CT,
            },
            &Hand::OnePair {
                two: C3,
                one_a: C7,
                one_b: C6,
                one_c: C5,
            },
            &Hand::HighCard {
                one_a: CK,
                one_b: C5,
                one_c: C4,
                one_d: C3,
                one_e: C2,
            },
        ];
        // let expected = expected.map(|h: Hand| &h);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_ordering_3() {
        let list: [Hand; 5] =
            ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|input| parse_hand(input).unwrap());

        let mut hands: Vec<&Hand> = list.iter().collect::<Vec<&Hand>>();
        hands.sort();
        let sorted = hands;
        let expected = [
            &Hand::ThreeOfAKind {
                three: C5,
                one_a: CJ,
                one_b: CT,
            },
            &Hand::ThreeOfAKind {
                three: CQ,
                one_a: CA,
                one_b: CJ,
            },
            &Hand::TwoPairs {
                two_a: CJ,
                two_b: CT,
                one: CK,
            },
            &Hand::TwoPairs {
                two_a: CK,
                two_b: C7,
                one: C6,
            },
            &Hand::OnePair {
                two: C3,
                one_a: CK,
                one_b: CT,
                one_c: C2,
            },
        ];
        // let expected = expected.map(|h: Hand| &h);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_refs() {
        let vv: Vec<Hand> = vec![Hand::FiveOfAKind { five: C4 }];
        let vr: Vec<&Hand> = vec![&Hand::FiveOfAKind { five: C4 }];
        let vvr = vv.iter().map(|h| h.as_ref()).collect::<Vec<&Hand>>(); // why...
        assert_eq!(vvr, vr);
    }

    #[test]
    fn test_parse_input() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = parse_input(input);
        dbg!(&result);
        let expected = 765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5;
        assert_eq!(expected, result.ok().unwrap());
    }
}
