//! A module that describes a logical term in a formula
//!
//! A term consists of a series of symbol values that can have
//! new values pushed into it. Insertions will make sure that the
//! term is symbol is unique in a term


/// A logical symbol in a term
#[derive(Copy, Clone, Debug, Hash, Eq)]
pub struct Symbol {
    pub val: char,
    pub state: bool,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.val == other.val && self.state == other.state
    }
}

/// Represents a list of symbols in OR operator chain which in a LRS notation
///   would make up something like { A, B, ¬C }. Insertions are sanitised to
///   make sure that there are never two contradicting symbols in the same term.
#[derive(Clone, Debug)]
pub struct Term {
    pub symbols: Vec<Symbol>,
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        for s in &self.symbols {
            if !other.contains(s) {
                return false;
            }
        }

        for s in &other.symbols {
            if !self.contains(s) {
                return false;
            }
        }

        // return self.symbols == other.symbols;
        return true;
    }
}


impl Term {

    /// Simple linear `contains` function for a Term. Avoid using this
    ///   as it can lead to high runtimes on longer terms.
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

    /// Removes a symbol from this term if it exists in linear time
    pub fn remove(&mut self, sym: Symbol) -> bool {
        if !self.contains(&sym) {
            return false;
        }

        /* Only push to new vector if value is right */
        let mut newvec: Vec<Symbol> = Vec::new();
        for s in &self.symbols {
            if sym != *s {
                newvec.push(*s);
            }
        }

        /* Then use the new vector instead */
        self.symbols = newvec;
        return true;
    }

    /// Will find all symbols in a term that offend a logical state in that term
    /// In other words they will find the boolean opposite of said term. This means
    /// that if a term consists of { A, ¬A, B } this function will return 
    /// both A and ¬A but not B because it's unrelated to the "offence"
    pub fn find_offending(&self) -> Vec<Symbol> {
        let mut v = Vec::new();

        for s in &self.symbols {
            if self.contains(&Symbol { val: s.val, state: s.state }) {
                v.push(s.clone());
            }
        }

        return v;
    }

    /// This function will determine if this is a fuzzy term
    ///
    /// What that means is that a term has a series of symbols that with themselves
    /// aren't in conflict but at the same time don't conclude a well-defined
    /// state. Each symbol in the term has multiple "fuzzy" interpretations that
    /// still fullfil the entire term (or clause) without being a fixed result. 
    pub fn is_fuzzy(&self) -> bool {
        if self.symbols.len() == 1 {
            return false;
        }

        for s in &self.symbols {
            /* Find a symbol of same value, opposite state */
            if self.contains(&Symbol { val: s.val, state: !s.state }) {
                return false;
            }
        }
        return true;
    }

    /// Merge two terms together in a very naive way. This function will only check
    /// for same duplicates (to avoid { A, A, B } situations). This will however allow
    /// { A, ¬A } scenarios. Those need to be removed manually
    pub fn merge(&mut self, other: Term) {
        let ovec: Vec<Symbol> = other.symbols;
        for sym in ovec.iter() {
            self.insert(sym.clone());
        }
    }
}


/// A helper macro that creates terms with a variable amount of symbols
///   based on the more user friendly syntax
///
/// term!["A", "!B", "C"]
///
/// Please note that your symbols aren't allowed to be multi-character
/// but can include all utf-8 symbols (including emoji 🔥🎉)
#[macro_export]
macro_rules! term {
    ( $( $x:expr ),* ) => {{
        let mut ts: Vec<Symbol> = Vec::new();
        $( ts.push(symbol!($x)); )*        
        Term { symbols: ts }
    }};
}


/// A very simple helper macro which takes a string and turns it into a symbol
#[macro_export]
macro_rules! symbol {
    ($x:expr) => {{
        if $x.starts_with('!') {
            let ch = $x.chars().nth(1).unwrap();
            Symbol { val: ch, state: false }
        } else {
            let ch = $x.chars().nth(0).unwrap();
            Symbol { val: ch, state: true }
        }
    }};
}
