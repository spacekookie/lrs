//! A module that describes a logical term in a formula
//!
//! A term consists of a series of symbol values that can have
//! new values pushed into it. Insertions will make sure that the
//! term is symbol is unique in a term


/// A logical symbol in a term
pub struct Symbol {
    pub v: char,
    pub s: bool,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.v == other.v && self.s == other.s
    }
}

/// Represents a list of symbols in OR operator chain which in a LRS notation
/// would make up something like { A, B, ~C }. Insertions are sanitised to make
/// sure that there are never two contradicting symbols in the same term
pub struct Term {
    pub symbols: Vec<Symbol>,
}


impl Term {
    /// Simple linear `contains` function for a Term. Avoid using this
    ///  as it can lead to high runtimes on longer terms.
    pub fn contains(&self, sym: &Symbol) -> bool {
        for symbol in self.symbols.iter() {
            if symbol == sym {
                return true;
            }
        }
        return false;
    }

    /// Insert a new symbol at the end of the term. Returns a boolean
    ///   indicating if the insert was successful!
    pub fn insert(&mut self, sym: Symbol) -> bool {
        if self.contains(&sym) {
            return false;
        }

        self.symbols.push(sym);
        return true;
    }
}
