use std::io::{self, BufRead};

fn main() {

    let reader = io::stdin();

    let numbers: Vec<i32> = reader.lock()
        .lines() // "0 2"
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
//        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    for x in numbers {
        println!("x: {}", x);
    }

//    println!("{:?}", numbers);

}