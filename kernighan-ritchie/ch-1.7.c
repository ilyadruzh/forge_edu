#include <stdio.h>

main()
{
    fromFahrToCelsius(0, 300, 20);
}

void fromFahrToCelsius(int lower, int upper, int step)
{
    float fahr, celsius;

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
