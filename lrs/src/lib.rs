//! This will contain some good documentation about the LRS crate
//!
//! For now I guess enjoy this emoji of a cup 🍵

mod terms;
pub use terms::*;

mod equation;
pub use equation::*;

/* Unit tests below */

macro_rules! symbol {
    ($v:expr, $s:expr) => ( Symbol { val: $v, state: $s } );
}

#[test]
fn lrs_symbol_create() {
    let s = symbol!('A', true);
    assert!(s.val == 'A');
    assert!(s.state == true);
}

#[test]
fn lrs_term_create() {
    let s = symbol!('A', true);
    let t = Term::new(s);

    assert!(t.contains(&symbol!('A', true)));
}