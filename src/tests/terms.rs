//! A module that handles unit testing for terms
#![allow(unused)]

use terms::*;

/// Tests the most basic term creation example
#[test]
fn create() {
    let mut vec: Vec<Symbol> = Vec::new();
    vec.push(symbol!["A"]);
    vec.push(symbol!["!🐎"]);
    vec.push(symbol!["🍝"]);
    vec.push(symbol!["!📰"]);

    let term = Term { symbols: vec };
    assert!(term.contains(&symbol!["A"]));
    assert!(term.contains(&symbol!["!🐎"]));
    assert!(term.contains(&symbol!["🍝"]));
    assert!(term.contains(&symbol!["!📰"]));
}

/// Tests simple term creation via a utility macro
#[test]
fn create_macro() {
    let term = term!["A", "!🐎", "🍝", "!📰"];

    assert!(term.contains(&symbol!["A"]));
    assert!(term.contains(&symbol!["!🐎"]));
    assert!(term.contains(&symbol!["🍝"]));
    assert!(term.contains(&symbol!["!📰"]));
}

/// Create a term, then remove symbols from it
#[test]
fn remove() {
    let mut term = term!["A", "!🐎", "🍝", "!📰"];

    /* Try to remove something that doesn't exist */
    term.remove(symbol!["!A"]);

    term.remove(symbol!["A"]);
    assert!(! term.contains(&symbol!["A"]));

    term.remove(symbol!["!🐎"]);
    assert!(! term.contains(&symbol!["!🐎"]));
    
    term.remove(symbol!["🍝"]);
    assert!(! term.contains(&symbol!["🍝"]));

    term.remove(symbol!["!📰"]);
    assert!(! term.contains(&symbol!["!📰"]));
}

/// Create a term, then insert further symbols
#[test]
fn insert() {
    let mut term = term!["A"];
    term.insert(symbol!["!A"]);
    term.insert(symbol!["B"]);

    assert!(term.contains(&symbol!["A"]));
    assert!(term.contains(&symbol!["!A"]));
    assert!(term.contains(&symbol!["B"]));

    /* Fail an insert */
    term.insert(symbol!["B"]);
    assert!(term.contains(&symbol!["B"]));
}

/// A test that finds "fuzzy" terms (see docs for more info)
#[test]
fn find_fuzzy() {
    let a = term!["A", "B"];
    let b = term!["A", "!A"];
    let c = term!["A"];

    assert!(a.is_fuzzy() == true);
    assert!(b.is_fuzzy() == false);
    assert!(c.is_fuzzy() == false);
}

/// A test that compares two terms
#[test]
fn compare() {
    let a = term!["A", "B"];
    let b = term!["B", "C"];
    let c = term!["A", "B"];

    assert!(a == c);
    assert!(a != b);
}