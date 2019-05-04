use std::fmt;

#[derive(Debug)]
pub enum LamTerm {
    Abstraction(usize, Box<LamTerm>),
    Application(Box<LamTerm>, Box<LamTerm>),
    Var(usize, usize)
}

use LamTerm::*;
/*
impl<'a> fmt::Display for LamTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Abstraction(varcount, body) => write!(f, "(%{} {})", varcount, body),
            Application(left, right) => write!(f, "({} @ {})", left, right),
            Var(v, k) => write!(f, "({}, {})", v, k)
        }
    }
}
*/
pub fn testterm1() -> LamTerm {
    Application(
        Box::new(Abstraction(1, Box::new(Application(
            Box::new(Abstraction(1, Box::new(Application(
                Box::new(Var(1, 1)), Box::new(Var(0,1)))))), 
            Box::new(Var(0,1)))))), 
        Box::new(Abstraction(1, Box::new(Var(0,1)))))
}
    // (%x ((%y ((x) y)) x)) %z z"
    // (@, 
    //  (mlam, [(0, 1)], (@, 
    //      (mlam, [(0, 1)], (@, 
    //          (1, 1), (0, 1))), 
    //      (0, 1))), 
    //  (mlam, [(0, 1)], (0, 1)))
    