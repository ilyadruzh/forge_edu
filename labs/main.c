#include <stdio.h>
#include <stdlib.h>

// TODO: добавить диалог с пользователем с определением размера массива

// А) произведение элементов с нечетными значениями (нечетные индексы или нечетные значения в массивах)
// Б) количество элементов массива, расположенных между минимальным и максимальным элементами
// В) поменять порядок следования элементов в массиве (поменять местами i-й элемент c n – i-м элементом)

int main(int argc, char const *argv[])
{
    float x[20];
    float a = 5.66;

    for (int i = 0; i < 20; i++)
    {
        x[i] = (float)rand() / (float)(RAND_MAX / a);
        printf("% f \n", x[i]);
    }

    /// Task A: произведение элементов с нечетными значениями (нечетные индексы или нечетные значения в массивах)
    {
        float accA = 0;
        for (int i = 1; i < (int)(sizeof(x) / sizeof(float)); i = i + 2)
        {
            printf("x[i]: % f \n", x[i]);
            if (accA == 0.0)
            {
                accA = x[i];
            }
            else
            {
                accA = accA * x[i];
            }
        }
        printf("accA: % f \n", accA);
    }

    /// Task B: количество элементов массива, расположенных между минимальным и максимальным элементами
    {
        float max_var = 0;
        float min_var = 1;
        int min_index = 0;
        int max_index = 0;

        for (int i = 1; i < (int)(sizeof(x) / sizeof(float)); i++)
        {
            if (x[i] > max_var)
            {
                max_var = x[i];
                max_index = i;
            }
            if (x[i] < min_var)
            {
                min_var = x[i];
                min_index = i;
            }
        }

        // Подсчитываем кол-во элементов между минимальный и максимальны элементом
        // НЕ ВКЛЮЧАЯ ИХ в кол-во
        int count = (min_index < max_index) ? max_index - min_index - 2 : min_index - max_index - 2;
        printf("count: %d\n", count);

        printf("max_var: %f\n", max_var);     // максимальное значение
        printf("max index: %d\n", max_index); // индекс максимального значения

        printf("min_var: %f\n", min_var);     // минимальное значение
        printf("min index: %d\n", min_index); // индекс минимального значения

        /// Task B: поменять порядок следования элементов в массиве (поменять местами i-й элемент c n – i-м элементом)
        /// Нужно «перевернуть» массив
        {

            float x_reverse[20];

            for (int c = (int)(sizeof(x) / sizeof(float)) - 1, d = 0; c >= 0; c--, d++)
            {
                x_reverse[d] = x[c];
            }

            for (int c = 0; c < (int)(sizeof(x) / sizeof(float)); c++)
            {
                x[c] = x_reverse[c];
                printf("reversed array: x: %f \n", x[c]);
            }
        }
        return 0;
    }