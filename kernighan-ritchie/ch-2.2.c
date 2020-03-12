#include <stdio.h>
#include <limits.h>
#include <float.h>

int main(int argc, char const *argv[])
{
    /* code */
    printf("\n\tFROM LIMITS\n\n");
    printf("char min: %d max: %d\n", CHAR_MIN, CHAR_MAX);
    printf("short min: %d max: %d\n", SHRT_MIN, SHRT_MAX);
    printf("int min: %d max: %d\n", INT_MIN, INT_MAX);
    printf("long min: %ld max: %ld\n", LONG_MIN, LONG_MAX);
    // printf("signed long min: %ld max: %ld\n", LLONG_MIN, LONG_MAX);

    printf("\n\tCALCULATE\n\n");

    // TODO

    float x = 0.0;
    printf("float size: %ld \n", size_t(x));

    return 0;
}
