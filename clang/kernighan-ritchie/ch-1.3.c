#include <stdio.h>

main()
{
    reverseFahrToCelsius();
}

void reverseFahrToCelsius()
{
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    fahr = upper;
    printf("Fahr  Celsius\n");
    printf("------------\n");

    // (32 °F − 32) × 5/9 = 0 °C
    while (fahr >= lower)
    {
        celsius = (5.0 / 9.0) * (fahr - 32.0);
        printf("%3.0f %6.1f\n", fahr, celsius);
        fahr = fahr - step;
    }
}