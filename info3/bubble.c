#include <stdio.h>
#define N 1000
int main()
{
    int n = 10;
    int a[10] = {79, 83, 92, 84, 24, 45, 94, 58, 25, 90};
    int i, j, k;
    int count = 0;
    int x, z;

    // for (i = 0; i < n; i++)
    // {
    //     for (j = n - 1; j > i; j--)
    //     {
    //         x = a[j - 1];
    //         a[j - 1] = a[j];
    //         a[j] = x;
    //         count = count + 1;
    //     }
    // }

    printf("%d\n", count);

    // for (i = 0; i < n - 1; i++)
    // {
    //     for (j = 0; j < n - i - 1; j++)
    //     {
    //         if (a[j] > a[j + 1])
    //         {
    //             int tmp = a[j];
    //             a[j] = a[j + 1];
    //             a[j + 1] = tmp;
    //         }
    //     }
    // }
}