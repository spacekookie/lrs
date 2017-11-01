//! A small module which handles all unit testing inside LRS.
//! Structured per-component to make the testing code easier to read, 
//! understand and expand in areas that are still lacking.still
//!
//! If you find sections that need more testing or where test code is not 
//! understandable, please don't shy away from creating a new module
//! 
//! 
//! Here is also a little guide how to write new unit tests for LRS. Generally 
//! test cases should rely on as little external code as possible. A test case 
//! should create it's own data, work on it and then assert success or failure.
//! 
//! When creating a unit test for a module or datastructure, it is most
//! important to test the basic workflow of what developers can actually 
//! do with said structure, such as...
//! 
//!  - Creation
//!  - Cleanup
//!  - Basic queuries
//!  - Adding new elements
//!  - Removing existing elements
//! 
//! Any further functionality is then very specific to a structure but the same general workflow
//! philosophy should be used.
//!
//! For any further questions about testing, don't hesitate to contact me 
#![allow(unused)]
#![feature(test)]
#![cfg(test)]

mod symbols;
mod terms;
mod clauses;
mod benches;

use terms::*;
use clause::*;
use result::*;



/// A small function which tests result creation
#[test]
fn result_create() {
    let r = result![true, ("A", false), ("B", true)];
    assert_eq!(r.symbols.get(&symbol!["A"]).unwrap(), &false);
    assert_eq!(r.symbols.get(&symbol!["B"]).unwrap(), &true);
    assert_eq!(r.solvable, true);

    println!("{:?}", r);
}