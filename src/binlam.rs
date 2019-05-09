extern crate lam_term;

use lam_term::LamTerm;
use LamTerm::*;

pub fn lam_term_to_bin(term: &LamTerm, buffer: &mut Vec<usize>) -> (usize, usize) {
    match term {
        Abstraction(varcount, body) => {
            let start = buffer.len();
            buffer.push(2); // Abstraction instruction code
            buffer.push(*varcount);
            let pos = buffer.len();
            buffer.push(0);
            buffer.push(0);
            let (startbody, endbody) = lam_term_to_bin(body, buffer);
            buffer[pos] = startbody;
            buffer[pos+1] = endbody;
            return (start, pos+2)
        },
        Application(left, right) => {
            let start = buffer.len();
            buffer.push(1); // Application instruction code
            let pos = buffer.len();
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            let (lstart, lend) = lam_term_to_bin(left, buffer);
            buffer[pos] = lstart;
            buffer[pos + 1] = lend;
            let (rstart, rend) = lam_term_to_bin(right, buffer);
            buffer[pos + 2] = rstart;
            buffer[pos + 3] = rend;
            return (start, pos + 4)
        },
        Var(v, k) => {
            let start = buffer.len();
            buffer.push(0); // Var instruction code
            buffer.push(*v);
            buffer.push(*k);
            return (start, start+3)
        }
    }
}

/*
pub fn bin_term_to_lam<'a>(term: &[usize], termbuffer: &Vec<usize>) -> Result<LamTerm<'a>, &'a str> {
    match term[0] {
        1 => {
            let left: Result<LamTerm<'a>, &'a str> = bin_term_to_lam(&termbuffer[term[1]..term[2]], termbuffer);
            let right: Result<LamTerm<'a>, &'a str> = bin_term_to_lam(&termbuffer[term[3]..term[4]], termbuffer);
            Ok(Application(&left?, &right?))
        },
        2 => {
            let varcount = term[1];
            let body = bin_term_to_lam(&termbuffer[term[2]..term[3]], termbuffer);
            Ok(Abstraction(varcount, &body?))
        },
        0 => Ok(Var(term[1], term[2])),
        _ => Err("Error translating binary term")
    }
}*/

pub fn bin_term_to_lam<'a>(term: &[usize], termbuffer: &[usize]) -> Result<Box<LamTerm>, &'a str> {
    match term[0] {
        1 => {
            let left = bin_term_to_lam(&termbuffer[term[1]..term[2]], termbuffer)?;
            let right = bin_term_to_lam(&termbuffer[term[3]..term[4]], termbuffer)?;
            Ok(Box::new(Application(left, right)))
        },
        2 => {
            let varcount = term[1];
            let body = bin_term_to_lam(&termbuffer[term[2]..term[3]], termbuffer)?;
            Ok(Box::new(Abstraction(varcount, body)))
        },
        0 => Ok(Box::new(Var(term[1], term[2]))),
        _ => Err("Error translating binary term")
    }
}

pub fn run_kirvine<'a>(term: &'a Vec<usize>) -> Result<(&'a [usize], usize), &'a str> {
    let mut heap: Vec<(isize, Vec<(&[usize], isize)>)> = Vec::new();
    let mut curenviron: isize = -1;
    let mut stack: Vec<(&[usize], isize)> = Vec::new();
    let mut curterm = &term[..];
    let mut steps = 0;
    loop {
        steps += 1;
        match curterm[0] {
            1 => {
                let right = &term[curterm[3]..curterm[4]];
                stack.push((right, curenviron));
                curterm = &term[curterm[1]..curterm[2]];
            },
            2 => {
                let varcount = curterm[1];
                if stack.len() < varcount {
                    return Ok((curterm, steps));
                }
                heap.push((curenviron, stack.split_off(stack.len() - varcount)));
                curenviron = (heap.len() - 1) as isize;
                curterm = &term[curterm[2]..curterm[3]];
            },
            0 => {
                let v = curterm[1];
                let k = curterm[2];
                if v != 0 {
                    for _ in 0..v {
                        if curenviron >= 0 {
                            curenviron = heap[curenviron as usize].0;
                        } else {
                            return Ok((curterm, steps));
                        }
                    }
                }
                if curenviron >= 0 {
                    let apps = &heap[curenviron as usize].1;
                    let clos = &apps[apps.len() - k];
                    curterm = clos.0;
                    curenviron = clos.1;
                } else {
                    return Ok((curterm, steps));
                }
            },
            _ => return Err("Error Computing Term")
        }
    }
}

pub fn run_secd<'a>(term: &'a Vec<usize>) -> () {

}

fn secd_compute<'a>(term: &'a Vec<usize>) -> Result<((usize, Vec<(isize, usize)>, Vec<(usize, isize)>), usize), &'a str> {
    let mut stack: Vec<usize> = Vec::new();
    let mut vals: Vec<(usize, isize)> = Vec::new();
    let mut heap: Vec<(isize, usize)> = Vec::new();
    let mut curenviron: isize = -1;
    let mut curtermlocs: Vec<isize> = termseq(0, term);
    let mut dump: Vec<(Vec<usize>, isize, Vec<isize>)> = Vec::new();
    let mut steps = 0;

    loop {
        steps += 1;
        if curtermlocs.len() == 0 {
            let res = stack.pop().unwrap();
            if dump.len() == 0 {
                return Ok(((res, heap, vals), steps));
            }
            let sec = dump.pop().unwrap();
            stack = sec.0;
            curenviron = sec.1;
            curtermlocs = sec.2;
            stack.push(res);
            continue;
        }
        let toptermloc = curtermlocs.pop().unwrap();
        match toptermloc {
            // Application
            -1 => {
                let left = vals[stack.pop().unwrap()];
                let rightpos = stack.pop().unwrap();
                curenviron = left.1;
                heap.push((curenviron, rightpos));
                curenviron = (heap.len() as isize) - 1;
                match term[left.0] {
                    // Abstraction
                    2 => {
                        match term[term[left.0 + 2]] {
                            // Application
                            1 => {
                                dump.push((stack, curenviron, curtermlocs));
                                stack = Vec::new();
                                curtermlocs = termseq(term[left.0 + 2], term);
                            },
                            // Abstraction
                            2 => {
                                vals.push((term[left.0 + 2], curenviron));
                                stack.push(vals.len() - 1);
                            },
                            // Var
                            0 => curtermlocs.push(term[left.0 + 2] as isize),
                            _ => return Err("Error computing term")
                        }
                    },
                    _ => return Err("Only closed terms allowed")
                }
            },
            _ => match term[toptermloc as usize] {
                // Abstraction
                2 => {
                    vals.push((toptermloc as usize, curenviron));
                    stack.push(vals.len() - 1);
                },
                // Var
                0 => {
                    for _ in 0..term[(toptermloc as usize) + 1] {
                        curenviron = heap[curenviron as usize].0;
                    }
                    stack.push(heap[curenviron as usize].1);
                }
                _ => return Err("Abstractions do not exist")
            }
        }
    }
}

fn termseq(termstart: usize, termbuffer: &[usize]) -> Vec<isize> {
    let mut seq: Vec<isize> = Vec::new();
    match termbuffer[termstart] {
        // Application
        1 => {
            seq.push(-1);
            seq.extend(termseq(termbuffer[termstart + 1], termbuffer));
            seq.extend(termseq(termbuffer[termstart + 3], termbuffer));
        },
        _ => seq.push(termstart as isize)
    }
    seq
}