#[macro_use] extern crate lrs;
use lrs::*;


fn main() {
    let mut term = Term::new(Symbol { val: 'A', state: false });
    term.insert(Symbol { val: 'B', state: true });
    term.insert(Symbol { val: 'C', state: false });

    // No nice way to print yet :(
    // println!("{:?}", term);

    for s in term.symbols.iter() {
        let n = match s.state {
            true => "",
            false => "~"
        };
        println!("{}{}", n, s.val);
    }

    term!["A"];
}
