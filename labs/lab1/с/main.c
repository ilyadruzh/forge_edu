#include <stdio.h>

int calculate(float x, float y)
{
    int res;

    if ((-2 <= x && x <= -1) && (y <= 1.5 * x + 3.5) && (y >= -1.5 * x - 2.5))
    {
        printf("Левый треугольник\n");
        return 1;
    }
    if ((1 <= x && x <= 2) && (y >= 1.5 * x - 2.5) && (y <= -1.5 * x + 3.5))
    {
        printf("Правый треугольник\n");
        return 1;
    }
    else if ((-1 <= y && y <= 2) && (-1 <= x && x <= 1))
    {
        printf("Прямоугольник\n");
        return 1;
    }
    else
    {
        return 0;
    }

    return res;
}

//
int main(int argc, char const *argv[])
{
    float x;
    float y;
    int result;

    // Вводим X: нужно сделать проверку, что x подходящее число. Может быть с плавающей точкой
    printf("Введите точку x: ");
    scanf("%f", &x);

    // Вводим Y: нужно сделать проверку, что x подходящее число. Может быть с плавающей точкой
    printf("\nВведите точку y: ");
    scanf("%f", &y);

    printf("Принадлежит ли точка (%f, %f) заштрихованной области?\n", x, y);

    result = calculate(x, y);

    if (result > 0)
    {
        printf("Да, точка (%f, %f) принадлежит заданной области\n", x, y);
    }
    else
    {
        printf("Нет,точка (%f, %f) не принадлежит заданной области\n", x, y);
    }

    return 0;
}