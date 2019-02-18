#include <stdio.h>
#include <string.h>
#include "mathbasic.h"

extern int sum(int a, int b);

int gcd(int a, int b) {

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

int main(int argc, char const *argv[]) {
    int a = 2;
    int b = 3;
    int c = sum(1,3);

    printf("MathFunc. Sum. a: %d, b: %d. a + b = %d\n", a, b, c);
    printf("Greatest common divisor for %d and %d = %d\n", a, b, gcd(a, b));

    return 0;
}
