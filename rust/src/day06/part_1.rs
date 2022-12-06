#[cfg(test)]
mod tests {
    const WINDOW_SIZE: usize = 4;
    use crate::solutions::*;

    const SAMPLES: [(&str, usize); 5] = [
        (include_str!("sample_inputs/1.txt"), 7),
        (include_str!("sample_inputs/2.txt"), 5),
        (include_str!("sample_inputs/3.txt"), 6),
        (include_str!("sample_inputs/4.txt"), 10),
        (include_str!("sample_inputs/5.txt"), 11),
    ];

    #[bench]
    fn bench_vec(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_vec::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_vec_with_solution() {
        assert_eq!(solve_vec::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_vec_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(solve_vec::<WINDOW_SIZE>(sample_input), sample_answer)
        }
    }

    #[bench]
    fn bench_hashset(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_hashset::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_hashset_with_solution() {
        assert_eq!(solve_hashset::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_hashset_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(solve_hashset::<WINDOW_SIZE>(sample_input), sample_answer)
        }
    }

    #[bench]
    fn bench_snaketwix(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_snaketwix::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_snaketwix_with_solution() {
        assert_eq!(solve_snaketwix::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_snaketwix_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(solve_snaketwix::<WINDOW_SIZE>(sample_input), sample_answer)
        }
    }

    #[bench]
    fn bench_snaketwix_modified(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_snaketwix_modified::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_snaketwix_modified_with_solution() {
        assert_eq!(solve_snaketwix_modified::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_snaketwix_modified_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_snaketwix_modified::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_nicopap_original(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_nicopap_original::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_nicopap_original_with_solution() {
        assert_eq!(solve_nicopap_original::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_nicopap_original_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_nicopap_original::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }
    #[bench]
    fn bench_nicopap_original_without_windows(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_nicopap_original_without_windows::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_nicopap_original_without_windows_with_solution() {
        assert_eq!(
            solve_nicopap_original_without_windows::<WINDOW_SIZE>(crate::INPUT),
            1723
        )
    }

    #[test]
    fn test_nicopap_original_without_windows_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_nicopap_original_without_windows::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_nicopap_improved(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_nicopap_improved::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_nicopap_improved_with_solution() {
        assert_eq!(solve_nicopap_improved::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_nicopap_improved_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_nicopap_improved::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_nicopap_improved_again(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_nicopap_improved_again::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_nicopap_improved_again_with_solution() {
        assert_eq!(
            solve_nicopap_improved_again::<WINDOW_SIZE>(crate::INPUT),
            1723
        )
    }

    #[test]
    fn test_nicopap_improved_again_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_nicopap_improved_again::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_manevillef(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_manevillef::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_manevillef_with_solution() {
        assert_eq!(solve_manevillef::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_manevillef_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(solve_manevillef::<WINDOW_SIZE>(sample_input), sample_answer)
        }
    }

    #[bench]
    fn bench_manevillef_again(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_manevillef_again::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_manevillef_again_with_solution() {
        assert_eq!(solve_manevillef_again::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_manevillef_again_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_manevillef_again::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_harudagondi(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_harudagondi::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_harudagondi_with_solution() {
        assert_eq!(solve_harudagondi::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_harudagondi_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_harudagondi::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    /* Requires phf crate
    #[bench]
    fn bench_vertesians(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_vertesians::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_vertesians_with_solution() {
        assert_eq!(solve_vertesians::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_vertesians_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(solve_vertesians::<WINDOW_SIZE>(sample_input), sample_answer)
        }
    }
     */

    #[bench]
    fn bench_vertesians_nodeps(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_vertesians_nodeps::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_vertesians_nodeps_with_solution() {
        assert_eq!(solve_vertesians_nodeps::<WINDOW_SIZE>(crate::INPUT), 1723)
    }

    #[test]
    fn test_vertesians_nodeps_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_vertesians_nodeps::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_vertesians_nodeps_improved(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_vertesians_nodeps_improved::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_vertesians_nodeps_improved_with_solution() {
        assert_eq!(
            solve_vertesians_nodeps_improved::<WINDOW_SIZE>(crate::INPUT),
            1723
        )
    }

    #[test]
    fn test_vertesians_nodeps_improved_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_vertesians_nodeps_improved::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_nicopap_vertesians_nodeps_const(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_nicopap_vertesians_nodeps_const::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_nicopap_vertesians_nodeps_const_with_solution() {
        assert_eq!(
            solve_nicopap_vertesians_nodeps_const::<WINDOW_SIZE>(crate::INPUT),
            1723
        )
    }

    #[test]
    fn test_nicopap_vertesians_nodeps_const_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_nicopap_vertesians_nodeps_const::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }

    #[bench]
    fn bench_vertesians_3_1(bencher: &mut test::Bencher) {
        bencher.iter(|| solve_vertesians_nodeps_3_1::<WINDOW_SIZE>(crate::INPUT));
    }

    #[test]
    fn test_vertesians_3_1_with_solution() {
        assert_eq!(
            solve_vertesians_nodeps_3_1::<WINDOW_SIZE>(crate::INPUT),
            1723
        )
    }

    #[test]
    fn test_vertesians_3_1_with_sample_solutions() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(
                solve_vertesians_nodeps_3_1::<WINDOW_SIZE>(sample_input),
                sample_answer
            )
        }
    }
}
