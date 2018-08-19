#include <stdio.h>
#include <stdlib.h>

int main(int argc, char const *argv[])
{
    int num;
    num = 2;
    printf("Hello World\n");
    printf("Моя первая программа!\n");
    printf("Во %d-м семестре я изучаю язык Си!\n", num);
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
        int *x;
        float *k;
        x = (int)malloc(sizeof(int));     /* выделить память для хранения целочисленной переменной */
        k = (float)calloc(sizeof(float)); /* выделить память для хранения вещественной переменной */
    }
    return 0;
}
