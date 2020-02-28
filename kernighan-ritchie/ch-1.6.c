#include <stdio.h>

#define IN 1
#define OUT 0

int main()
{
    worldLenght();
    return 0;
}

void digitcount()
{
    int c, i, nwhite, nother;
    int ndigit[10];

    nwhite = nother = 0;

    for (i = 0; i < 10; ++i)
    {
        ndigit[i] = 0;
    }

    while ((c = getchar()) != EOF)
    {
        if (c >= '0' && c <= '9')
            ++ndigit[c - '0'];
        else if (c == ' ' || c == '\n' || c == '\t')
            ++nwhite;
        else
            ++nother;

        printf("digits =");
        for (i = 0; i < 10; ++i)
            printf(" %d", ndigit[i]);

        printf(", whitespace = %d, other = %d\n", nwhite, nother);
    }
}

void worldLenght()
{
    int c, i, x, max, counter, worldCount;
    int lenght[10];
    max = counter = worldCount = 0;

    while ((c = getchar()) != EOF)
    {
        counter = counter + 1;
        if (c == '\n')
        {
            worldCount = worldCount + 1;
            lenght[worldCount] = counter;
            counter = 0;

            break;
        }
        else if (c == ' ' || c == '\t')
        {
            if (max < counter)
            {
                max = counter;
            }
            lenght[worldCount] = counter;
            counter = 0;
            worldCount = worldCount + 1;
        }
    }

    printf("max: %d\n", max);
    printf("worldCount: %d\n", worldCount);

    for (x = max; x > 0; x = x - 1)
    {
        printf("\n");
        for (i = 0; i < worldCount; i++)
        {

            if (lenght[i] < x)
            {
                printf(" ");
            }
            else
            {
                printf("-");
            }
        }
    }
    printf("\n");
}

// TODO: 1.14
