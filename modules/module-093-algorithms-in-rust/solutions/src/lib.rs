//! Module 093 — Algorithms in Rust: sorting, searching, two-pointer.
//!
//! Reference solution. Compare against your `exercises/` implementation
//! after you have made a genuine attempt.

/// Sorts `slice` by value, returning a *new* sorted `Vec` — the input is
/// borrowed, not mutated. O(n log n) worst case; stable.
pub fn merge_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
    if slice.len() <= 1 {
        return slice.to_vec();
    }
    let mid = slice.len() / 2;
    let left = merge_sort(&slice[..mid]);
    let right = merge_sort(&slice[mid..]);
    merge(&left, &right)
}

/// Sorts `slice` in place using quicksort.
pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot = partition(slice);
    quick_sort(&mut slice[..pivot]);
    quick_sort(&mut slice[pivot + 1..]);
}

/// Finds `target` in a *sorted* `slice`, returning one matching index.
/// Returns `None` when absent. O(log n).
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Finds two indices `(i, j)` with `i < j` such that
/// `slice[i] + slice[j] == target` in a *sorted* slice. O(n).
pub fn two_sum_sorted(slice: &[i64], target: i64) -> Option<(usize, usize)> {
    let mut lo = 0;
    let mut hi = slice.len().checked_sub(1)?;
    while lo < hi {
        let sum = slice[lo] + slice[hi];
        if sum == target {
            return Some((lo, hi));
        } else if sum < target {
            lo += 1;
        } else {
            hi -= 1;
        }
    }
    None
}

/// Maximum water a pair of bars can hold. O(n).
pub fn max_water(heights: &[u32]) -> u32 {
    if heights.is_empty() {
        return 0;
    }
    let (mut lo, mut hi) = (0, heights.len() - 1);
    let mut best = 0;
    while lo < hi {
        let area = (hi - lo) as u32 * heights[lo].min(heights[hi]);
        best = best.max(area);
        // Always move the *shorter* bar: only a taller one can improve
        // the area, since the width is only ever shrinking.
        if heights[lo] < heights[hi] {
            lo += 1;
        } else {
            hi -= 1;
        }
    }
    best
}

/// Merges two sorted slices into one sorted `Vec`.
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out.push(left[i].clone());
            i += 1;
        } else {
            out.push(right[j].clone());
            j += 1;
        }
    }
    out.extend_from_slice(&left[i..]);
    out.extend_from_slice(&right[j..]);
    out
}

/// Partitions `slice` around a pivot (the last element), returning the
/// pivot's final index. Lomuto's scheme.
fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot = slice.len() - 1;
    let mut store = 0;
    for i in 0..pivot {
        if slice[i] < slice[pivot] {
            slice.swap(i, store);
            store += 1;
        }
    }
    slice.swap(store, pivot);
    store
}
