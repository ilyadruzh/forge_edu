### Лабораторная работа 2 "ОБРАБОТКА МАССИВОВ"

В одномерном массиве, состоящем из вещественных элементов, вычислить:

- А) произведение элементов с нечетными значениями
- Б) количество элементов массива, расположенных между минимальным и максимальным элементами
- В) поменять порядок следования элементов в массиве (поменять местами i-й элемент c n – i-м элементом)


### Решение A

1) создаем переменные для массива и аккумулятора
2) в первом цикле for заполняем массив случайными значениями
3) во втором цикле for проверяем каждый элемент массива на чётность
4) если значение не чётно, то умножаем аккумулятор на значение массива
5) Выводим на экран значение аккумулятора

### Решение Б

1) Вводим переменные для минимального значения `min` и для максимального `max`.
2) Проходим циклом по массиву и определяем позицию - индекс - минимального и максимального значения, записываем в переменные их индексы
3) Вычитаем из максимального значения минимальное значение - получаем кол-во элементов между ними

### Решение В

1) Создаем два массива: старый и новый, и временную переменную для счётчика
2) заполняем старый массив случайными значениями
3) в цикле фор проходим старый массив в обратном порядке и добавляем в новый массив значения

Нужно «перевернуть» массив.  
Т.е., если был массив   1 2 3 4 5, то после выполнения задания должен получиться массив 5 4 3 2 1. 

Если тоже самое записать через индексы массива, то в общем виде получится, что нужно i-тый элемент поменять с n-i-1 элементом. 

Для массива из 10-ти элементов (пять обменов):
0 <------>  9
1 <------>  8
2 <------>  7
3 <------>  6
4 <------>  5

## Отчет

### Текст индивидуального задания

### Алгоритм решения задачи

### Текст программы

### Результат тестирования программы

Инициализируем массив случайными числами

