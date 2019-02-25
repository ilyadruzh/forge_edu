use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let res: i32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

     Fibonacci Recursive
    println!("fibonacci_recursive: {}", fibonacci_recursive(res));

    println!("fiboncacci_iteration: {}", fiboncacci_iteration(res));

    println!("last_digit_of_fib_num: {}", last_digit_of_fib_num(res));
}


fn fibonacci_recursive(x: i32) -> i32 {
    match x {
        0 => { 0 }
        1 => { 1 }
        _ => { fibonacci_recursive(x - 1) + fibonacci_recursive(x - 2) }
    }
}

fn fiboncacci_iteration(x: i32) -> i32 {
    let mut n1: i32 = 0;
    let mut n2: i32 = 1;
    let mut res: i32 = 1;

    match x {
        0 => 1,
        1 => 1,
        _ => {
            for i in (0..x - 1).rev() {
                res = n1 + n2;
                n1 = n2;
                n2 = res;
            }
            res
        }
    }
}

fn fibonacci_table(num: i32) -> i32 {
0
}

fn fibonacci_memoized(x: i32) {}

fn last_digit_of_fib_num(x: i32) -> i32 {
    ((fiboncacci_iteration(x)%10) + (fiboncacci_iteration(x+1)%10))%10
}