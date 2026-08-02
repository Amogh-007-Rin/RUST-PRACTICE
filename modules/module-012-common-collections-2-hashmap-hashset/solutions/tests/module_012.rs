use std::collections::{HashMap, HashSet};

use module_012_solutions::{
    build_scoreboard, count_above, intersection, top_scorer, unique_words, word_frequencies,
};

#[test]
fn build_scoreboard_pairs_players_with_scores() {
    let players = ["alice", "bob", "carol"];
    let scores = [3, 5, 7];
    let board = build_scoreboard(&players, &scores);
    assert_eq!(board.get("alice"), Some(&3));
    assert_eq!(board.get("bob"), Some(&5));
    assert_eq!(board.get("carol"), Some(&7));
}

#[test]
fn build_scoreboard_truncates_to_the_shorter_slice() {
    let players = ["alice", "bob", "carol"];
    let scores = [1];
    let board = build_scoreboard(&players, &scores);
    assert_eq!(board.len(), 1);
    assert_eq!(board.get("alice"), Some(&1));
}

#[test]
fn word_frequencies_counts_repeated_words() {
    let frequencies = word_frequencies("the cat and the dog and the");
    assert_eq!(frequencies.get("the"), Some(&3));
    assert_eq!(frequencies.get("cat"), Some(&1));
    assert_eq!(frequencies.get("dog"), Some(&1));
    assert_eq!(frequencies.len(), 4);
}

#[test]
fn word_frequencies_is_case_sensitive() {
    let frequencies = word_frequencies("The the");
    assert_eq!(frequencies.get("The"), Some(&1));
    assert_eq!(frequencies.get("the"), Some(&1));
}

#[test]
fn word_frequencies_of_empty_text_is_empty() {
    assert!(word_frequencies("").is_empty());
}

#[test]
fn top_scorer_returns_the_best_player() {
    let mut board = HashMap::new();
    board.insert("alice", 3);
    board.insert("bob", 9);
    board.insert("carol", 6);
    assert_eq!(top_scorer(&board), Some("bob"));
}

#[test]
fn top_scorer_of_empty_board_is_none() {
    assert_eq!(top_scorer(&HashMap::new()), None);
}

#[test]
fn unique_words_merges_words_from_all_texts() {
    let texts = ["hello world", "world again", "hello rust"];
    let words = unique_words(&texts);
    assert_eq!(words.len(), 4);
    assert!(words.contains("hello"));
    assert!(words.contains("world"));
    assert!(words.contains("again"));
    assert!(words.contains("rust"));
}

#[test]
fn unique_words_of_empty_input_is_empty() {
    assert!(unique_words(&[]).is_empty());
}

#[test]
fn intersection_keeps_only_shared_elements() {
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let common = intersection(&a, &b);
    assert_eq!(common.len(), 2);
    assert!(common.contains(&3));
    assert!(common.contains(&4));
    assert!(!common.contains(&1));
    assert!(!common.contains(&5));
}

#[test]
fn intersection_of_disjoint_sets_is_empty() {
    let a: HashSet<i32> = [1, 2].into_iter().collect();
    let b: HashSet<i32> = [3, 4].into_iter().collect();
    assert!(intersection(&a, &b).is_empty());
}

#[test]
fn count_above_counts_qualified_scores() {
    let mut board = HashMap::new();
    board.insert("alice", 3);
    board.insert("bob", 9);
    board.insert("carol", 6);
    assert_eq!(count_above(&board, 6), 2);
    assert_eq!(count_above(&board, 100), 0);
}
