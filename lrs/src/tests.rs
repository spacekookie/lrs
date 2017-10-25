//! Some unit tests for all the internal functionality of lrs

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_macros)]
#[allow(unused_imports)]
mod tests {
    use terms::*;
    use equation::*;

    // macro_rules! symbol {
    //     ($v:expr, $s:expr) => ( Symbol { val: $v, state: $s } );
    // }

    #[test]
     fn lrs_symbol_create() {
        let s = symbol!["A"];
        assert!(s.val == 'A');
        assert!(s.state == true);
    }

    #[test]
    fn lrs_term_create() {
        let term = term!["A", "!🐎", "🐟", "C"];

        assert!(term.contains(&symbol!["A"]));
        assert!(term.contains(&symbol!["!🐎"]));
        assert!(term.contains(&symbol!["🐟"]));
        assert!(term.contains(&symbol!["C"]));
    }

    #[test]
    #[allow(unused_variables)]
    fn lrs_term_remove() {
        let mut term = term!["A", "!🐎", "🐟", "C"];

        term.remove(symbol!["A"]);
        assert!(! term.contains(&symbol!["A"]));

        term.remove(symbol!["!🐎"]);
        assert!(! term.contains(&symbol!["!🐎"]));
        
        term.remove(symbol!["🐟"]);
        assert!(! term.contains(&symbol!["🐟"]));

        term.remove(symbol!["C"]);
        assert!(! term.contains(&symbol!["C"]));
    }
}
