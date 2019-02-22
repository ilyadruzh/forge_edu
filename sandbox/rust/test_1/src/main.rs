type price = i32;
type size = u32;
type meta = u128;

fn main() {
    let mut vec_1: Vec<(size, meta)> = Vec::new();
    let mut vec_2: Vec<(price, Vec<(size, meta)>)> = Vec::new();

    let nor: u8 = 1;
    println!("{}", nor);

    println!("Hello, world!");
}

//  Добавление (price, (size, meta)), сохраняя сортировку Vec<(size, meta)> по price.
// (size, meta) сохраняются по принципу FIFO.
// https://ru.wikipedia.org/wiki/FIFO
fn add_to_vec(p: price, s: size, m: meta) -> Vec<(price, (size, meta))> {

    let mut tmp: Vec<(price, (size, meta))> = Vec::new();
    tmp.insert(price, (price, (s, m)));

    return tmp;
}

// Разделение структуры на две по (price', size'),
// где в одной остаются все элементы price <= price',
// а сумма всех [size] <= size'. (Пара (size, meta) может быть разорвана на две).
fn div_vec(separator: i32) {

}

// Оценить сложность полученного алгоритма.
// Измерить время работы каждой из операций на рандомном наборе значений.
// – 100/1000 prices, 10/100 (size, meta) на каждый price;
// – разделение на равные по size доли;

