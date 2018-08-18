// Greatest common divisor - Наибольший общий делитель

#include <stdio.h>

int gcd(int a, int b)
{

    printf("a: %d, b: %d\t", a, b);

    int c = a / b;

    printf("c: %d\n", c);

    if (c == 0)
    {
        return b;
    }
    else
    {
        a = b;
        b = c;
        gcd(a, b);
    }
}

int main(int argc, char const *argv[])
{
    int a = 10;
    int b = 5;

    printf("Наибольший общий делитель для %d и %d = %d\n", a, b, gcd(a, b));
    return 0;
}
