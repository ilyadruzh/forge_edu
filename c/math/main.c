#include <stdio.h>
#include <string.h>
#include "mathbasic.h"

extern int sum(int a, int b);

int main(int argc, char const *argv[])
{
    int a = 2;
    int b = 3;
    int c = sum(1,3);

    printf("Hello world!\n");
    printf("MathFunc. Sum. a: %d, b: %d. a + b = %d\n", a, b, c);
    return 0;
}
