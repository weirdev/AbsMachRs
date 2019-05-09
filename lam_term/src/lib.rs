use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum LamTerm {
    Abstraction(usize, Box<LamTerm>),
    Application(Box<LamTerm>, Box<LamTerm>),
    Var(usize, usize)
}

use LamTerm::*;

impl LamTerm {
    pub fn parse(text: &str) -> LamTerm {
        fn parse_helper(mut text: &str, env: &mut HashMap<char, usize>) -> Option<LamTerm> {
            text = text.trim();
            if text.is_empty() {
                return None;
            } else {
                let mut chars = text.chars();
                let c =  chars.next().unwrap();
                if c == '(' {
                    let (termtext, remterm) = take_paren(&text[1..]);
                    let term = parse_helper(termtext, env);
                    let appterm = parse_helper(remterm, env);
                    match appterm {
                        None => return Some(term.expect("NV0")),
                        Some(appt) => return Some(Application(Box::new(term.expect("NV1")), Box::new(appt)))
                    }
                } else if c == '%' {
                    let lvar = chars.next().unwrap();
                    for val in env.values_mut() {
                        *val += 1;
                    }
                    env.insert(lvar, 0);
                    return Some(Abstraction(1, Box::new(parse_helper(&text[2..], env).expect("NV2"))));
                } else {
                    return Some(Var(*env.get(&c).expect("NV3"), 1))
                }
            }
        }

        fn take_paren(text: &str) -> (&str, &str) {
            let mut i: usize = 0;
            let mut pc = 1;
            for c in text.chars() {
                if c == '(' {
                    pc += 1;
                } else if c == ')' {
                    pc -= 1;
                }
                i += 1;
                if pc == 0 {
                    break;
                }
            }
            return (&text[..i-1], &text[i..])
        }

        parse_helper(text, &mut HashMap::new()).unwrap()
    }
}

impl<'a> fmt::Display for LamTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Abstraction(varcount, body) => write!(f, "(%{} {})", varcount, body),
            Application(left, right) => write!(f, "({} @ {})", left, right),
            Var(v, k) => write!(f, "({}, {})", v, k)
        }
    }
}

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
    