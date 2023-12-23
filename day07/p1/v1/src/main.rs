use std::{cmp::Ordering, env, fmt, fs};

type Number = i128;

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

type OriginalHand = [Card; 5];

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Hand {
    FiveOfAKind {
        five: Card,
        orig: OriginalHand,
    },
    FourOfAKind {
        four: Card,
        one: Card,
        orig: OriginalHand,
    },
    FullHouse {
        three: Card,
        two: Card,
        orig: OriginalHand,
    },
    ThreeOfAKind {
        three: Card,
        one_a: Card,
        one_b: Card,
        orig: OriginalHand,
    },
    TwoPairs {
        two_a: Card,
        two_b: Card,
        one: Card,
        orig: OriginalHand,
    },
    OnePair {
        two: Card,
        one_a: Card,
        one_b: Card,
        one_c: Card,
        orig: OriginalHand,
    },
    HighCard {
        one_a: Card,
        one_b: Card,
        one_c: Card,
        one_d: Card,
        one_e: Card,
        orig: OriginalHand,
    },
}

#[rustfmt::skip]
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind { orig: s_orig, .. }, Hand::FiveOfAKind { orig: o_orig, .. }) => s_orig.cmp(o_orig),
            (Hand::FiveOfAKind { .. }, _ )                                           => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FourOfAKind { .. },        Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FourOfAKind { orig: s_orig, .. },
             Hand::FourOfAKind { orig: o_orig, .. })                    => s_orig.cmp(o_orig),
            (Hand::FourOfAKind { .. }, _)                               => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FullHouse { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { orig: s_orig, .. },
             Hand::FullHouse { orig: o_orig, .. })             => s_orig.cmp(o_orig),
            (Hand::FullHouse { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::ThreeOfAKind { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FullHouse { .. })   => Ordering::Less,
            (Hand::ThreeOfAKind { orig: s_orig, ..},
             Hand::ThreeOfAKind { orig: o_orig, .. })             => s_orig.cmp(o_orig),
            (Hand::ThreeOfAKind { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::TwoPairs { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::TwoPairs { orig: s_orig, .. },
             Hand::TwoPairs { orig: o_orig, .. })              => s_orig.cmp(o_orig),
            (Hand::TwoPairs { .. }, _ )                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::OnePair { .. }, Hand::FiveOfAKind { .. })   => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FourOfAKind { .. })   => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FullHouse { .. })     => Ordering::Less,
            (Hand::OnePair { .. }, Hand::ThreeOfAKind { .. })  => Ordering::Less,
            (Hand::OnePair { .. }, Hand::TwoPairs { .. })      => Ordering::Less,
            (Hand::OnePair { orig: s_orig, .. },
             Hand::OnePair { orig: o_orig, .. })               => s_orig.cmp(o_orig),
            (Hand::OnePair { .. }, Hand::HighCard {..})        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::HighCard { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::TwoPairs     { .. }) => Ordering::Less,
            (Hand::HighCard { orig: s_orig, .. },
             Hand::HighCard { orig: o_orig, .. })              => s_orig.cmp(o_orig),
            (Hand::HighCard {..}, _)                           => Ordering::Less
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
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Card::C2 => "2",
            Card::C3 => "3",
            Card::C4 => "4",
            Card::C5 => "5",
            Card::C6 => "6",
            Card::C7 => "7",
            Card::C8 => "8",
            Card::C9 => "9",
            Card::CT => "T",
            Card::CJ => "J",
            Card::CQ => "Q",
            Card::CK => "K",
            Card::CA => "A",
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = match self {
            Hand::FiveOfAKind { orig, .. } => orig,
            Hand::FourOfAKind { orig, .. } => orig,
            Hand::FullHouse { orig, .. } => orig,
            Hand::ThreeOfAKind { orig, .. } => orig,
            Hand::TwoPairs { orig, .. } => orig,
            Hand::OnePair { orig, .. } => orig,
            Hand::HighCard { orig, .. } => orig,
        };
        l.iter().map(|c| c.fmt(f)).collect()
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

    let result = result_2
        .iter()
        .enumerate()
        .map(|(i, pl)| {
            let rank = i + 1;
            let strength = (pl.bid as Number) * (rank as Number);
            println!("{} {} {} = {}", rank, pl.hand, pl.bid, strength);
            strength
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
    let mut orig: Vec<Card> = Vec::new();

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

    let orig = input.chars().collect::<Vec<char>>();
    let orig: Result<Vec<Card>, _> = orig.iter().map(|c| Card::try_from(*c)).collect();
    let Ok(orig) = orig else {
        return Err(MyError("oops".into()));
    };

    let orig: [Card; HAND_SIZE] = orig.try_into().unwrap_or_else(|v: Vec<Card>| {
        panic!(
            "Expected a Vec of length {} but it was {}",
            HAND_SIZE,
            v.len()
        )
    });

    let hand = couples_to_hand(couples, orig);

    if total as usize != HAND_SIZE {
        return Err(MyError(format!(
            "wrong number of cards {} - {:?}",
            total, hand
        )));
    }

    return hand;
}
const HAND_SIZE: usize = 5;
fn couples_to_hand(mut couples: Vec<Couple>, orig: [Card; HAND_SIZE]) -> Result<Hand, MyError> {
    couples.sort();
    let couples = couples.into_iter().rev().collect::<Vec<Couple>>();

    match (
        couples.get(0),
        couples.get(1),
        couples.get(2),
        couples.get(3),
        couples.get(4),
    ) {
        (Some((5, a5)), None, None, None, None) => Ok(Hand::FiveOfAKind { five: *a5, orig }),
        (Some((4, a4)), Some((1, b1)), None, None, None) => Ok(Hand::FourOfAKind {
            four: *a4,
            one: *b1,
            orig,
        }),
        (Some((3, a3)), Some((2, b2)), None, None, None) => Ok(Hand::FullHouse {
            three: *a3,
            two: *b2,
            orig,
        }),
        (Some((3, a3)), Some((1, b1)), Some((1, c1)), None, None) => Ok(Hand::ThreeOfAKind {
            three: *a3,
            one_a: *b1,
            one_b: *c1,
            orig,
        }),
        (Some((2, a2)), Some((2, b2)), Some((1, c1)), None, None) => Ok(Hand::TwoPairs {
            two_a: *a2,
            two_b: *b2,
            one: *c1,
            orig,
        }),
        (Some((2, a2)), Some((1, b1)), Some((1, c1)), Some((1, d1)), None) => Ok(Hand::OnePair {
            two: *a2,
            one_a: *b1,
            one_b: *c1,
            one_c: *d1,
            orig,
        }),
        (Some((1, a1)), Some((1, b1)), Some((1, c1)), Some((1, d1)), Some((1, e1))) => {
            Ok(Hand::HighCard {
                one_a: *a1,
                one_b: *b1,
                one_c: *c1,
                one_d: *d1,
                one_e: *e1,
                orig,
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
            ("A", Err(MyError("wrong number of cards 1 - Err(MyError(\"ooops: (Some((1, CA)), None, None, None, None)\"))".into()))),
            ("33333", Ok(Hand::FiveOfAKind { five: C3 , orig: [C3, C3, C3, C3, C3]})),
            ("333333333", Err(MyError("wrong number of cards 9 - Err(MyError(\"ooops: (Some((9, C3)), None, None, None, None)\"))".into()))),
            (
                "67345",
                Ok(Hand::HighCard {
                    one_a: C7,
                    one_b: C6,
                    one_c: C5,
                    one_d: C4,
                    one_e: C3,
                    orig: [C6,C7,C3,C4,C5]
                }),
            ),
            (
                "33332",
                Ok(Hand::FourOfAKind {
                    four: C3,
                    one: Card::C2,
                    orig: [C3, C3, C3, C3, C2]
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
                orig: [C3, C2, CT, C3, CK],
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
    fn test_refs() {
        let vv: Vec<Hand> = vec![Hand::FiveOfAKind {
            five: C4,
            orig: [C4, C4, C4, C4, C4],
        }];
        let vr: Vec<&Hand> = vec![&Hand::FiveOfAKind {
            five: C4,
            orig: [C4, C4, C4, C4, C4],
        }];
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
        // let expected = 765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5;
        let expected = 6440;
        assert_eq!(expected, result.ok().unwrap());
    }
}
