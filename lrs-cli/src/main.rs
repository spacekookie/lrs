pub struct Symbol {
    v: char,
    s: bool
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.v == other.v && self.s == other.s
    }
}


fn main() {
    let a = Symbol { v: 'A', s: true };
    let nb = Symbol { v: 'B', s: false };
    
    println!("Compare: {:?}", compare(a, nb));
}

fn compare(a: Symbol, b: Symbol) -> bool {
    return a == b;
}