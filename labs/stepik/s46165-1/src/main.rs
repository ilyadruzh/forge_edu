use std::clone::Clone;
use std::env;
use std::io::{self, BufRead, Read, Write};

fn main() {
    let stdin = io::stdin();
    // find_fib_num_from_terminal(stdin); // Поиск n-ого числа Фибоначчи
    // find_last_digit_in_fib_num(stdin); // Поиск последней цифры в n-ом числе Фибоначчи
    // find_n_mod_m(stdin); найти остаток от деления n-го числа Фибоначи на m-ное число Фибоначи
}

fn find_fib_num_from_terminal(stdin: std::io::Stdin) {
    for line in stdin.lock().lines() {
        let fib_result = fib(line.unwrap().parse::<u32>().unwrap());
        println!("{}", fib_result);
    }
}

fn find_last_digit_in_fib_num(stdin: std::io::Stdin) {
    for line in stdin.lock().lines() {
        let src_num = line.unwrap().parse::<u32>().unwrap();

        let mut a = 0;
        let mut b = 1;
        let mut res = 0;

        match src_num {
            0 => src_num,
            1 => src_num,
            _ => {
                for x in 2..src_num + 1 {
                    res = (a + b) % 10;
                    a = b;
                    b = res;
                }
                res
            }
        };
        println!("result = {}", res % 10);
    }
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

fn find_n_mod_m(stdin: std::io::Stdin) {
    for line in stdin.lock().lines() {
        println!("line = {:?}", line);
    }
}
