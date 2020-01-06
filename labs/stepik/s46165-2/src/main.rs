// алгоритм Евклида
use std::io::{self, BufRead, Read, Write};

fn main() {
    let stdin = io::stdin();
    println!("{}", greatest_common_divisor_euclidean_1(stdin)); // Наибольший общий делитель, через алгоритм Евклида
    gcd_for_fib_nums(3, 5); //TODO: Наибольший общий делитель соседних чисел Фибоначчи
}

fn greatest_common_divisor_euclidean_1(stdin: io::Stdin) -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut res = 0;

    match input.trim() {
        input => {
            if let Ok(mut numbers) = parse_input(input) {
                println!("numbers: {:?}", numbers);

                while numbers[1] != 0 {
                    let tmp = numbers[0];
                    numbers[0] = numbers[1];
                    numbers[1] = tmp % numbers[1];
                }
                res = numbers[0];
            }
        }
    }

    return res;
}

fn gcd_for_fib_nums(a: u32, b: u32) {
    println!("{}, {}", a, b);
}

fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|token| token.parse())
        .collect()
}
