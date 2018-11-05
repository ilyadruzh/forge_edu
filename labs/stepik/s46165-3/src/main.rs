use std::io::{self, BufRead, Read, Write};

// TODO: Напишите программу, которая получает на вход два числа 1≤n,m≤2⋅109
// и находит минимальное число квадратов одинакового размера,
// на которое можно разрезать прямоугольник размера n×m.
// Например, прямоугольник размера 6×10 можно разбить на 15 квадратов (и на меньшее число нельзя).

fn main() {
    let stdin = io::stdin();
    // find_sqaure(stdin);
    // last_digit(stdin);
}

fn find_sqaure(stdin: io::Stdin) -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut res = 0;
    let mut num_src = 0;
    let mut lcn = 0;
    let mut gcd_var = 0;

    let mut a_var = 0;
    let mut b_var = 0;

    let mut anum = 0;

    match input.trim() {
        input => {
            if let Ok(mut numbers) = parse_input(input) {
                a_var = nod(numbers[0]);
                b_var = nod(numbers[1]);
                num_src = numbers[0] * numbers[1];
                gcd_var = gcd1(numbers[0], numbers[1]);
                println!("gcd: {}", gcd_var);

                if (numbers[0] != numbers[1]) {
                    while numbers[1] != 0 {
                        let tmp = numbers[0];
                        numbers[0] = numbers[1];
                        numbers[1] = tmp % numbers[1];
                    }

                    println!("num[0]: {}", numbers[0]);

                    match numbers[0] % 10 {
                        1 => {
                            res = num_src;
                            res
                        }
                        0 => {
                            res = num_src;
                            res
                        }
                        _ => {
                            res = num_src / (gcd_var * gcd_var);
                            res
                        }
                    };
                } else {
                    res = num_src / (a_var * b_var);
                    println!("res: {}", res);
                    return res;
                }
            }
        }
    }

    println!("res: {}", res);

    return res;
}

fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|token| token.parse())
        .collect()
}

fn gcd1(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd1(b, a % b);
}

fn nod(x: u32) -> u32 {
    let mut res = 0;
    for i in 2..x {
        if x % i == 0 {
            res = i;
            return res;
        }
    }
    res
}

//TODO:  Напишите программу, которая получает на вход два числа 1≤n,m≤1018 и находит
// последнюю цифру наибольшего общего делителя чисел Фибоначчи Fn и Fm (то есть gcd(Fn,Fm)mod10).

// 34 51
// gcd: 17
// fib_src: 1597
// another way: 7

// 7 14
// gcd: 7
// fib_src: 3
// another way: 3

fn last_digit(stdin: io::Stdin) -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut another_way = 0;

    match input.trim() {
        input => {
            if let Ok(mut numbers) = parse_input(input) {
                let gcd_var = gcd(numbers[0], numbers[1]);
                println!("gcd: {}", gcd_var);

                let fib_src = fib3(gcd_var);
                println!("fib_src: {}", fib_src);

                another_way = fib3(gcd(numbers[0], numbers[1])) % 10;
                println!("another way: {}", another_way);
            }
            return another_way;
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    let mut res = 0;

    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    res = a;

    return res;
}

fn fib(x: u64) -> u64 {
    match x {
        0 => return 0,
        1 => return 1,
        _ => fib(x - 1) + fib(x - 2),
    }
}

fn fib_loop(x: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;

    match x {
        0 => x,
        1 => x,
        _ => {
            for x in 2..x + 1 {
                res = (a + b) % 10;
                a = b;
                b = res;
            }
            res
        }
    };

    return res;
}

fn fib3(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let a = [[1, 1], [1, 0]];

    fast_exp2x2(a, n - 1)[0][0]
}

fn exp2x2(a: [[u32; 2]; 2], n: u32) -> [[u32; 2]; 2] {
    let mut result = [[1, 0], [0, 1]];

    for _ in 0..n {
        result = mul2x2(result, a);
    }

    result
}

fn mul2x2(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[1][0] * b[0][1],
            a[0][0] * b[1][0] + a[1][0] * b[1][1],
        ],
        [
            a[0][1] * b[0][0] + a[1][1] * b[0][1],
            a[0][1] * b[1][0] + a[1][1] * b[1][1],
        ],
    ]
}

fn fast_exp2x2(a: [[u32; 2]; 2], n: u32) -> [[u32; 2]; 2] {
    if n == 1 {
        return a;
    }

    let mut result = fast_exp2x2(mul2x2(a, a), n >> 1);

    if n & 1 == 1 {
        result = mul2x2(a, result);
    }

    result
}
