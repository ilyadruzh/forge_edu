// Примитивная функция сложения
// TODO: принимать обобщеные типы и возвращать обобщенный тип Result, Option
// Проводить проверку внутри - возвращать или ошибку с описанием или результат суммирования
fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

fn find_hypotenuse(a: i64, b: i64) -> f64 {
    let a2 = a * a;
    let b2 = b * b;
    let h = ((a + b) as f64).sqrt();
    return h;
}

fn main() {

    let x = i32::max_value();

    let y = x.wrapping_add(1);
    assert_eq!(y, i32::min_value());

    let y = x.saturating_add(1);
    assert_eq!(y, i32::max_value());

    let (y, overflowed) = x.overflowing_add(1);
    assert!(overflowed);
    assert_eq!(y, i32::min_value());

    match x.checked_add(1) {
        Some(y) => unreachable!(),
        None => println!("overflowed"),
    }
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
