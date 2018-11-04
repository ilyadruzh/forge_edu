// Дано число 1≤n≤107, необходимо найти последнюю цифру n-го числа Фибоначчи.

// В данной задаче, впрочем, этой проблемы можно избежать,
// поскольку нас интересует только последняя цифра числа Фибоначчи: если 0≤a,b≤9
// — последние цифры чисел Fi и Fi+1 соответственно, то (a+b)mod10 — последняя цифра числа Fi+2.

use std::env;
use std::io::{self, BufRead, Read, Write};

fn main() {
    let stdin = io::stdin();
    // find_fib_num_from_terminal(stdin);
    // find_last_digit_in_fib_num(stdin);
    // let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", &args);
    // fib(args[1].parse::<u32>().unwrap());

    let x: u32 = 942975;

    println!("fib: {}", fib_n(x)%10);
}

fn u32_to_array_u8(x: i32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

fn fib(x: u32) -> u32 {
    match x {
        0 => return 0,
        1 => return 1,
        _ => fib(x - 1) + fib(x - 2),
    }
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn find_fib_num_from_terminal(stdin: std::io::Stdin) {
    for line in stdin.lock().lines() {
        let fib_result = fib(line.unwrap().parse::<u32>().unwrap());
        println!("{}", fib_result);
    }
}

fn find_last_digit_in_fib_num(stdin: std::io::Stdin) {
    for line in stdin.lock().lines() {
        let fib_result = line.unwrap().parse::<u32>().unwrap() % 10;
        println!("fib: {}", fib_result);
    }
}

//возвращает n-е число Фибоначчи
fn fib_n(n: u32) -> i32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => (fib_n(n - 1) %10) + (fib_n(n - 2)%10),
    }
}
