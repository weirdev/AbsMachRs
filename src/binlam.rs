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