type price = i32;
type size = u32;
type meta = u128;

fn main() {
    //let mut vec_1: Vec<(size, meta)> = Vec::new(); // Vec<(size, meta)>

    let mut vec_2: Vec<(price, Vec<(size, meta)>)> = Vec::new();

    println!("Hello, world!");
}

//  Добавление (price, (size, meta)), сохраняя сортировку основного Vec по price. (size, meta) сохраняются по принципу FIFO.
fn add_to_vec(p: price, s: size, m: meta) -> Vec<(price, Vec<(size, meta)>)> {

    let mut tmp: Vec<(price, Vec<(size, meta)>)> = Vec::new();
    return tmp;
}

// Разделение структуры на две по (price', size'), где в одной остаются все элементы price <= price',
// а сумма всех [size] <= size'. (Пара (size, meta) может быть разорвана на две).
fn div_vec() {}

// Оценить сложность полученного алгоритма.
// Измерить время работы каждой из операций на рандомном наборе значений.
// – 100/1000 prices, 10/100 (size, meta) на каждый price;
// – разделение на равные по size доли;
