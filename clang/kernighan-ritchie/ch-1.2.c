#include <stdio.h>

main()
{
    fromFahrToCelsius();
    printf("\n---------------------------\n");
    fromCelsiusToFahr();
}

void fromFahrToCelsius()
{
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    fahr = lower;
    printf("Fahr  Celsius\n");
    printf("------------\n");

    // (32 °F − 32) × 5/9 = 0 °C
    while (fahr <= upper)
    {
        celsius = (5.0 / 9.0) * (fahr - 32.0);
        printf("%3.0f %6.1f\n", fahr, celsius);
        fahr = fahr + step;
    }
}

void fromCelsiusToFahr()
{
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    celsius = lower;
    printf("Celsius  Fahr\n");
    printf("------------\n");

    // (32 °F − 32) × 5/9 = 0 °C
    while (celsius <= upper)
    {
        fahr = (celsius * 9.0 / 5.0) + 32;
        printf("%3.0f %6.1f\n", celsius, fahr);
        celsius = celsius + step;
    }
}