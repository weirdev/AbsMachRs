extern crate lam_term;

use lam_term::LamTerm;
use LamTerm::*;

struct Closure<'a> {
    term: &'a LamTerm<'a>,
    environ: isize
}

pub fn run_krivine<'a>(term: &'a LamTerm) -> (&'a LamTerm<'a>, usize) {
    let mut heap: Vec<(isize, Vec<Closure>)> = Vec::new();
    let mut curenviron: isize = -1;
    let mut stack: Vec<Closure> = Vec::new();
    let mut curterm = term;
    let mut steps = 0;
    loop {
        steps += 1;
        match curterm {
            LamTerm::Application(left, right) => {
                stack.push(Closure {
                    term: right,
                    environ: curenviron
                });
                curterm = *left
            },
            LamTerm::Abstraction(varcount, body) => {
                if stack.len() < *varcount {
                    break;
                }
                heap.push((curenviron, stack.split_off(stack.len() - *varcount)));
                curenviron = (heap.len() - 1) as isize;
                curterm = body;
            },
            LamTerm::Var(v, k) => {
                if *v != 0 {
                    for _ in 0..*v {
                        if curenviron >= 0 {
                            curenviron = heap[curenviron as usize].0;
                        } else {
                            break;
                        }
                    }
                }
                if curenviron >= 0 {
                    let apps = &heap[curenviron as usize].1;
                    let clos = &apps[apps.len() - k];
                    curterm = clos.term;
                    curenviron = clos.environ;
                } else {
                    break;
                }
            }
        }
    }
    (curterm, steps)
}

