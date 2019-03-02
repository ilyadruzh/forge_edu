extern crate num;

use std::io;
use std::io::prelude::*;
use std::ops::Div;
use std::ops::Rem;


use num::bigint::{BigUint, ToBigUint};
use num::Integer;

fn main() {
    let stdin = io::stdin();
    let res: Vec<u64> = stdin.lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    // TODO: Даны целые числа 1≤𝑛≤10^18 и 2≤𝑚≤10^5, необходимо найти остаток от деления 𝑛-го числа Фибоначчи на 𝑚.

    println!("{}", fibonacci_matrix(res[0]).rem(res[1]));

}

fn fibonacci_recursive(x: u64) -> u64 {
    match x {
        0 => { 0 }
        1 => { 1 }
        _ => { fibonacci_recursive(x - 1) + fibonacci_recursive(x - 2) }
    }
}

fn fiboncacci_iteration(x: u64) -> u64 {
    let mut n1: u64 = 0;
    let mut n2: u64 = 1;
    let mut res: u64 = 1;

    match x {
        0 => 1,
        1 => 1,
        _ => {
            for i in (0..x - 1).rev() {
                res = n1 + n2;
                n1 = n2;
                n2 = res % 10;
            }
            res
        }
    }
}

fn fibonacci_matrix(num: u64) -> u64 {
    if num == 0 { return 0; }
    if num == 1 { return 1; }

    let matrix = [[1, 1], [1, 0], ];
    let mut result = [[1, 0], [0, 1], ];

    for x in 0..num {
        result = [
            [result[0][0] * matrix[0][0] + result[1][0] * matrix[0][1], result[0][0] * matrix[1][0] + result[1][0] * matrix[1][1]],
            [result[0][1] * matrix[0][0] + result[1][1] * matrix[0][1], result[0][1] * matrix[1][0] + result[1][1] * matrix[1][1]],
        ]
    }

    result[0][0]
}

fn fibonacci_egyptian() {}

fn fibonacci_memoized(x: u64) {}

fn fib2(n: i32) -> BigUint {
    if n == 0 { return 0.to_biguint().unwrap(); }

    let mut a = 0.to_biguint().unwrap();
    let mut b = 1.to_biguint().unwrap();

    for _ in 1..n {
        let c = &a + &b;
        a = b;
        b = c;
    }

    b
}

fn mul2x2(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
    [
        [&a[0][0] * &b[0][0] + &a[1][0] * &b[0][1], &a[0][0] * &b[1][0] + &a[1][0] * &b[1][1]],
        [&a[0][1] * &b[0][0] + &a[1][1] * &b[0][1], &a[0][1] * &b[1][0] + &a[1][1] * &b[1][1]],
    ]
}

fn op_n_times<T, Op>(a: T, op: &Op, n: u64) -> T
    where Op: Fn(&T, &T) -> T {
    if n == 1 { return a; }

    let mut result = op_n_times::<T, Op>(op(&a, &a), &op, n >> 1);
    if n & 1 == 1 {
        result = op(&a, &result);
    }

    result
}

fn fast_exp2x2(a: [[BigUint; 2]; 2], n: u64) -> [[BigUint; 2]; 2] {
    op_n_times(a, &mul2x2, n)
}

fn fib4(n: u64) -> BigUint {
    if n == 0 { return 0.to_biguint().unwrap(); }
    if n == 1 { return 1.to_biguint().unwrap(); }

    let a = [
        [1.to_biguint().unwrap(), 1.to_biguint().unwrap()],
        [1.to_biguint().unwrap(), 0.to_biguint().unwrap()],
    ];

    fast_exp2x2(a, n - 1)[0][0].clone()
}