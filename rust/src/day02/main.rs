const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

pub(crate) struct CubeCounts {
    red: u16,
    green: u16,
    blue: u16,
}
impl CubeCounts {
    #[must_use]
    const fn power(&self) -> u16 {
        self.red * self.green * self.blue
    }

    #[must_use]
    const fn zero() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    #[must_use]
    const fn red(red: u16) -> Self {
        Self {
            red,
            green: 0,
            blue: 0,
        }
    }

    #[must_use]
    const fn green(green: u16) -> Self {
        Self {
            red: 0,
            green,
            blue: 0,
        }
    }

    #[must_use]
    const fn blue(blue: u16) -> Self {
        Self {
            red: 0,
            green: 0,
            blue,
        }
    }
}

impl std::ops::Add<CubeCounts> for CubeCounts {
    type Output = Self;

    fn add(self, rhs: CubeCounts) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::iter::Sum<CubeCounts> for CubeCounts {
    fn sum<I: Iterator<Item = CubeCounts>>(iter: I) -> Self {
        iter.fold(CubeCounts::zero(), |acc, x| acc + x)
    }
}

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}
