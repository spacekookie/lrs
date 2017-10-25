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
    fn lrs_compare_clauses() {
        let c1 = clause![term!["A"]];
        let c2 = clause![term!["A"]];
        assert_eq!(c1, c2);

        let c3 = clause![term!["A", "B"], term!["!B"]];
        let c4 = clause![term!["B", "A"], term!["!B"]];
        assert_eq!(c3, c4);
        
        let c5 = clause![term!["A", "B"], term!["B"]];
        let c6 = clause![term!["A"], term!["!B"]];
        assert!(c5 != c6);
    }

    #[test]
    fn lrs_simple_reduce() {
        let a1 = term!["A", "!B"];
        let b1 = term!["C", "!D"];
        let c1 = term!["E"];

        let mut clause = clause![a1, b1, c1];
        clause.reduce();

        /* We should no longer contain "E" */
        assert!(! clause.contains(&term!["E"]));

        ///// More complex example
        let a2 = term!["A", "!B"];
        let b2 = term!["C", "!D"];
        let c2 = term!["E"];
        let d = term!["F"];

        let mut cl2 = clause![a2.clone(), b2.clone(), c2, d];
        cl2.reduce();
        cl2.reduce();

        /* Should now look like {A, !B} & {C, !D} */
        assert_eq!(cl2, clause![a2, b2]);
    }

    #[test]
    fn lrs_merge_reduce() {
        let a1 = term!["A", "!B"];
        let b1 = term!["A", "B"];
        let c1 = term!["C", "!D"];

        let mut cl1 = clause![a1, b1, c1];
        cl1.reduce();
        assert_eq!(cl1, clause![term!["A"], term!["C", "!D"]]);

        /* ...Oh fuck this is a difficult one... */
        let a2 = term!["A", "!B"];
        let b2 = term!["A", "B"];
        let c2 = term!["!A", "D"];
        
        let mut cl2 = clause![a2, b2, c2];
        cl2.reduce();
        cl2.reduce();
        
        /* It needs to have picked a2 and b2 first 😟 */
        assert_eq!(cl2, clause![term!["D"]]);
    }
}
