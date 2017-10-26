//! Some unit tests for all the internal functionality of lrs

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_macros)]
#[allow(unused_imports)]
mod tests {
    use terms::*;
    use clause::*;
    use result::*;

    #[test]
     fn lrs_symbol_create() {
        let s = symbol!["A"];
        assert!(s.val == 'A');
        assert!(s.state == true);
    }

    #[test]
    fn lrs_term_create() {
        let term = term!["A", "!🐎", "🍝", "!📰"];

        assert!(term.contains(&symbol!["A"]));
        assert!(term.contains(&symbol!["!🐎"]));
        assert!(term.contains(&symbol!["🍝"]));
        assert!(term.contains(&symbol!["!📰"]));
    }

    #[test]
    fn lrs_term_remove() {
        let mut term = term!["A", "!🐎", "🍝", "!📰"];

        term.remove(symbol!["A"]);
        assert!(! term.contains(&symbol!["A"]));

        term.remove(symbol!["!🐎"]);
        assert!(! term.contains(&symbol!["!🐎"]));
        
        term.remove(symbol!["🍝"]);
        assert!(! term.contains(&symbol!["🍝"]));

        term.remove(symbol!["!📰"]);
        assert!(! term.contains(&symbol!["!📰"]));
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
    fn lrs_simple_reduce1() {
        let a = term!["A", "!B"];
        let b = term!["C", "!D"];
        let c = term!["E"];

        let mut cl = clause![a, b, c];
        cl.reduce();

        /* We should no longer contain "E" */
        assert!(! cl.contains(&term!["E"]));
    }

    #[test]
    fn lrs_simple_reduce2() {
        let a = term!["A", "!B"];
        let b = term!["C", "!D"];
        let c = term!["E"];
        let d = term!["F"];

        let mut cl = clause![a.clone(), b.clone(), c, d];
        cl.reduce();
        cl.reduce();

        /* Should now look like {A, !B} & {C, !D} */
        assert_eq!(cl, clause![a, b]);
    }

    #[test]
    fn lrs_result_create() {
        let r = result![true, ("A", false), ("B", true)];
        assert_eq!(r.symbols.get(&symbol!["A"]).unwrap(), &false);
        assert_eq!(r.symbols.get(&symbol!["B"]).unwrap(), &true);
        assert_eq!(r.solvable, true);
    }

    // #[test]
    // fn lrs_merge_reduce_1() {
    //     let a1 = term!["A", "!B"];
    //     let b1 = term!["A", "B"];
    //     let c1 = term!["C", "!D"];

    //     let mut cl = clause![a1, b1, c1];
    //     cl.reduce();
    //     assert_eq!(cl, clause![term!["A"], term!["C", "!D"]]);
    // }

    // #[test]
    // fn lrs_merge_reduce_2() {
    
    //     /* ...Oh fuck this is a difficult one... */
    //     let a2 = term!["A", "!B"];
    //     let b2 = term!["A", "B"];
    //     let c2 = term!["!A", "D"];
        
    //     let mut cl = clause![a2, b2, c2];
    //     cl.reduce();
    //     cl.reduce();
        
    //     /* It needs to have picked a2 and b2 first 😟 */
    //     assert_eq!(cl, clause![term!["D"]]);
    // }
}
