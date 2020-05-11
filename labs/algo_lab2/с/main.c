#include <stdio.h>
#include <stdlib.h>
#define ARRAY_LEN_A 10
#define ARRAY_LEN_B 10
#define ARRAY_LEN_C 10

void solution_a(void);
void solution_b(void);
void solution_c(void);

int main(int argc, char const *argv[])
{
    solution_a();
    solution_b();
    solution_c();

    return 0;
}

// Произведение элементов с нечетными значениями
void solution_a()
{
    int array[ARRAY_LEN_A];
    int acc = 1;

    for (int a = 0; a < ARRAY_LEN_A - 1; ++a)
    {
        array[a] = rand() % 20;
        printf("%d ", array[a]);
    }
    for (int b = 0; b < ARRAY_LEN_A - 1; ++b)
        if (array[b] % 2 != 0)
            acc = acc * array[b];

    printf("\n1. Произведение элементов с нечетными значениями: %d\n", acc);
}

// Количество элементов массива, расположенных между минимальным и максимальным элементами
void solution_b()
{
    int array[ARRAY_LEN_B];
    int max, min;
    int max_idx, min_idx = 0;
    int count = 0;
    printf("\n");

    for (int a = 0; a < ARRAY_LEN_B - 1; ++a)
    {
        array[a] = rand() % 100;
        printf("%d ", array[a]);
    }

    max = min = array[0];

    for (int b = 0; b < ARRAY_LEN_B - 1; ++b)
    {
        if (array[b] > max)
        {
            max = array[b];
            max_idx = b;
        }
        if (array[b] < min)
        {
            min = array[b];
            min_idx = b;
        }
    }

    if (max_idx > min_idx)
        count = max_idx - min_idx;
    else
        count = min_idx - max_idx;

    printf("\n2. Количество элементов массива, расположенных между min и max элементами: %d\n", count - 1);
    printf("Индекс максимального элемента: %d\n", max_idx);
    printf("Индекс минимального элемента: %d\n", min_idx);
}

void solution_c()
{
    int array_old[ARRAY_LEN_C];
    int array_new[ARRAY_LEN_C];
    int counter = 0;

    printf("\n3. Поменять порядок следования элементов в массиве\n");
    printf("array_old: ");

    for (int a = 0; a < ARRAY_LEN_C - 1; ++a)
    {
        array_old[a] = rand() % 10;
        printf("%d ", array_old[a]);
    }

    printf("\narray_new: ");

    for (int b = ARRAY_LEN_C - 2; b >= 0; --b)
    {
        array_new[counter] = array_old[b];
        printf("%d ", array_new[counter]);
    }
    printf("\n");
}