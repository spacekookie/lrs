//! A module that represents a logical clause and works with terms.

use terms::*;
use clause::OperationType::DERIVE;
// use clause::OperationType::JOIN;
// use clause::OperationType::DELETE;
// use clause::OperationType::BRANCH;

/// Represents a logical equation in conjunctive normal form
/// Consists of a finite collection of Terms of symbols chained via OR
/// that are chained in the clause with AND operations.
pub struct Clause {
    pub terms: Vec<Term>,
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

    /// Run a single reduce step and return the exact reduce operation
    pub fn reduce(&mut self) -> Operation {
        /* First we search for single defined terms */
        let single = self.search_single_terms();
        match single {
            Some(t) => {
                
            }
            None => {}
        }

        return Operation {
            _type: DERIVE,
            term: Vec::new(),
            symbol: Vec::new(),
        };
    }

    //////////////////////////////////////////////////////////////

    /// Will return the first single-value term it encounters
    fn search_single_terms(&mut self) -> Option<&Term> {
        for term in self.terms.iter() {
            if term.symbols.len() == 1 {
                return Some(term);
            }
        }
        return None;
    }
}
