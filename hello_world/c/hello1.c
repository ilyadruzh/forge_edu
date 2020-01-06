#include <stdio.h>

int main(int argc, char const *argv[])
{
#define square(x) (x * x)
    int b = square(2.5);
    int a = square(2 + b);
    printf("Hello world\n %d", a);
    return 0;
}
