//! Integration tests for Module 093 — sorting, searching, two-pointer.
//!
//! The tests deliberately avoid `rand`: inputs come from a deterministic
//! linear congruential generator (LCG) so every run is reproducible —
//! exactly the property you want in tests.

use module_093_solutions::{binary_search, max_water, merge_sort, quick_sort, two_sum_sorted};

/// Deterministic pseudo-random-ish values, no external crates.
/// With the same seed you always get the same sequence — a failing test can
/// be reproduced forever.
fn lcg_values(seed: u64, count: usize) -> Vec<i64> {
    let mut state = seed;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as i64
        })
        .collect()
}

// ---------------------------------------------------------------------------
// merge_sort
// ---------------------------------------------------------------------------

#[test]
fn merge_sort_empty_and_single() {
    let empty: Vec<i32> = vec![];
    assert_eq!(merge_sort(&empty), Vec::<i32>::new());
    assert_eq!(merge_sort(&[42]), vec![42]);
}

#[test]
fn merge_sort_already_sorted() {
    let input = vec![1, 2, 3, 4, 5];
    assert_eq!(merge_sort(&input), vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_reverse_sorted() {
    let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    assert_eq!(merge_sort(&input), (1..=9).collect::<Vec<i32>>());
}

#[test]
fn merge_sort_with_duplicates() {
    let input = vec![3, 1, 2, 1, 3, 2, 1, 3];
    assert_eq!(merge_sort(&input), vec![1, 1, 1, 2, 2, 3, 3, 3]);
}

#[test]
fn merge_sort_matches_std_sort_on_lcg_inputs() {
    for seed in [1_u64, 7, 42, 1234] {
        let input = lcg_values(seed, 200);
        let mut expected = input.clone();
        expected.sort();
        assert_eq!(merge_sort(&input), expected, "seed {seed}");
    }
}

#[test]
fn merge_sort_does_not_mutate_input() {
    let input = lcg_values(99, 100);
    let copy = input.clone();
    let _ = merge_sort(&input);
    assert_eq!(input, copy, "merge_sort borrows its input");
}

// ---------------------------------------------------------------------------
// quick_sort
// ---------------------------------------------------------------------------

#[test]
fn quick_sort_empty_and_single() {
    let mut empty: Vec<i32> = vec![];
    quick_sort(&mut empty);
    assert!(empty.is_empty());

    let mut single = vec![7];
    quick_sort(&mut single);
    assert_eq!(single, vec![7]);
}

#[test]
fn quick_sort_already_sorted() {
    let mut input = vec![1, 2, 3, 4, 5];
    quick_sort(&mut input);
    assert_eq!(input, vec![1, 2, 3, 4, 5]);
}

#[test]
fn quick_sort_reverse_sorted() {
    let mut input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    quick_sort(&mut input);
    assert_eq!(input, (1..=9).collect::<Vec<i32>>());
}

#[test]
fn quick_sort_with_duplicates() {
    let mut input = vec![5, 2, 5, 1, 2, 5, 1, 2];
    quick_sort(&mut input);
    assert_eq!(input, vec![1, 1, 2, 2, 2, 5, 5, 5]);
}

#[test]
fn quick_sort_matches_std_sort_on_lcg_inputs() {
    for seed in [2_u64, 17, 256] {
        let mut input = lcg_values(seed, 300);
        let mut expected = input.clone();
        expected.sort();
        quick_sort(&mut input);
        assert_eq!(input, expected, "seed {seed}");
    }
}

#[test]
fn quick_sort_is_stable_enough_for_small_strings() {
    let mut input = vec!["pear", "apple", "fig", "apple", "fig", "pear"];
    let mut expected = input.clone();
    expected.sort();
    quick_sort(&mut input);
    assert_eq!(input, expected);
}

// ---------------------------------------------------------------------------
// binary_search
// ---------------------------------------------------------------------------

#[test]
fn binary_search_empty_and_single() {
    let empty: Vec<i32> = vec![];
    assert_eq!(binary_search(&empty, &5), None);
    assert_eq!(binary_search(&[5], &5), Some(0));
    assert_eq!(binary_search(&[5], &6), None);
}

#[test]
fn binary_search_finds_elements_in_lcg_data() {
    for seed in [3_u64, 55, 777] {
        let mut data = lcg_values(seed, 500);
        data.sort();
        // every element must be findable, and the reported index must hold it
        for (idx, value) in data.iter().enumerate() {
            let found = binary_search(&data, value);
            assert_eq!(
                found.map(|i| data[i]),
                Some(*value),
                "seed {seed} idx {idx}"
            );
        }
    }
}

#[test]
fn binary_search_missing_values_return_none() {
    let mut data = lcg_values(9, 300);
    data.sort();
    assert_eq!(binary_search(&data, &(i64::MIN)), None);
    assert_eq!(binary_search(&data, &(i64::MAX)), None);
    // a value strictly between two consecutive elements is absent
    for pair in data.windows(2) {
        if pair[1] - pair[0] > 1 {
            let between = pair[0] + 1;
            assert_eq!(binary_search(&data, &between), None);
            break;
        }
    }
}

#[test]
fn binary_search_handles_duplicates() {
    let data = vec![1, 1, 1, 2, 2, 3, 4, 4, 4, 5];
    for value in [1, 2, 3, 4, 5] {
        let idx = binary_search(&data, &value).expect("value present");
        assert_eq!(data[idx], value);
    }
    assert_eq!(binary_search(&data, &6), None);
}

// ---------------------------------------------------------------------------
// two_sum_sorted
// ---------------------------------------------------------------------------

#[test]
fn two_sum_basic_pairs() {
    let data = vec![2, 7, 11, 15];
    let (i, j) = two_sum_sorted(&data, 9).expect("pair exists");
    assert!(i < j);
    assert_eq!(data[i] + data[j], 9);
}

#[test]
fn two_sum_no_pair_returns_none() {
    let data = vec![1, 2, 3, 4, 5];
    assert_eq!(two_sum_sorted(&data, 100), None);
    assert_eq!(two_sum_sorted(&data, 1), None);
    assert_eq!(two_sum_sorted(&[], 0), None);
    assert_eq!(two_sum_sorted(&[5], 5), None);
}

#[test]
fn two_sum_with_negative_numbers() {
    let data = vec![-8, -3, 1, 2, 4, 9];
    let (i, j) = two_sum_sorted(&data, -2).expect("pair exists");
    assert!(i < j);
    assert_eq!(data[i] + data[j], -2);
}

#[test]
fn two_sum_duplicates_use_distinct_indices() {
    let data = vec![1, 1, 2, 2];
    let (i, j) = two_sum_sorted(&data, 2).expect("pair exists");
    assert!(i < j);
    assert_eq!(data[i] + data[j], 2);
}

#[test]
fn two_sum_cross_checked_against_brute_force() {
    for seed in [5_u64, 91, 2026] {
        let data = lcg_values(seed, 60);
        let mut sorted = data.clone();
        sorted.sort();
        for target in [0_i64, 17, -31, 999_999] {
            let brute: Option<(usize, usize)> = (0..sorted.len())
                .flat_map(|i| ((i + 1)..sorted.len()).map(move |j| (i, j)))
                .find(|&(i, j)| sorted[i] + sorted[j] == target);
            assert_eq!(
                two_sum_sorted(&sorted, target),
                brute,
                "seed {seed} target {target}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// max_water
// ---------------------------------------------------------------------------

#[test]
fn max_water_classic_example() {
    let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_water(&heights), 49);
}

#[test]
fn max_water_edge_cases() {
    assert_eq!(max_water(&[]), 0);
    assert_eq!(max_water(&[7]), 0);
    assert_eq!(max_water(&[3, 5]), 3);
    assert_eq!(max_water(&[9, 1, 1, 9]), 27);
}

#[test]
fn max_water_cross_checked_against_brute_force() {
    for seed in [11_u64, 64, 999] {
        let heights: Vec<u32> = lcg_values(seed, 40)
            .iter()
            .map(|v| (v % 1000) as u32)
            .collect();
        let brute = (0..heights.len())
            .flat_map(|i| ((i + 1)..heights.len()).map(move |j| (i, j)))
            .map(|(i, j)| (j - i) as u32 * heights[i].min(heights[j]))
            .max()
            .unwrap_or(0);
        assert_eq!(max_water(&heights), brute, "seed {seed}");
    }
}
