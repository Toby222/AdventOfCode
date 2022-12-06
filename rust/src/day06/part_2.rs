const STRING_SLICE_LENGTH: usize = 14;

pub(crate) fn part_2_vec(input: &'static str) -> usize {
    for idx in 0..(input.len() - STRING_SLICE_LENGTH) {
        let mut unique_chars = input[idx..idx + STRING_SLICE_LENGTH]
            .chars()
            .collect::<Vec<_>>();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == STRING_SLICE_LENGTH {
            return idx + STRING_SLICE_LENGTH;
        }
    }
    unreachable!("We should always find an answer");
}

pub(crate) fn part_2_hashset(input: &'static str) -> usize {
    for idx in 0..(input.len() - STRING_SLICE_LENGTH) {
        let unique_chars = input[idx..idx + STRING_SLICE_LENGTH]
            .chars()
            .collect::<std::collections::HashSet<_>>();
        if unique_chars.len() == STRING_SLICE_LENGTH {
            return idx + STRING_SLICE_LENGTH;
        }
    }
    unreachable!("We should always find an answer");
}

pub(crate) fn part_2_nicopap_original(input: &'static str) -> usize {
    nicopap_original::first_sop(input.as_bytes(), STRING_SLICE_LENGTH).unwrap()
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

pub(crate) fn part_2_nicopap_original_without_windows(input: &'static str) -> usize {
    let mut cache = Vec::with_capacity(STRING_SLICE_LENGTH);
    for (i, char) in input.chars().enumerate() {
        cache.insert(0, char);
        if cache.len() >= STRING_SLICE_LENGTH {
            let mut sorted_cache = cache.clone();
            sorted_cache.sort_unstable();
            sorted_cache.dedup();
            if sorted_cache.len() == STRING_SLICE_LENGTH {
                return i + 1;
            }
            cache.pop();
        }
    }
    unreachable!()
}

pub(crate) fn part_2_nicopap_improved(input: &'static str) -> usize {
    nicopap_improved::first_sop(input.as_bytes(), STRING_SLICE_LENGTH).unwrap()
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

pub(crate) fn part_2_nicopap_improved_again(input: &'static str) -> usize {
    nicopap_improved_again::first_sop::<STRING_SLICE_LENGTH>(input.as_bytes()).unwrap()
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

pub(crate) fn part_2_manevillef(input: &'static str) -> usize {
    manevillef::find_marker(&input.chars().collect::<Vec<_>>(), STRING_SLICE_LENGTH).unwrap()
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

pub fn part_2_snaketwix(input: &'static str) -> usize {
    let mut number_of_duplicates = 0;
    let mut char_map = std::collections::HashMap::new();
    let mut char_deque = std::collections::VecDeque::with_capacity(STRING_SLICE_LENGTH + 1);

    let mut ans = 0;

    for (index, char) in input.char_indices() {
        let entry = char_map.entry(char).or_insert(0);
        *entry += 1;

        if *entry > 1 {
            number_of_duplicates += 1;
        }

        char_deque.push_back(char);

        if index > STRING_SLICE_LENGTH - 1 {
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

pub fn part_2_snaketwix_modified(input: &'static str) -> usize {
    let mut number_of_duplicates = 0;
    let mut char_map = std::collections::HashMap::new();
    let mut char_deque = std::collections::VecDeque::with_capacity(STRING_SLICE_LENGTH + 1);

    let mut ans = 0;

    for (index, char) in input.char_indices() {
        let entry = char_map.entry(char).or_insert(0);
        *entry += 1;

        if *entry > 1 {
            number_of_duplicates += 1;
        }

        char_deque.push_back(char);

        if index > STRING_SLICE_LENGTH - 1 {
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

fn part_2_harudagondi(input: &'static str) {
    harudagondi::solve_part2(input);
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

    pub fn solve_part2(input: &'static str) -> usize {
        let mut solver = Solver::new(super::STRING_SLICE_LENGTH);
        for c in input.chars() {
            solver.update(c);
            if let Some(counter) = solver.starter() {
                return counter;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    const SAMPLES: [(&str, usize); 5] = [
        (include_str!("sample_inputs/1.txt"), 19),
        (include_str!("sample_inputs/2.txt"), 23),
        (include_str!("sample_inputs/3.txt"), 23),
        (include_str!("sample_inputs/4.txt"), 29),
        (include_str!("sample_inputs/5.txt"), 26),
    ];

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2_vec(crate::INPUT), 3708);
    }

    #[test]
    fn test_with_sample_solution() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(super::part_2_vec(sample_input), sample_answer);
        }
    }

    #[bench]
    fn bench_part_2_vec(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_vec(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_hashset(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_hashset(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_snaketwix(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_snaketwix(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_snaketwix_modified(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_snaketwix_modified(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_nicopap_original(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_nicopap_original(crate::INPUT));
    }
    #[bench]
    fn bench_part_2_nicopap_original_without_windows(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_nicopap_original_without_windows(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_nicopap_improved(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_nicopap_improved(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_nicopap_improved_again(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_nicopap_improved_again(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_manevillef(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_manevillef(crate::INPUT));
    }

    #[bench]
    fn bench_part_2_harudagondi(bencher: &mut test::Bencher) {
        bencher.iter(|| super::part_2_harudagondi(crate::INPUT));
    }
}
