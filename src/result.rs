//! A module that is used to build resolution results as well as resolution history.

use terms::Symbol;
use std::collections::HashMap;


/// A struct which keeps track of what symbols could be resolved
///   to value states as well as general solvability of a clause
#[derive(Debug)]
pub struct LResult {
    pub symbols: HashMap<Symbol, bool>,
    pub solvable: bool,
}


/// A helper macro that creates a new result struct with a series of symbol, state tuples
#[macro_export]
macro_rules! result {
    ( $s:expr, $( $x:expr ),* ) => {{
        use std::collections::HashMap;
        let mut map: HashMap<Symbol, bool> = HashMap::new();
        $(
            let sym: Symbol = symbol![$x.0]; 
            let boo: bool = $x.1;
            map.insert(sym, boo);
        )*

        LResult {
            symbols: map,
            solvable: $s,
        }
    }};
}
