# README

Имеется следующая структура данных: Vec<(price, Vec<(size, meta)>).

Написать имплементацию для следующих операций:

– Добавление (price, (size, meta)), сохраняя сортировку основного Vec по price. (size, meta) сохраняются по принципу FIFO.
- Разделение структуры на две по (price', size'), где в одной остаются все элементы price <= price', а сумма всех [size] <= size'. (Пара (size, meta) может быть разорвана на две).
 
Оценить сложность полученного алгоритма.
Измерить время работы каждой из операций на рандомном наборе значений.
– 100/1000 prices, 10/100 (size, meta) на каждый price;
– разделение на равные по size доли;
