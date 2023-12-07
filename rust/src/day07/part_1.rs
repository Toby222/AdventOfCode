use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Face {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Face::Two => write!(f, "2"),
            Face::Three => write!(f, "3"),
            Face::Four => write!(f, "4"),
            Face::Five => write!(f, "5"),
            Face::Six => write!(f, "6"),
            Face::Seven => write!(f, "7"),
            Face::Eight => write!(f, "8"),
            Face::Nine => write!(f, "9"),
            Face::T => write!(f, "T"),
            Face::J => write!(f, "J"),
            Face::Q => write!(f, "Q"),
            Face::K => write!(f, "K"),
            Face::A => write!(f, "A"),
        }
    }
}

#[derive(Debug)]
struct UnknownFace(char);
impl Display for UnknownFace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Unknown face symbol {}", self.0)
    }
}
impl Error for UnknownFace {}

impl TryFrom<char> for Face {
    type Error = UnknownFace;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Face::A),
            'K' => Ok(Face::K),
            'Q' => Ok(Face::Q),
            'J' => Ok(Face::J),
            'T' => Ok(Face::T),
            '9' => Ok(Face::Nine),
            '8' => Ok(Face::Eight),
            '7' => Ok(Face::Seven),
            '6' => Ok(Face::Six),
            '5' => Ok(Face::Five),
            '4' => Ok(Face::Four),
            '3' => Ok(Face::Three),
            '2' => Ok(Face::Two),
            _ => Err(UnknownFace(value)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Hand {
    FiveOfAKind {
        face: Face,
    },
    FourOfAKind {
        face: Face,
        extra: Face,
    },
    FullHouse {
        triple: Face,
        pair: Face,
    },
    ThreeOfAKind {
        face: Face,
        high_extra: Face,
        low_extra: Face,
    },
    TwoPair {
        higher_pair: Face,
        lesser_pair: Face,
        extra: Face,
    },
    OnePair {
        pair: Face,
        high_extra: Face,
        mid_extra: Face,
        low_extra: Face,
    },
    HighCard {
        face: Face,
        highest_extra: Face,
        high_extra: Face,
        low_extra: Face,
        lowest_extra: Face,
    },
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Hand::FiveOfAKind { face } => write!(f, "{}{}{}{}{}", face, face, face, face, face),
            Hand::FourOfAKind { face, extra } => {
                write!(f, "{}{}{}{}{}", face, face, face, face, extra)
            }
            Hand::FullHouse { triple, pair } => {
                write!(f, "{}{}{}{}{}", triple, triple, triple, pair, pair)
            }
            Hand::ThreeOfAKind {
                face,
                high_extra,
                low_extra,
            } => write!(f, "{}{}{}{}{}", face, face, face, high_extra, low_extra),
            Hand::TwoPair {
                higher_pair,
                lesser_pair,
                extra,
            } => write!(
                f,
                "{}{}{}{}{}",
                higher_pair, higher_pair, lesser_pair, lesser_pair, extra
            ),
            Hand::OnePair {
                pair,
                high_extra,
                mid_extra,
                low_extra,
            } => write!(
                f,
                "{}{}{}{}{}",
                pair, pair, high_extra, mid_extra, low_extra
            ),
            Hand::HighCard {
                face,
                highest_extra,
                high_extra,
                low_extra,
                lowest_extra,
            } => write!(
                f,
                "{}{}{}{}{}",
                face, highest_extra, high_extra, low_extra, lowest_extra
            ),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind { face: lhs }, Hand::FiveOfAKind { face: rhs }) => lhs.cmp(rhs),
            (Hand::FiveOfAKind { .. }, _) => Ordering::Greater,
            (Hand::FourOfAKind { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (
                Hand::FourOfAKind {
                    face: face_lhs,
                    extra: extra_lhs,
                },
                Hand::FourOfAKind {
                    face: face_rhs,
                    extra: extra_rhs,
                },
            ) => match face_lhs.cmp(face_rhs) {
                Ordering::Less => extra_lhs.cmp(extra_rhs),
                cmp => cmp,
            },
            (Hand::FourOfAKind { .. }, _) => Ordering::Greater,
            (Hand::FullHouse { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::FullHouse { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (
                Hand::FullHouse {
                    triple: triple_lhs,
                    pair: pair_lhs,
                },
                Hand::FullHouse {
                    triple: triple_rhs,
                    pair: pair_rhs,
                },
            ) => match triple_lhs.cmp(triple_rhs) {
                Ordering::Less => pair_lhs.cmp(pair_rhs),
                cmp => cmp,
            },
            (Hand::FullHouse { .. }, _) => Ordering::Greater,
            (Hand::ThreeOfAKind { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::ThreeOfAKind { .. }, Hand::FullHouse { .. }) => Ordering::Less,
            (
                Hand::ThreeOfAKind {
                    face: l_face,
                    high_extra: l_high_extra,
                    low_extra: l_low_extra,
                },
                Hand::ThreeOfAKind {
                    face: r_face,
                    high_extra: r_high_extra,
                    low_extra: r_low_extra,
                },
            ) => match l_face.cmp(r_face) {
                Ordering::Equal => match l_high_extra.cmp(r_high_extra) {
                    Ordering::Equal => l_low_extra.cmp(r_low_extra),
                    cmp => cmp,
                },
                cmp => cmp,
            },
            (Hand::ThreeOfAKind { .. }, _) => Ordering::Greater,
            (Hand::TwoPair { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::TwoPair { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::TwoPair { .. }, Hand::FullHouse { .. }) => Ordering::Less,
            (Hand::TwoPair { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (
                Hand::TwoPair {
                    higher_pair: higher_pair_lhs,
                    lesser_pair: lesser_pair_lhs,
                    extra: extra_lhs,
                },
                Hand::TwoPair {
                    higher_pair: higher_pair_rhs,
                    lesser_pair: lesser_pair_rhs,
                    extra: extra_rhs,
                },
            ) => {
                let lhs_max = &Face::max(*higher_pair_lhs, *lesser_pair_lhs);
                let lhs_min = &Face::min(*higher_pair_lhs, *lesser_pair_lhs);

                let rhs_max = &Face::max(*higher_pair_rhs, *lesser_pair_rhs);
                let rhs_min = &Face::min(*higher_pair_rhs, *lesser_pair_rhs);

                match lhs_max.cmp(rhs_max) {
                    Ordering::Equal => match lhs_min.cmp(rhs_min) {
                        Ordering::Equal => extra_lhs.cmp(extra_rhs),
                        cmp => cmp,
                    },
                    cmp => cmp,
                }
            }
            (Hand::TwoPair { .. }, _) => Ordering::Greater,
            (Hand::OnePair { .. }, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FourOfAKind { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::FullHouse { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (Hand::OnePair { .. }, Hand::TwoPair { .. }) => Ordering::Less,
            (
                Hand::OnePair {
                    pair: pair_lhs,
                    high_extra: high_extra_lhs,
                    mid_extra: mid_extra_lhs,
                    low_extra: low_extra_lhs,
                },
                Hand::OnePair {
                    pair: pair_rhs,
                    high_extra: high_extra_rhs,
                    mid_extra: mid_extra_rhs,
                    low_extra: low_extra_rhs,
                },
            ) => match pair_lhs.cmp(pair_rhs) {
                Ordering::Equal => match high_extra_lhs.cmp(high_extra_rhs) {
                    Ordering::Equal => match mid_extra_lhs.cmp(mid_extra_rhs) {
                        Ordering::Equal => low_extra_lhs.cmp(low_extra_rhs),
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
                cmp => cmp,
            },
            (Hand::OnePair { .. }, Hand::HighCard { .. }) => Ordering::Greater,
            (
                Hand::HighCard {
                    face: face_lhs,
                    highest_extra: highest_extra_lhs,
                    high_extra: high_extra_lhs,
                    low_extra: low_extra_lhs,
                    lowest_extra: lowest_extra_lhs,
                },
                Hand::HighCard {
                    face: face_rhs,
                    highest_extra: highest_extra_rhs,
                    high_extra: high_extra_rhs,
                    low_extra: low_extra_rhs,
                    lowest_extra: lowest_extra_rhs,
                },
            ) => match face_lhs.cmp(face_rhs) {
                Ordering::Equal => match highest_extra_lhs.cmp(highest_extra_rhs) {
                    Ordering::Equal => match high_extra_lhs.cmp(high_extra_rhs) {
                        Ordering::Equal => match low_extra_lhs.cmp(low_extra_rhs) {
                            Ordering::Equal => lowest_extra_lhs.cmp(lowest_extra_rhs),
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
                cmp => cmp,
            },
            (Hand::HighCard { .. }, _) => Ordering::Less,
        }
    }
}

impl From<[Face; 5]> for Hand {
    fn from(mut faces: [Face; 5]) -> Hand {
        faces.sort_unstable();
        let groups: Box<[_]> = faces.group_by(|a, b| a == b).collect();
        match groups.len() {
            1 => Hand::FiveOfAKind { face: faces[0] },
            2 => match (groups[0].len(), groups[1].len()) {
                (4, 1) => Hand::FourOfAKind {
                    face: faces[0],
                    extra: faces[4],
                },
                (3, 2) => Hand::FullHouse {
                    triple: faces[0],
                    pair: faces[3],
                },
                (2, 3) => Hand::FullHouse {
                    triple: faces[2],
                    pair: faces[0],
                },
                (1, 4) => Hand::FourOfAKind {
                    face: faces[1],
                    extra: faces[0],
                },
                _ => unreachable!(),
            },
            3 => match (groups[0].len(), groups[1].len(), groups[2].len()) {
                (1, 2, 2) => Hand::TwoPair {
                    higher_pair: faces[3],
                    lesser_pair: faces[1],
                    extra: faces[0],
                },
                (2, 1, 2) => Hand::TwoPair {
                    higher_pair: faces[3],
                    lesser_pair: faces[0],
                    extra: faces[2],
                },
                (2, 2, 1) => Hand::TwoPair {
                    higher_pair: faces[2],
                    lesser_pair: faces[0],
                    extra: faces[4],
                },
                (1, 1, 3) => Hand::ThreeOfAKind {
                    face: faces[4],
                    high_extra: faces[1],
                    low_extra: faces[0],
                },
                (1, 3, 1) => Hand::ThreeOfAKind {
                    face: faces[1],
                    high_extra: faces[4],
                    low_extra: faces[0],
                },
                (3, 1, 1) => Hand::ThreeOfAKind {
                    face: faces[0],
                    high_extra: faces[4],
                    low_extra: faces[3],
                },
                _ => unreachable!(),
            },
            4 => match (
                groups[0].len(),
                groups[1].len(),
                groups[2].len(),
                groups[3].len(),
            ) {
                (1, 1, 1, 2) => Hand::OnePair {
                    pair: faces[3],
                    high_extra: faces[2],
                    mid_extra: faces[1],
                    low_extra: faces[0],
                },
                (1, 1, 2, 1) => Hand::OnePair {
                    pair: faces[2],
                    high_extra: faces[4],
                    mid_extra: faces[1],
                    low_extra: faces[0],
                },
                (1, 2, 1, 1) => Hand::OnePair {
                    pair: faces[1],
                    high_extra: faces[4],
                    mid_extra: faces[3],
                    low_extra: faces[0],
                },
                (2, 1, 1, 1) => Hand::OnePair {
                    pair: faces[0],
                    high_extra: faces[4],
                    mid_extra: faces[3],
                    low_extra: faces[2],
                },
                _ => unreachable!(),
            },
            5 => Hand::HighCard {
                face: faces[4],
                highest_extra: faces[3],
                high_extra: faces[2],
                low_extra: faces[1],
                lowest_extra: faces[0],
            },
            _ => unreachable!(),
        }
    }
}
pub(crate) fn part_1(input: &'static str) -> usize {
    debug_assert!(Face::A > Face::K);
    let mut hands = input
        .lines()
        .map(|line| {
            let mut groups = line.split_ascii_whitespace();
            let faces = groups
                .next()
                .expect("Should be exactly 2 groups")
                .chars()
                .map(Face::try_from)
                .collect::<Result<Vec<_>, _>>()
                .expect("First group shold all be valid faces");
            let hand = <Hand as From<[Face; 5]>>::from(
                faces.try_into().expect("There should always be 5 faces"),
            );
            let bid = groups
                .next()
                .expect("Should be exactly 2 groups")
                .parse::<usize>()
                .expect("Second group should be a valild usize");
            debug_assert!(groups.next().is_none());
            (hand, bid)
        })
        .collect::<Box<[_]>>();
    for (hand, _) in hands.iter() {
        print!("{hand} ");
    }
    println!();
    hands.sort_unstable_by_key(|a| a.0);

    let unique_hands = hands
        .iter()
        .collect::<Box<[_]>>()
        .group_by(|a, b| a.0 == b.0)
        .count();
    debug_assert_eq!(hands.len(), unique_hands);

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (bid * (1 + idx)))
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 6440)
    }

    #[test]
    fn test_with_solution() {
        let result = super::part_1(crate::INPUT);
        assert!(result < 248680906, "{}", result);
        assert_eq!(result, 248179786);
    }
}
