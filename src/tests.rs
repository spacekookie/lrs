//! Some unit tests for all the internal functionality of lrs

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_macros)]
#[allow(unused_imports)]
mod tests {
    use terms::*;
    use clause::*;

    #[test]
     fn lrs_symbol_create() {
        let s = symbol!["A"];
        assert!(s.val == 'A');
        assert!(s.state == true);
    }

    #[test]
    fn lrs_term_create() {
        let term = term!["A", "!🐎", "🐟", "!🚀"];

        assert!(term.contains(&symbol!["A"]));
        assert!(term.contains(&symbol!["!🐎"]));
        assert!(term.contains(&symbol!["🐟"]));
        assert!(term.contains(&symbol!["!🚀"]));
    }

    #[test]
    fn lrs_term_remove() {
        let mut term = term!["A", "!🐎", "🐟", "!🚀"];

        term.remove(symbol!["A"]);
        assert!(! term.contains(&symbol!["A"]));

        term.remove(symbol!["!🐎"]);
        assert!(! term.contains(&symbol!["!🐎"]));
        
        term.remove(symbol!["🐟"]);
        assert!(! term.contains(&symbol!["🐟"]));

        term.remove(symbol!["!🚀"]);
        assert!(! term.contains(&symbol!["!🚀"]));
    }

    #[test]
    fn lrs_compare_terms() {
        let a = term!["A", "B"];
        let b = term!["B", "C"];
        let clause = clause![a, b];

        assert!(clause.contains(&term!["A", "B"]));
        assert!(clause.contains(&term!["B", "A"]));

        assert!(clause.contains(&term!["C", "B"]));
        assert!(! clause.contains(&term!["C", "D"]));
    }

    #[test]
    #[ignore]
    fn lrs_simple_reduce() {
        let a = term!["A", "!B"];
        let b = term!["C", "!D"];
        let c = term!["E"];

        let mut clause = clause![a, b, c];
        clause.reduce();
    }
}
