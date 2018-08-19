use std::num::sqrt;

// Примитивная функция сложения
// TODO: принимать обобщеные типы и возвращать обобщенный тип Result, Option
// Проводить проверку внутри - возвращать или ошибку с описанием или результат суммирования
fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

// Реализация рекурсии
fn recursion(a: i64, b: i64) -> i64 {
    return 0;
}

// НАйти гипотенузу
fn find_hypotenuse(a: i64, b: i64) -> i64 {
    let a2 = a * a;
    let b2 = b * b;
    let h = (a + b).sqrt();
    return h;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_is_works() {
        assert_eq!(7, sum(2, 5));
    }

    #[test]
    #[should_panic(expected = "Выход за пределы типа")]
    fn greater_than_i64() {
        let x = sum(18446744073709551615, 18446744073709551615);
        println!("x: {}", x);
    }

}
