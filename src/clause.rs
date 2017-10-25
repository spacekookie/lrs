//! A module that represents a logical clause and works with terms.

use terms::*;
use clause::OperationType::DERIVE;
// use clause::OperationType::JOIN;
// use clause::OperationType::DELETE;
// use clause::OperationType::BRANCH;

/// Represents a logical equation in conjunctive normal form
/// Consists of a finite collection of Terms of symbols chained via OR
/// that are chained in the clause with AND operations.
#[derive(Debug)]
pub struct Clause {
    pub terms: Vec<Term>,
}

impl PartialEq for Clause {
    fn eq(&self, other: &Clause) -> bool {
        for t in &self.terms {
            if !other.contains(t) {
                return false
            }
        }

        for t in &other.terms {
            if !self.contains(t) {
                return false
            }
        }

        return true;
    }
}


/// Describes a reduce operation type
pub enum OperationType {
    /// Eliminates single symbol terms
    DERIVE,
    /// Joins two terms, eliminating at most one symbol
    JOIN,
    /// Delete a symbol from a term for different reasons
    DELETE,
    /// Run a branching scenario
    BRANCH,
}


/// Represents a reduce operation and every piece of data involved
#[allow(dead_code)]
pub struct Operation<'a> {
    _type: OperationType,
    term: Vec<&'a Term>,
    symbol: Vec<&'a Symbol>,
}


impl Clause {

    /// Insert a term naively into an clause
    pub fn insert(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Removes a term from the clause based on it's values, not
    ///   the order the symbols appear in the term in.
    ///
    /// This is a relatively slow function (it runs on linear time) so
    ///   should be avoided whenever possible.
    pub fn remove(&mut self, term: Term) {
        if !self.contains(&term) {
            return;
        }
        
        /* Only push to new vector if value is right */
        let mut newvec: Vec<Term> = Vec::new();
        for t in &self.terms {
            if term != *t {
                newvec.push(t.clone());
            }
        }

        self.terms = newvec;
    }

    /// Run a single reduce step and return the exact reduce operation
    pub fn reduce(&mut self) -> Operation {

        /* First we search for single defined terms */
        let mut single: Term = self.search_single_terms().unwrap();
        self.reduce_single_term(&mut single);

        return Operation {
            _type: DERIVE,
            term: Vec::new(),
            symbol: Vec::new(),
        };
    }

    /// Check if a term is contained in this clause. The order of symbols
    ///   in the terms don't matter, as long as the values are the same
    ///
    /// This function is relatively slow (O(n²)) because it compares 
    ///   all symbols in a term with all symbols in the term provided
    pub fn contains(&self, term: &Term) -> bool {
        for t in &self.terms {
            if t == term {
                return true;
            }
        }

        return false;
    }

    //////////////////////////////////////////////////////////////

    /// Reduces a single term from the clause (rule 1)
    fn reduce_single_term(&mut self, term: &mut Term) {

        let mut op = Operation {
            _type: DERIVE,
            term: Vec::new(),
            symbol: Vec::new(),
        };

        op.term.push(term);
        for symbol in &term.symbols {
            op.symbol.push(symbol);
        }
        self.remove(term.clone());
    }

    /// Will return the first single-value term it encounters
    fn search_single_terms(&mut self) -> Option<Term> {
        for term in self.terms.iter() {
            if term.symbols.len() == 1 {
                return Some(term.clone());
            }
        }
        return None;
    }
}


/// A helper macro that can be used to easily create a clause from multiple
///   terms. The clause will use conjuctive-normal-form to connect terms.
///
/// clause![term![symbol!["A"]], b, c]]
#[macro_export]
macro_rules! clause {
    ( $( $x:expr ),* ) => {{
        let mut tmp: Vec<Term> = Vec::new();
        $( tmp.push($x); )*        
        Clause { terms: tmp }
    }};
}