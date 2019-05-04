extern crate lam_term;

mod rslam;
mod binlam;

use std::env;

use lam_term::testterm1;

fn main() {
    test_rs_kirvine();
    test_bin_kirvine();
}

fn test_rs_kirvine() {
    let tt1 = testterm1();
    let (resterm, steps) = rslam::run_krivine(&tt1);
    println!("{}", steps);
    println!("{}", resterm);
}

fn test_bin_kirvine() {
    let mut binterm: Vec<usize> = Vec::new();
    binlam::lam_term_to_bin(&testterm1(), &mut binterm);
    match binlam::run_kirvine(&binterm) {
        Ok((resterm, steps)) => {
            println!("{}", steps);
            println!("{}", binlam::bin_term_to_lam(resterm, &binterm[..]).ok().unwrap())
        },
        Err(message) => println!("{}", message)
    }
}

fn addterm(term: Vec<u8>, mut terms: Vec<u8>) -> usize {
    let term_index = terms.len();
    terms.extend(term);
    term_index
}
