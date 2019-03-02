use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // let res = fact1(5);
    // println!("factorial 5: {}\n", res);
    // let res2 = sum_from_to(5, 10);
    // println!("res2: {}", res2);

    println!("fib(11) {}", fib(10));

    pisano_period(50, 3);
}

// Factorial version 1
fn fact1(x: i32) -> i32 {
    if (x == 1) {
        return 1;
    } else {
        x * fact1(x - 1)
    }
}

// Sum for all period
fn sum_from_to_v1(from: i32, to: i32) -> i32 {
    let mut res: i32 = 0;
    for num in from..to {
        res+=num
    }
    return res;
}
// в функциональном стиле
fn sum_from_to_v2(from: i32, to: i32) -> i32 {
    0
}

fn generate_workout(intensity: i32, random_number: i32) {
    let expensive_closure = |num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity))
        }
    }
}

fn fib(num: u64) -> u64 {

    let mut a = 0;
    let mut b = 1;
    let mut res = 0;

    match num {
        1 => return 1,
        0 => return 0,
        _ => {
            for x in 1..num {
                res = a + b;
                a = b;
                b = res;
            }

            res
        }
    }
}

fn pisano_period(fib_num: u64, num: u64) {

    let mut a = 0;
    let mut b = 1;
    let mut res = 0;

    let mut vec_arr = Vec::new();
    let mut count = 0;

    match fib_num {
        1 => println!("1"),
        0 => println!("0"),
        _ => {
            vec_arr.push(0);
            vec_arr.push(1);
            for x in 0..fib_num {

                res = a + b;
                // print!("- {} ", res % num);
                count += {
                    let x = res % num;
                    let y = (res +1) % num;

                    if x == 0 && y == 1 {
                    println!("\nx: {}", x);
                    println!("y: {}", y);
                        1
                    } else {
                        0
                    }
                };
                a = b;
                b = res;
                vec_arr.push(res % num);
            }
        }
    }
    
    let period = fib_num % count;

    println!("vec {:?}", vec_arr);
    println!("count {}", count);
    println!("period: {}", period);
}