extern crate lam_term;

mod rslam;
mod binlam;

use std::env;

use lam_term::testterm1;

fn main() {
    //test_rs_kirvine();
    test_bin_kirvine();
}

fn test_rs_kirvine() {
    let (resterm, steps) = rslam::run_krivine(&testterm1);
    println!("{}", steps);
    println!("{}", resterm);
}

fn test_bin_kirvine() {
    let mut binterm: Vec<usize> = Vec::new();
    binlam::lam_term_to_bin(&testterm1, &mut binterm);
    match binlam::run_kirvine(&binterm) {
        Ok((resterm, steps)) => println!("{}", steps),
        Err(message) => println!("{}", message)
    }
}

fn addterm(term: Vec<u8>, mut terms: Vec<u8>) -> usize {
    let term_index = terms.len();
    terms.extend(term);
    term_index
}
