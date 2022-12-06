#![allow(dead_code)]

// My original solution
pub(crate) fn solve_vec<const N: usize>(input: &'static str) -> usize {
    for idx in 0..(input.len() - N) {
        let mut unique_chars = input[idx..idx + N].chars().collect::<Vec<_>>();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == N {
            return idx + N;
        }
    }
    unreachable!("We should always find an answer");
}

pub(crate) fn solve_hashset<const N: usize>(input: &'static str) -> usize {
    for idx in 0..(input.len() - N) {
        let unique_chars = input[idx..idx + N]
            .chars()
            .collect::<std::collections::HashSet<_>>();
        if unique_chars.len() == N {
            return idx + N;
        }
    }
    unreachable!("We should always find an answer");
}

pub(crate) fn solve_nicopap_original<const N: usize>(input: &'static str) -> usize {
    nicopap_original::first_sop(input.as_bytes(), N).unwrap()
}

mod nicopap_original {
    fn all_distinct<T: Clone + Ord>(slice: &[T]) -> bool {
        let mut slice_copy = slice.to_vec();
        slice_copy.sort_unstable();
        slice_copy.windows(2).all(|pair| pair[0] != pair[1])
    }
    // sop stands for "start of packet"
    pub(crate) fn first_sop<T: Clone + Ord>(
        datastream: &[T],
        required_distinct: usize,
    ) -> Option<usize> {
        datastream
            .windows(required_distinct)
            .enumerate()
            .find_map(|(i, slice)| all_distinct(slice).then_some(i + required_distinct))
    }
}

pub(crate) fn solve_nicopap_original_without_windows<const N: usize>(input: &'static str) -> usize {
    let mut cache = Vec::with_capacity(N);
    for (i, char) in input.chars().enumerate() {
        cache.insert(0, char);
        if cache.len() >= N {
            let mut sorted_cache = cache.clone();
            sorted_cache.sort_unstable();
            sorted_cache.dedup();
            if sorted_cache.len() == N {
                return i + 1;
            }
            cache.pop();
        }
    }
    unreachable!()
}

pub(crate) fn solve_nicopap_improved<const N: usize>(input: &'static str) -> usize {
    nicopap_improved::first_sop(input.as_bytes(), N).unwrap()
}

mod nicopap_improved {
    fn all_distinct<T: Clone + Ord>(slice: &[T], collect_vec: &mut Vec<T>) -> bool {
        collect_vec.clear();
        slice.clone_into(collect_vec);
        collect_vec.sort_unstable();
        collect_vec.windows(2).all(|pair| pair[0] != pair[1])
    }
    // sop stands for "start of packet"
    pub(crate) fn first_sop<T: Clone + Ord>(
        datastream: &[T],
        required_distinct: usize,
    ) -> Option<usize> {
        let mut collect_vec = Vec::with_capacity(required_distinct);
        datastream
            .windows(required_distinct)
            .enumerate()
            .find_map(|(i, slice)| {
                all_distinct(slice, &mut collect_vec).then_some(i + required_distinct)
            })
    }
}

pub(crate) fn solve_nicopap_improved_again<const N: usize>(input: &'static str) -> usize {
    nicopap_improved_again::first_sop::<N>(input.as_bytes()).unwrap()
}

mod nicopap_improved_again {
    struct AlphabetSet(u32);
    impl AlphabetSet {
        const fn new() -> Self {
            Self(0)
        }
        /// Add letter to set, return `true` if it is already part of it.
        /// `symbol` must be an ascii latin alphabet letter.
        fn add_letter_or_contains(&mut self, symbol: u8) -> bool {
            let to_set = 1 << (symbol.wrapping_sub(b'a') as u32);
            let already_set = self.0 & to_set == to_set;
            self.0 |= to_set;
            already_set
        }
    }
    fn all_distinct(slice: &[u8]) -> bool {
        let mut set = AlphabetSet::new();
        !slice
            .iter()
            .any(|letter| set.add_letter_or_contains(*letter))
    }

    // sop stands for "start of packet"
    pub(crate) fn first_sop<const W: usize>(datastream: &[u8]) -> Option<usize> {
        datastream
            .array_windows::<W>()
            .enumerate()
            .find_map(|(i, slice)| all_distinct(slice).then_some(i + W))
    }
}

pub(crate) fn solve_manevillef<const N: usize>(input: &'static str) -> usize {
    manevillef::find_marker(&input.chars().collect::<Vec<_>>(), N).unwrap()
}

mod manevillef {
    use std::collections::HashSet;

    pub(crate) fn find_marker(chars: &[char], window: usize) -> Option<usize> {
        chars
            .windows(window)
            .into_iter()
            .position(|items| {
                let set: HashSet<char> = items.iter().copied().collect();
                set.len() == window
            })
            .map(|p| p + window)
    }
}

pub(crate) fn solve_manevillef_again<const N: usize>(input: &'static str) -> usize {
    let bytes = input.as_bytes();
    let mut cache = bytes[..N].to_owned();
    for (i, byte) in bytes[N..].iter().enumerate() {
        let mut sorted = cache.clone();
        sorted.sort_unstable();
        sorted.dedup();
        if sorted.len() == N {
            return i + N;
        }
        cache.rotate_left(1);
        cache[N - 1] = *byte;
    }
    unreachable!()
}

pub fn solve_snaketwix<const N: usize>(input: &'static str) -> usize {
    let mut number_of_duplicates = 0;
    let mut char_map = std::collections::HashMap::new();
    let mut char_deque = std::collections::VecDeque::with_capacity(N + 1);

    let mut ans = 0;

    for (index, char) in input.char_indices() {
        let entry = char_map.entry(char).or_insert(0);
        *entry += 1;

        if *entry > 1 {
            number_of_duplicates += 1;
        }

        char_deque.push_back(char);

        if index > N - 1 {
            let char = char_deque.pop_front().unwrap();

            // Know that the entry exists, but not sure how to unwrap the entry to get access to the value
            let entry = char_map.entry(char).or_default();
            *entry -= 1;

            if *entry >= 1 {
                number_of_duplicates -= 1;
            }

            if number_of_duplicates == 0 {
                ans = index + 1;
                break;
            }
        }
    }

    ans
}

pub fn solve_snaketwix_modified<const N: usize>(input: &'static str) -> usize {
    let mut number_of_duplicates = 0;
    let mut char_map = std::collections::HashMap::new();
    let mut char_deque = std::collections::VecDeque::with_capacity(N + 1);

    let mut ans = 0;

    for (index, char) in input.char_indices() {
        let entry = char_map.entry(char).or_insert(0);
        *entry += 1;

        if *entry > 1 {
            number_of_duplicates += 1;
        }

        char_deque.push_back(char);

        if index > N - 1 {
            let char = char_deque.pop_front().unwrap();

            let entry = match char_map.entry(char) {
                std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                std::collections::hash_map::Entry::Vacant(_) => unreachable!(),
            };
            *entry -= 1;

            if *entry >= 1 {
                number_of_duplicates -= 1;
            }

            if number_of_duplicates == 0 {
                ans = index + 1;
                break;
            }
        }
    }

    ans
}

pub fn solve_harudagondi<const N: usize>(input: &'static str) -> usize {
    harudagondi::solve_part2::<N>(input)
}

mod harudagondi {
    use std::collections::{HashSet, VecDeque};

    #[derive(Default)]
    struct Solver {
        buffer: VecDeque<char>,
        counter: usize,
        buffer_size: usize,
    }

    impl Solver {
        fn new(buffer_size: usize) -> Self {
            Self {
                buffer: VecDeque::new(),
                counter: 0,
                buffer_size,
            }
        }

        fn update(&mut self, c: char) {
            if self.buffer.len() < self.buffer_size {
                self.buffer.push_back(c);
            } else {
                self.buffer.pop_front();
                self.buffer.push_back(c);
            }
            self.counter += 1;
        }

        fn starter(&self) -> Option<usize> {
            let buffer = self.buffer.iter().fold(HashSet::new(), |mut acc, c| {
                acc.insert(*c);
                acc
            });

            if buffer.len() == self.buffer_size {
                Some(self.counter)
            } else {
                None
            }
        }
    }

    pub fn solve_part2<const N: usize>(input: &'static str) -> usize {
        let mut solver = Solver::new(N);
        for c in input.chars() {
            solver.update(c);
            if let Some(counter) = solver.starter() {
                return counter;
            }
        }
        unreachable!();
    }
}

/* Requires phf crate
pub fn solve_vertesians<const N: usize>(input: &'static str) -> usize {
    vertesians::main::<N>(input)
}

mod vertesians {
    pub(super) fn main<const N: usize>(input: &'static str) -> usize {
        let data = input
            .chars()
            .map(|c| *SYMBOL_MAP.get(&c).expect("Invalid characters in input"))
            .collect::<Vec<u32>>();

        assert!(!data.is_empty(), "Received empty input");

        process::<N>(data)
    }

    fn process<const N: usize>(data: Vec<u32>) -> usize {
        let mut counter = 0;

        loop {
            let combined = {
                let mut combined = 0;

                for i in 0..N {
                    combined |= data[counter + i];
                }

                combined
            };

            if check::<N>(combined) {
                return counter + N;
            } else {
                counter += 1;
            }
        }
    }

    static SYMBOL_MAP: phf::Map<char, u32> = phf::phf_map! {
        'a' => 0b0000_0000_0000_0000_0000_0000_0000_0001,
        'b' => 0b0000_0000_0000_0000_0000_0000_0000_0010,
        'c' => 0b0000_0000_0000_0000_0000_0000_0000_0100,
        'd' => 0b0000_0000_0000_0000_0000_0000_0000_1000,
        'e' => 0b0000_0000_0000_0000_0000_0000_0001_0000,
        'f' => 0b0000_0000_0000_0000_0000_0000_0010_0000,
        'g' => 0b0000_0000_0000_0000_0000_0000_0100_0000,
        'h' => 0b0000_0000_0000_0000_0000_0000_1000_0000,
        'i' => 0b0000_0000_0000_0000_0000_0001_0000_0000,
        'j' => 0b0000_0000_0000_0000_0000_0010_0000_0000,
        'k' => 0b0000_0000_0000_0000_0000_0100_0000_0000,
        'l' => 0b0000_0000_0000_0000_0000_1000_0000_0000,
        'm' => 0b0000_0000_0000_0000_0001_0000_0000_0000,
        'n' => 0b0000_0000_0000_0000_0010_0000_0000_0000,
        'o' => 0b0000_0000_0000_0000_0100_0000_0000_0000,
        'p' => 0b0000_0000_0000_0000_1000_0000_0000_0000,
        'q' => 0b0000_0000_0000_0001_0000_0000_0000_0000,
        'r' => 0b0000_0000_0000_0010_0000_0000_0000_0000,
        's' => 0b0000_0000_0000_0100_0000_0000_0000_0000,
        't' => 0b0000_0000_0000_1000_0000_0000_0000_0000,
        'u' => 0b0000_0000_0001_0000_0000_0000_0000_0000,
        'v' => 0b0000_0000_0010_0000_0000_0000_0000_0000,
        'w' => 0b0000_0000_0100_0000_0000_0000_0000_0000,
        'x' => 0b0000_0000_1000_0000_0000_0000_0000_0000,
        'y' => 0b0000_0001_0000_0000_0000_0000_0000_0000,
        'z' => 0b0000_0010_0000_0000_0000_0000_0000_0000,
    };

    fn check<const N: usize>(combined: u32) -> bool {
        let mut flipped_count = 0;

        for position in 0..26 {
            let filtered = combined & !(1 << position);

            if filtered != combined {
                flipped_count += 1;
            }

            if flipped_count == N {
                return true;
            }
        }

        false
    }
}
*/

pub fn solve_vertesians_nodeps<const N: usize>(input: &'static str) -> usize {
    vertesians_nodeps::main::<N>(input)
}

mod vertesians_nodeps {
    pub(super) fn main<const N: usize>(input: &'static str) -> usize {
        let data = input
            .chars()
            .map(|c| 1 << (c as u32 - 0x61))
            .collect::<Vec<u32>>();

        assert!(!data.is_empty(), "Received empty input");

        process::<N>(data)
    }

    fn process<const N: usize>(data: Vec<u32>) -> usize {
        let mut counter = 0;

        loop {
            let combined = {
                let mut combined = 0;

                for i in 0..N {
                    combined |= data[counter + i];
                }

                combined
            };

            if check::<N>(combined) {
                return counter + N;
            } else {
                counter += 1;
            }
        }
    }

    fn check<const N: usize>(combined: u32) -> bool {
        let mut flipped_count = 0;

        for position in 0..26 {
            let filtered = combined & !(1 << position);

            if filtered != combined {
                flipped_count += 1;
            }

            if flipped_count == N {
                return true;
            }
        }

        false
    }
}

pub fn solve_vertesians_nodeps_improved<const N: usize>(input: &'static str) -> usize {
    vertesians_nodeps_improved::main::<N>(input)
}

mod vertesians_nodeps_improved {
    pub(super) fn main<const N: usize>(input: &'static str) -> usize {
        let data = input
            .chars()
            .map(|c| 1 << (c as u32 - 0x61))
            .collect::<Vec<u32>>();

        assert!(!data.is_empty(), "Received empty input");

        process::<N>(data)
    }

    fn process<const N: usize>(data: Vec<u32>) -> usize {
        let mut counter = 0;

        loop {
            let combined = {
                let mut combined = 0;

                for i in 0..N {
                    combined |= data[counter + i];
                }

                combined
            };

            // Thanks to Gibonius for the suggestion to use u32::count_ones()
            if (combined & u32::MAX).count_ones() == N as u32 {
                return counter + N;
            } else {
                counter += 1;
            }
        }
    }
}

pub const fn solve_nicopap_vertesians_nodeps_const<const N: usize>(input: &'static str) -> usize {
    // For 0ns results
    // match N {
    //     4 => nicopap_vertesians_const::RESULT_PART_1,
    //     14 => nicopap_vertesians_const::RESULT_PART_2,
    //     _ => unreachable!(),
    // }
    nicopap_vertesians_nodeps_const::process::<N>(input.as_bytes())
}

mod nicopap_vertesians_nodeps_const {
    pub(super) const RESULT_PART_1: usize = process::<4>(include_bytes!("./input.txt"));
    pub(super) const RESULT_PART_2: usize = process::<14>(include_bytes!("./input.txt"));

    pub(super) const fn process<const N: usize>(data: &'static [u8]) -> usize {
        let mut counter = 0;

        loop {
            let combined = {
                let mut combined = 0;
                let mut i = 0;
                loop {
                    if i == N {
                        break;
                    }
                    combined |= 1 << (data[counter + i] - b'a') as u32;
                    i += 1;
                }
                combined
            };

            if check::<N>(combined) {
                return counter + N;
            } else {
                counter += 1;
            }
        }
    }

    const fn check<const N: usize>(combined: u32) -> bool {
        let flipped_count = (combined & 0b11_1111_1111_1111_1111_1111_1111).count_ones();
        flipped_count == (N as u32)
    }
}

pub fn solve_vertesians_nodeps_3_1<const N: usize>(input: &'static str) -> usize {
    vertesians_nodeps_3_1::main::<N>(input)
}

mod vertesians_nodeps_3_1 {
    pub(super) fn main<const N: usize>(input: &'static str) -> usize {
        let data = input
            .chars()
            .map(|c| 1 << (c as u32 - 0x61))
            .collect::<Vec<u32>>();

        assert!(!data.is_empty(), "Received empty input");

        process::<N>(data)
    }

    fn process<const N: usize>(data: Vec<u32>) -> usize {
        for counter in 0.. {
            let combined = {
                let mut combined = 0;
                for i in 0..N {
                    combined |= data[counter + i];
                }
                combined
            };

            // Thanks to Gibonius for the suggestion to use u32::count_ones()
            if combined.count_ones() == N as u32 {
                return counter + N;
            }
        }

        panic!("No valid sequence found");
    }
}
