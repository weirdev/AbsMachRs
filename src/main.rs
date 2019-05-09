extern crate lam_term;

mod rslam;
mod binlam;

use std::env;

use lam_term::{testterm1, LamTerm};

fn main() {
    let term = pars_arg().unwrap();
    test_rs_kirvine(&term);
    test_rs_secd(&term);
    test_bin_kirvine(&term);
}

fn test_rs_kirvine(term: &Box<LamTerm>) {
    let (resterm, steps) = rslam::run_krivine(term);
    println!("{}", steps);
    println!("{}", resterm);
}

fn test_bin_kirvine(term: &Box<LamTerm>) {
    let mut binterm: Vec<usize> = Vec::new();
    binlam::lam_term_to_bin(term, &mut binterm);
    match binlam::run_kirvine(&binterm) {
        Ok((resterm, steps)) => {
            println!("{}", steps);
            println!("{}", binlam::bin_term_to_lam(&resterm, &resterm).ok().unwrap())
        },
        Err(message) => println!("{}", message)
    }
}

fn test_rs_secd(term: &Box<LamTerm>) {
    let (resterm, steps) = rslam::run_secd(term);
    println!("{}", steps);
    println!("{}", resterm);
}

fn addterm(term: Vec<u8>, mut terms: Vec<u8>) -> usize {
    let term_index = terms.len();
    terms.extend(term);
    term_index
}

fn pars_arg<'a>() -> Result<Box<LamTerm>, &'a str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Missing required arg");
    }
    let lambda_string = &args[1];
    Ok(Box::new(LamTerm::parse(lambda_string)))
}