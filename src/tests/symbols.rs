//! A module that handles unit testing for symbols
#[allow(unused)]

use terms::*;

/// A simple function testing basic creation of a Symbol
#[test]
fn create() {
    let s = Symbol { val: 'A', state: true };
    assert!(s.val == 'A');
    assert!(s.state == true);
}

/// A simple function that creates a Symbol via the utility macro
#[test]
fn create_macro() {
    let s = symbol!["A"];
    assert!(s.val == 'A');
    assert!(s.state == true);
}

/// A simple function which tests that "==" asserts correctly for Symbol structs
#[test]
fn compare() {
    let s1 = symbol!["A"];
    let s2 = symbol!["A"];
    let s3 = symbol!["B"];

    assert!(s1 == s2);
    assert!(s1 != s3);
}