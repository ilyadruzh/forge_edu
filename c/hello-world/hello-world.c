#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define PI 3.141
#define eps 0.0001 // макро, задающее точность вычислений
// макро, определяющее значение функции при заданном х
#define func(x) (sin(x) - x + 0.15)
// макро, определяющее вычисление производной при заданном х
#define delta(x0, x1) (func(x1) - func(x0)) / (x1 - x0)

int main(int argc, char const *argv[])
{
    int num;
    num = 2;
    printf("Hello World\n");
    printf("Моя первая программа!\n");
    printf("Во %d-м семестре я изучаю язык Си!\n", num);

    ////////////////////////////////////////////////

    void change(int *n, int *m)
    {
        int buf = *n;
        *n = *m;
        *m = buf;
    }

    ////////////////////////////////////////////////

    int fibonacci(int n)
    {
        if (n == 0)
            return 1;
        if (n == 1)
            return 1;
        return (fibonacci(n - 1) + fibonacci(n - 2));
    }

    ////////////////////////////////////////////////

    int Search(int *x, int k, int n)
    {
        for (int i = 0; i < n; i++)
        {
            /* цикл закончит свою работу после того, как будет найдено нужное значение */
            if (x[i] == k)
                return i;
        }
        // или будут просмотрены все элементы массива
        return -1;
    }

    ////////////////////////////////////////////////

    {
        int i, n;

        for (i = 0; i < 10; i++)
        {
            n += 1;
        }
        printf("n: %d\n", n);
    }

    ////////////////////////////////////////////////

    {
        int n = 12;
        while (n < 20)
        {
            n--;
        }
        printf("%d\n ", n);
    }

    ////////////////////////////////////////////////

    {
        printf("/n/n // Вопрос 10 // /n/n\n");
    }

    ///////////////////////////////////////////////

    {
        //     int *x;
        //     float *k;
        //     x = (int)malloc(sizeof(int));     /* выделить память для хранения целочисленной переменной */
        //     k = (float)calloc(sizeof(float)); /* выделить память для хранения вещественной переменной */
    }

    /////////////////////////////////////////////

    {
        float x1 = 12 / 25;
        printf("x1 = %f\n", x1);
        int x2 = 25 / 12;
        printf("x2 = %d\n", x2);

        int *p = (int *)malloc(sizeof(int));
        int x3 = 5;
        *p = 5 * x3;

        printf("x3 = %d, p = %ls\n", x3, p);
    }

    /////////////////////////////////////////////

    {
        // printf("Пример использования функции printf:\nчисло PI = %8.2f", PI);
    }

    /////////////////////////////////////////////

    {
        char surname[30]; // описание строки символов
        char name[20];    // описание строки символов
        // печать строки
        printf("Введите, пожалуйста, свою фамилию: ");
        scanf("%s", surname);                         // ввод строки с клавиатуры
        printf("Введите, пожалуйста, Ваше имя: ");    //печать строки
        scanf("%s", name);                            // ввод строки с клавиатуры
        printf("Введите произвольное целое число->"); //печать строки
        int x, y;                                     // описание целочисленных переменных
        scanf("%d", &x);                              // ввод целой переменной
        // печать строки
        printf("Введите второе произвольное целое число->");
        scanf("%d", &y); // ввод целой переменной
        printf("Здравствуйте, %s %s \n", name, surname);
        printf("Сумма двух чисел %d + %d = %d\n", x, y, x + y);
        printf("Разность двух чисел %d-%d = %d\n", x, y, x - y);
    }

    {
        int x = 12;
        float y = 25.5;
        printf("\n *** %8.3f ***\n", y / x);
    }

    {
        // найдите ошибки
        float x4 = 12.1;
        float y4 = 13.12;
        printf("Значение x = %f \nЗначение y = %f ", &x4, &y4);
    }

    { // найдите ошибки

        float x = 12.1;
        float y = 13.12;
        float m = x + y;
        printf("Пример вычисления суммы: %f");
    }

    {
        printf("Программа вычисления значения по заданной формуле\n");
        float x, y, z;
        printf("Введите значение x -->");
        scanf("%f", &x);
        printf("Введите значение y -->");
        scanf("%f", &y);
        printf("Введите значение z -->");
        scanf("%f", &z);
        float f = x * x * x + y;
        f = f / (x * x + z - y);
        f += x * z;
        f -= 2 * sin(y);
        printf("\n Введены значения %5.2f %5.2f  %5.2f \n"
               " Значение f = %8.3f \n",
               x, y, z, f);
    }

    {
        printf("Программа вычисления по заданной формуле\n");
        float x, y, z;
        printf("Введите значение x -->");
        int m = scanf("%f", &x);
        if (m != 1)
        {
            printf("Введены некорректные данные.\n ");
            system("pause");
            return 0;
        }
        printf("Введите значение y -->");
        m = scanf("%f", &y);
        if (m != 1)
        {
            printf("Введены некорректные данные.\n ");
            system("pause");
            return 0;
        }
        printf("Введите значение z -->");
        m = scanf("%f", &z);
        if (m != 1)
        {
            printf("Введены некорректные данные.\n ");
            system("pause");
            return 0;
        }
        float f = x * x * x + y;
        float f1 = x * x + z - y;
        if (f1 == 0)
        {
            printf("Ошибка - деление на 0!!!\n");
            system("pause");
            return 0;
        }
        f = f / f1;
        f += x * z;
        f -= 2 * sin(y);
        printf("\n Введены значения %5.2f %5.2f %5.2f \n "
               "Значение f = %8.3f \n",
               x, y, z, f);
    }

    {
        int x = 2;
        int y = 3;
        int k;
        printf("\n Введите 1, для вычисления суммы x + y \n");
        printf("Введите 2, для вычисления произведения x*y \n");
        printf("Введите 3, для вычисления разности x - y \n");
        printf("Введите 4, для вычисления разности y - x \n");
        scanf("%d", &k);
        switch (k)
        {
        case 1:
            printf("Сумма равна %d \n", x + y);
            break;
        case 2:
            printf("Произведение равно %d \n", x * y);
            break;
        case 3:
            printf("Разность равна %d \n", x - y);
            break;
        case 4:
            printf("Разность равна %d \n", y - x);
            break;
        default:
            printf("Введено число, не принадлежащее интервалу [1;4]!\n");
        }
    }

    {
        int i, n;
        printf("Введите целое число n: ");
        scanf("%d", &n);
        int factorial = 1;
        for (i = 2; i <= n; i++)
            factorial *= i;
        printf("Значение факториала числа %d равно %d \n", n, factorial);
    }
    {
        int f = 1;
        do
        {
            int z = rand() % 100;
            if (z > 30)
                continue;
            if (z < 10)
                f = 0;
            printf("%d", z);
        } while (f);
    }

    // реализовать тоже самое на других языках программирования
    {
        double eps, q, q1, S = 0;
        int n = 1;
        printf("Введите значение точности: ");
        scanf("%lf", &eps);
        q = 1 / 6.; // Вычисление значения первого элемента ряда
                    // Обратите внимание на операцию деления двух констант
        while (q >= eps)
        {
            if (n % 2 == 0)
                S += q;
            else
                S -= q;
            q = q / (n + 3);
            n++;
        }
        printf("Количество итераций %d\nЗначение суммы %8.3lf\n", n - 1, S);
    }

    {
        system("chcp 1251");
        double z = 1.25, eps, x, z1;
        printf("Введите значение точности: ");
        scanf("%lf", &eps);
        double d = eps + 1;
        printf("Введите значение х: ");
        scanf("%lf", &x);
        while (d >= eps)
        {
            z1 = z / 2 + x / (2 * z);
            d = fabs(z - z1);
            z = z1;
        }
        printf("Квадратный корень из числа %8.3lf равен %9.4lf\n", x, z);
        printf("С использованием функции sqrt - %9.4lf\n", sqrt(x));
        system("PAUSE");
        return 0;
    }

    {
        double x0 = 0.5;
        double x1 = 0.52;
        double x2;
        double k;
        double z = func(x1);

        while (fabs(x0 - x1) > eps)
        {
            k = delta(x0, x1);
            x2 = x1 - z / k;
            x0 = x1;
            x1 = x2;
            z = func(x1);
        }

        printf("Корень уравнения --> %6.4f\n", x1);
        printf("Проверка решения func(%6.4f) = %6.4f\n", x1, func(x1));
    }

    {
        int k = 12;
        int x = 1;
        for (int i = 0; i < k; i++)
        {
            x += i;
            printf("%d \n", i);
        }
    }

    {
        int x, y;
        printf("Введите целое x: ");
        scanf("%d", &x);
        printf("Введите целое y: ");
        scanf("%d", &y);
        change(&x, &y); // вызов функции с адресами переменных x и y
        printf(" x = %d\n y = %d\n", x, y);
    }

    {
        int n;
        printf("Введите целое n: ");
        scanf("%d", &n);
        int x = fibonacci(n);
        printf(" x [%d] = %d\n", n, x);
    }

    {
        // Описание указателя на целое число
        int *x;
        int n, i;
        printf("Введите размерность массива: ");
        scanf("%d", &n);
        // выделение памяти
        x = (int *)malloc(sizeof(int) * n);
        for (i = 0; i < n; i++)
        {
            printf("x[%d] -> ", i);
            // чтение элементов с клавиатуры
            scanf("%d", &x[i]);
        }
        // очистка экрана
        system("cls");
        // печать элементов массива
        for (i = 0; i < n; i++)
            printf("%5d", x[i]);
        // освобождение памяти
        free(x);
        printf("\n");
    }

    {
        // функция рандомизации ядра датчика случайных чисел
        srand();
        int *x;
        int n, i;
        printf("Введите размер массива: ");
        scanf("%d", &n);
        x = (int *)malloc(sizeof(int) * n);
        for (i = 0; i < n; i++)
        {
            x[i] = rand() % 300;
            printf("%d ", x[i]);
        }
        int y;
        printf("\nВведите значение для поиска: ");
        scanf("%d", &y);
        int ind = Search(x, y, n);
        if (ind == -1)
            printf("В массиве нет элемента со значением %d\n", y);
        else
            printf("Элемент %d с индексом %d\n", y, ind);
        free(x);
    }

    {
        int X[6] = {0, -3, 2, -11, -5, 4};
        int n = 6, max = -1000, i;
        for (i = 0; i < n; i++)
            if (X[i] > max && X[i] < 0)
                max = X[i];

        printf("%d \n", max);
        return 0;
    }

    return 0;
}
