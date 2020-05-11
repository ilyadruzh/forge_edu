#include <stdio.h>

int main(int argc, char const *argv[])
{

    prog1();
    return 0;
}

int prog0(void)
{
    int z1, z2, z3;
    int x = 12, y = 2;
    z1 = x & y;
    z2 = x ^ y;
    z3 = x | y;

    printf("z1: %d\nz2: %d\nz3: %d\n", z1, z2, z3);
    printf("pointer z1: %d\npointer z2: %d\npointer z3: %d\n", &z1, &z2, &z3);
}

int prog1(void)
{
    int a, b, c;

    printf("a = ");
    scanf("%d", &a);
    printf("b = ");
    scanf("%d", &b);

    printf("summa=%d\n", a + b);
}
