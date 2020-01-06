use std::io::{self, BufRead};
use std::io::prelude::*;

fn main() {

    let reader = io::stdin();

    let numbers: Vec<i32> = reader.lock()
        .lines() // "0 2"
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let res: i32 = numbers.iter().sum();

    println!("{}", res);
}