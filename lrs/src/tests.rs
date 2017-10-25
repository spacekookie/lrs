//! Some unit tests for all the internal functionality of lrs

#[cfg(test)]
mod tests {
    use terms::*;
    use equation::*;

    macro_rules! symbol {
        ($v:expr, $s:expr) => ( Symbol { val: $v, state: $s } );
    }

    #[test]
     fn lrs_symbol_create() {
        let s = symbol!('A', true);
        assert!(s.val == 'A');
        assert!(s.state == true);
    }

    #[test]
    fn lrs_term_create() {
        let s = symbol!('A', true);
        let t = Term::new(s);

        assert!(t.contains(&symbol!('A', true)));
    }
}
