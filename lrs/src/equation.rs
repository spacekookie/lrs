//! A module that represents equations and works with terms

use terms::*;
use equation::OperationType::DERIVE;
use equation::OperationType::JOIN;
use equation::OperationType::DELETE;
use equation::OperationType::BRANCH;

/// Represents an equation in LRS standard form where each
///  term is chained in an AND operation
pub struct Equation {
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
pub struct Operation<'a> {
    _type: OperationType,
    term: Vec<&'a Term>,
    symbol: Vec<&'a Symbol>,
}


impl Equation {
    /// Insert a term naively into an equation
    pub fn insert(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Run a single reduce step and return the exact reduce operation
    pub fn reduce(&mut self) -> Operation {
        /* First we search for single defined terms */
        let single = self.search_single_terms();
        match single {
            Some(term) => {
                println!("Fooo");
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
