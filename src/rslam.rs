extern crate lam_term;

use std::borrow::Borrow;

use lam_term::LamTerm;
use LamTerm::*;

struct Closure<'a> {
    term: &'a Box<LamTerm>,
    environ: isize
}

enum ControlElement<'a> {
    Term(&'a Box<LamTerm>),
    App
}
use ControlElement::*;

pub fn run_krivine<'a>(term: &'a Box<LamTerm>) -> (Box<LamTerm>, usize) {
    let (clos, heap, steps) = krivine_compute(term);
    (krivine_closure_to_closed_term(&clos, &heap, 0), steps)
}

fn krivine_compute<'a>(term: &'a Box<LamTerm>) -> (Closure, Vec<(isize, Vec<Closure<'a>>)>, usize) {
    let mut heap: Vec<(isize, Vec<Closure<'a>>)> = Vec::new();
    let mut curenviron: isize = -1;
    let mut stack: Vec<Closure<'a>> = Vec::new();
    let mut curterm = term;
    let mut steps = 0;
    loop {
        println!("{}", curterm);
        steps += 1;
        match (*curterm).borrow() {
            Application(ref left, ref right) => {
                stack.push(Closure {
                    term: &right,
                    environ: curenviron
                });
                curterm = &left
            },
            Abstraction(varcount, body) => {
                if stack.len() < *varcount {
                    break;
                }
                heap.push((curenviron, stack.split_off(stack.len() - varcount)));
                curenviron = (heap.len() - 1) as isize;
                curterm = &body;
            },
            Var(v, k) => {
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
    (Closure {term: curterm, environ: curenviron}, heap, steps)
}

fn krivine_closure_to_closed_term<'a>(clos: &Closure<'a>, heap: &Vec<(isize, Vec<Closure<'a>>)>, level: usize) -> Box<LamTerm> {
    match (*clos.term).borrow() {
        Abstraction(n, subterm) => Box::new(Abstraction(*n, krivine_closure_to_closed_term(&Closure {
                term: subterm,
                environ: clos.environ
            }, heap, level + 1))),
        Application(st1, st2) => Box::new(Application(
            krivine_closure_to_closed_term(&Closure {
                term: st1,
                environ: clos.environ
            }, heap, level),
            krivine_closure_to_closed_term(&Closure {
                term: st2,
                environ: clos.environ
            }, heap, level)
        )),
        Var(v,k) => {
            if *v >= level {
                let mut env = clos.environ;
                for _ in level..*v {
                    env = heap[env as usize].0;
                }
                return krivine_closure_to_closed_term(&heap[env as usize].1[k-1], heap, 0);
            } else {
                return Box::new(Var(*v, *k));
            }
        }
    }
}

pub fn run_secd(term: &Box<LamTerm>) -> (Box<LamTerm>, usize) {
    let (mut result, steps) = secd_compute(term);
    (secd_closure_to_closed_term(result.0, &result.1, &mut result.2, 0), steps)
}

fn secd_compute<'a>(term: &'a Box<LamTerm>) -> ((usize, Vec<(isize, usize)>, Vec<Closure>), usize) {
    let mut stack: Vec<usize> = Vec::new();
    let mut vals: Vec<Closure> = Vec::new();
    let mut heap: Vec<(isize, usize)> = Vec::new();
    let mut curenviron: isize = -1;
    let mut curterm = termseq(term);
    let mut dump: Vec<(Vec<usize>, isize, Vec<ControlElement>)> = Vec::new();
    let mut steps = 0;

    loop {
        steps += 1;
        if curterm.len() == 0 {
            let res = stack.pop().unwrap();
            if dump.len() == 0 {
                return ((res, heap, vals), steps)
            }
            let sec = dump.pop().unwrap();
            stack = sec.0;
            curenviron = sec.1;
            curterm = sec.2;
            stack.push(res);
            continue;
        }
        let topterm = curterm.pop().unwrap();
        match topterm {
            App => {
                let left = &vals[stack.pop().unwrap()];
                let rightpos = stack.pop().unwrap();
                curenviron = left.environ;
                heap.push((curenviron, rightpos));
                curenviron = (heap.len() as isize) - 1;
                match (*left.term).borrow() {
                    Abstraction(_, body) => {
                        match (*body).borrow() {
                            Application(..) => {
                                dump.push((stack, curenviron, curterm));
                                stack = Vec::new();
                                curterm = termseq(&body);
                            },
                            Abstraction(..) => {
                                vals.push(Closure {
                                    term: &body,
                                    environ: curenviron});
                                stack.push(vals.len()-1);
                            },
                            Var(..) => curterm.push(Term(&body))
                        }
                    },
                    _ => panic!("Only closed terms allowed")
                }
            },
            Term(value) => {
                match (*value).borrow() {
                    Abstraction(..) => {
                        vals.push(Closure {
                            term: &value,
                            environ: curenviron});
                        stack.push(vals.len()-1);
                    },
                    Var(v, _) => {
                        for _ in 0..*v {
                            curenviron = heap[curenviron as usize].0;
                        }
                        stack.push(heap[curenviron as usize].1);
                    },
                    _ => panic!("Abstractions do not exist in term seq")
                }
            }
        }
    }
}

fn secd_closure_to_closed_term<'a>(clos: usize, heap: &Vec<(isize, usize)>, vals: &mut Vec<Closure<'a>>, level: usize) -> Box<LamTerm> {
    match (*vals[clos].term).borrow() {
        Abstraction(n, subterm) => {
            let recclos = Closure {
                term: subterm,
                environ: vals[clos].environ
            };
            vals.push(recclos);
            Box::new(Abstraction(*n, secd_closure_to_closed_term(vals.len()-1, heap, vals, level + 1)))
        },
        Application(st1, st2) => {
            let recclos1 = Closure {
                term: st1,
                environ: vals[clos].environ
            };
            let recclos2 = Closure {
                    term: st2,
                    environ: vals[clos].environ
            };
            let pos = vals.len();
            vals.push(recclos1);
            vals.push(recclos2);
            Box::new(Application(
                secd_closure_to_closed_term(pos, heap, vals, level),
                secd_closure_to_closed_term(pos+1, heap, vals, level)))
            },
        Var(v,k) => {
            if *v >= level {
                let mut env = vals[clos].environ;
                for _ in level..*v {
                    env = heap[env as usize].0;
                }
                return secd_closure_to_closed_term(heap[env as usize].1, heap, vals, 0);
            } else {
                return Box::new(Var(*v, *k));
            }
        }
    }
}

fn termseq(term: &Box<LamTerm>) -> Vec<ControlElement> {
    let mut seq: Vec<ControlElement> = Vec::new();
    match (*term).borrow() {
        Application(left, right) => {
            seq.push(App);
            seq.extend(termseq(&left));
            seq.extend(termseq(&right));
        },
        _ => seq.push(Term(term))
    }
    seq
}