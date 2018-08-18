#include <stdio.h>
#include <string.h>
#include "mathfunc.h"

extern int sum(int a, int b);

int main(int argc, char const *argv[])
{
    int c = sum(1,3);

    printf("Hello world!\n");
    printf("MathFunc. Sum. a = {}, b = {}. a + b = {}\n");
    return 0;
}
