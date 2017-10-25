//! A module that describes a logical term in a formula
//!
//! A term consists of a series of symbol values that can have
//! new values pushed into it. Insertions will make sure that the
//! term is symbol is unique in a term


/// A logical symbol in a term
#[derive(Copy, Clone, Debug)]
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
    /// Very simple constructor that initialises a term with one symbol
    pub fn new(symbol: Symbol) -> Term {
        let mut t = Term {
            symbols: Vec::new(),
        };

        t.symbols.push(symbol);
        return t;
    }

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


/// A slightly smaller helper macro that creates a symbol based on a string
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
