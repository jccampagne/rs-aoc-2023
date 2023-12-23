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

type OriginalHand = Vec<Card>;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Hand {
    FiveOfAKind { cards: OriginalHand },
    FourOfAKind { cards: OriginalHand },
    FullHouse { cards: OriginalHand },
    ThreeOfAKind { cards: OriginalHand },
    TwoPairs { cards: OriginalHand },
    OnePair { cards: OriginalHand },
    HighCard { cards: OriginalHand },
}

#[rustfmt::skip]
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind { cards: s_cards }, Hand::FiveOfAKind { cards: o_cards}) => s_cards.cmp(o_cards),
            (Hand::FiveOfAKind { .. }, _ )                                              => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FourOfAKind { .. },        Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FourOfAKind { cards: s_cards},
             Hand::FourOfAKind { cards: o_cards})                    => s_cards.cmp(o_cards),
            (Hand::FourOfAKind { .. }, _)                            => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::FullHouse { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { cards: s_cards},
             Hand::FullHouse { cards: o_cards})                => s_cards.cmp(o_cards),
            (Hand::FullHouse { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::ThreeOfAKind { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FullHouse { .. })   => Ordering::Less,
            (Hand::ThreeOfAKind { cards: s_cards, ..},
             Hand::ThreeOfAKind { cards: o_cards})                => s_cards.cmp(o_cards),
            (Hand::ThreeOfAKind { .. }, _)                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::TwoPairs { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::TwoPairs { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::TwoPairs { cards: s_cards},
             Hand::TwoPairs { cards: o_cards})                 => s_cards.cmp(o_cards),
            (Hand::TwoPairs { .. }, _ )                        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::OnePair { .. }, Hand::FiveOfAKind { .. })   => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FourOfAKind { .. })   => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FullHouse { .. })     => Ordering::Less,
            (Hand::OnePair { .. }, Hand::ThreeOfAKind { .. })  => Ordering::Less,
            (Hand::OnePair { .. }, Hand::TwoPairs { .. })      => Ordering::Less,
            (Hand::OnePair { cards: s_cards},
             Hand::OnePair { cards: o_cards})                  => s_cards.cmp(o_cards),
            (Hand::OnePair { .. }, Hand::HighCard {..})        => Ordering::Greater,
            // ---------------------------------------------------------------
            (Hand::HighCard { .. }, Hand::FiveOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FourOfAKind  { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::FullHouse    { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::HighCard { .. }, Hand::TwoPairs     { .. }) => Ordering::Less,
            (Hand::HighCard { cards: s_cards},
             Hand::HighCard { cards: o_cards})                 => s_cards.cmp(o_cards),
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
            Hand::FiveOfAKind { cards: orig } => orig,
            Hand::FourOfAKind { cards: orig } => orig,
            Hand::FullHouse { cards: orig } => orig,
            Hand::ThreeOfAKind { cards: orig } => orig,
            Hand::TwoPairs { cards: orig } => orig,
            Hand::OnePair { cards: orig } => orig,
            Hand::HighCard { cards: orig } => orig,
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
    let mut freqs: Vec<Quantity> = Vec::new();
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
            freqs.push(i);

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
    freqs.push(i);

    let orig = input.chars().collect::<Vec<char>>();
    let orig: Result<Vec<Card>, _> = orig.iter().map(|c| Card::try_from(*c)).collect();
    let Ok(orig) = orig else {
        return Err(MyError("oops".into()));
    };

    let hand = couples_to_hand(freqs, orig);

    if total as usize != HAND_SIZE {
        return Err(MyError(format!(
            "wrong number of cards {} - {:?}",
            total, hand
        )));
    }

    return hand;
}
const HAND_SIZE: usize = 5;
fn couples_to_hand(mut couples: Vec<Quantity>, orig: OriginalHand) -> Result<Hand, MyError> {
    couples.sort();
    let couples = couples.into_iter().rev().collect::<Vec<Quantity>>();

    match (
        couples.get(0),
        couples.get(1),
        couples.get(2),
        couples.get(3),
        couples.get(4),
    ) {
        (Some(5), None, None, None, None) => Ok(Hand::FiveOfAKind { cards: orig }),
        (Some(4), Some(1), None, None, None) => Ok(Hand::FourOfAKind { cards: orig }),
        (Some(3), Some(2), None, None, None) => Ok(Hand::FullHouse { cards: orig }),
        (Some(3), Some(1), Some(1), None, None) => Ok(Hand::ThreeOfAKind { cards: orig }),
        (Some(2), Some(2), Some(1), None, None) => Ok(Hand::TwoPairs { cards: orig }),
        (Some(2), Some(1), Some(1), Some(1), None) => Ok(Hand::OnePair { cards: orig }),
        (Some(1), Some(1), Some(1), Some(1), Some(1)) => Ok(Hand::HighCard { cards: orig }),
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
            ("A", Err(MyError("wrong number of cards 1 - Err(MyError(\"ooops: (Some(1), None, None, None, None)\"))".into()))),
            ("33333", Ok(Hand::FiveOfAKind { cards: [C3, C3, C3, C3, C3].into() })),
            ("333333333", Err(MyError("wrong number of cards 9 - Err(MyError(\"ooops: (Some(9), None, None, None, None)\"))".into()))),
            (
                "67345",
                Ok(Hand::HighCard {
                    cards: [C6,C7,C3,C4,C5].into()
                }),
            ),
            (
                "33332",
                Ok(Hand::FourOfAKind {
                    cards: [C3, C3, C3, C3, C2].into()
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
                cards: [C3, C2, CT, C3, CK].into(),
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
        assert!(parse_hand("22333") < parse_hand("33222"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
        assert!(parse_hand("AAAKK") > parse_hand("AAA22"));
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
