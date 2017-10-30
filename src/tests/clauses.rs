//! A module that tests everything to dow with clauses 
#![allow(unused)]

use terms::*;
use clause::*;

#[test]
fn create() {
    let c1 = clause![term!["A"]];
    let c2 = clause![term!["A"]];
    assert_eq!(c1, c2);
}


#[test]
fn compare() {
    let c1 = clause![term!["A"]];
    let c2 = clause![term!["A"]];
    assert_eq!(c1, c2);

    let c3 = clause![term!["A", "B"], term!["!B"]];
    let c4 = clause![term!["B", "A"], term!["!B"]];
    assert_eq!(c3, c4);
    
    let c5 = clause![term!["A", "B"], term!["B"]];
    let c6 = clause![term!["A"], term!["!B"]];
    assert!(c5 != c6);
}

#[test]
fn reduce_1() {
    let a = term!["A", "!B"];
    let b = term!["C", "!D"];
    let c = term!["E"];

    let mut cl = clause![a, b, c];
    cl.reduce();

    /* We should no longer contain "E" */
    assert!(! cl.contains(&term!["E"]));
}

#[test]
fn reduce_2() {
    let a = term!["A", "!B"];
    let b = term!["C", "!D"];
    let c = term!["E"];
    let d = term!["F"];

    let mut cl = clause![a.clone(), b.clone(), c, d];
    cl.reduce();
    cl.reduce();

    /* Should now look like {A, !B} & {C, !D} */
    assert_eq!(cl, clause![a, b]);
}


#[test]
fn count_symbols() {
    let cl = clause![   term!["A", "B"], 
                        term!["A", "!C"], 
                        term!["C"], 
                        term!["B"],
                        term!["A"]
                    ];

    let count = cl.count_symbols();
    let a: i32 = *count.get(&symbol!["A"]).unwrap();
    let b: i32 = *count.get(&symbol!["B"]).unwrap();
    let c: i32 = *count.get(&symbol!["C"]).unwrap();
    let nc: i32 = *count.get(&symbol!["!C"]).unwrap();

    assert_eq!(a, 3);
    assert_eq!(b, 2);
    assert_eq!(c, 1);
    assert_eq!(nc, 1);
}