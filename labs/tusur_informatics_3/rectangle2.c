// #include <iostream>
#include <math.h>
#include <stdio.h>

double cos(double a);
float cosf(float a);
long double cosl(long double a);

float f(float x)
{
    return (1 + x * cosf(x)) * (1 + x * cosf(x));
}
float calcInter(int a, int b, int n)
{
    float h, S, x;
    int i;                  // Итератор
    h = (b - a) / (float)n; // n - число разбиений 10; 40; 160; 640.
    S = 0.0;                // Интеграл
    for (i = 0; i < n; i++)
    {
        x = a + h * (i + 0.5);
        S = S + f(x);
    }
    S = h * S;
    return S;
}
void main()
{
    float y;
    y = calcInter(-6, -5, 10);
    printf("f: %f\n", y);
}