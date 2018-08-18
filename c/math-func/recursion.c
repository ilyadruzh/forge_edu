#include <stdio.h>

// выполнить программу
// cc recursion.c -o bin/recursion && ./bin/recursion

int rec(int a)
{
    printf("%d\r", a);

    if (a > a - 1)
    {
        int i = a + 1;
        rec(i);
    }
}

int main()
{
    rec(1);
}