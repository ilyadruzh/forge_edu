type price = usize;
type size = u32;
type meta = u128;

// Recomendation
// сразу задать размер всех векторов, так они работают быстрее
// 

// TODO create with 'self' and Trait semantics
fn main() {
    let vec: Vec<(price, Vec<(size, meta)>)> = create_vec();
    let separator: price = 400;
    let (res_1, res_2) = div_vec(create_vec(), separator);
    println!("res_1: {:?}\nres_2: {:?}", res_1.len(), res_2.len());
}

fn create_vec() -> Vec<(price, Vec<(size, meta)>)> {
    let mut vec: Vec<(price, Vec<(size, meta)>)> = Vec::new();

    for p in 100..1000 {
        for s in 10..100 {
            for m in 10..100 {
                let mut tmp1 = Vec::new();
                tmp1.push((s, m));
                vec.push((p, tmp1));
            }
        }
    }
    vec
}

//  Добавление (price, (size, meta)), сохраняя сортировку Vec<(size, meta)> по price.
// (size, meta) сохраняются по принципу FIFO.
// https://ru.wikipedia.org/wiki/FIFO
fn add_to_vec(p: price, s: size, m: meta) -> Vec<(price, Vec<(size, meta)>)> {
    let mut tmp: Vec<(price, Vec<(size, meta)>)> = Vec::new();
    let mut item: Vec<(size, meta)> = Vec::new();
    item.push((s, m));
    tmp.insert(p, (p, item)); // push into vec with sort by price
    return tmp;
}

// Разделение структуры на две по (price', size'),
// где в одной остаются все элементы price <= price',
// а сумма всех [size] <= size'. (Пара (size, meta) может быть разорвана на две).
pub fn div_vec(
    vec: Vec<(price, Vec<(size, meta)>)>,
    separator: price,
) -> (Vec<(price, size)>, Vec<(price, size)>) {
    // TODO: Vec<(price', size')> size' = sum of all sizes in that price
    let mut vec_1: Vec<(price, size)> = Vec::new();
    // other Vec<(price, size)>
    let mut vec_2: Vec<(price, size)> = Vec::new();

    // map()a

    vec.map()


    (vec_1, vec_2)
}

// Оценить сложность полученного алгоритма.
// Измерить время работы каждой из операций на рандомном наборе значений.
// – 100/1000 prices, 10/100 (size, meta) на каждый price;
// – разделение на равные по size доли;
