// #include <iostream.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
int a, b, n, i;
float h, x, s;
int main()
{
    printf("Vvedite interval [a;b]\n");
    printf("a=");
    scanf("%d", &a);
    printf("b=");
    scanf("%d", &b);
    printf("Vvedite kolvo tochek \n");
    scanf("%d", &n);
    h = (float)(b - a) / n;
    printf("h=%f", h);

    x = a;
    s = 0;
    for (i = 0; i <= (n - 1); i++)
    {
        s += (x * x) + (5 * x);
        x = x + h;
    }
    printf("\nReshenie naideno metodom levih prjamougolnikov");
    printf("\ns=%.2f \n", s);

    x = a + h / 2;
    s = 0;
    for (i = 0; i <= n; i++)
    {
        s += (x * x) + (5 * x);
        x = x + h;
    }
    printf("\nReshenie naideno metodom srednih prjamougolnikov");
    printf("\ns=%.2f \n", s);
    x = a + h;
    s = 0;
    for (i = 0; i <= n; i++)
    {
        s += (x * x) + (5 * x);
        x = x + h;
    }
    printf("\nReshenie naideno metodom pravih prjamougolnikov");
    printf("\ns=%.2f \n", s);

    system("PAUSE");
    return 0;
}