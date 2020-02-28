#include <stdio.h>

#define IN 1
#define OUT 0

int main()
{
    // int c;
    // c = getchar();
    // while (c != EOF)
    // {
    //     putchar(c);
    //     c = getchar();
    // }

    // wharValueHaveGetCharEOF();
    // printEOF();
    // changeSymbols();
    eachWordOnNewLine();
    return 0;
}

void wharValueHaveGetCharEOF()
{
    int result = (getchar() != EOF);
    printf("getchar() != EOF: %d\n", result);
}

void printEOF()
{
    printf("EOF: %d\n", EOF);
}

void countOfEscapeSymbols()
{
    int c, nl, tab, whitespace;
    whitespace = tab = nl = 0;

    while ((c = getchar()) != EOF)
    {
        if (c == '\n')
        {
            nl++;
            printf("newline: %d\n", nl);
            printf("tab: %d\n", tab);
            printf("whitespace: %d\n", whitespace);
        }
        else if (c == '\t')
        {
            tab++;
        }
        else if (c == ' ')
        {
            whitespace++;
        }
        else
        {
        }
    }
}

void trimAlotWhitespace()
{
    char c;
    int ws;
    ws = c = 0;
    while ((c = getchar()) != EOF)
    {
        if (ws == 0 && c == ' ')
        {
            ws++;
            printf("%c", c);
        }
        else if (ws >= 1 && c == ' ')
        {
        }
        else
        {
            ws = 0;
            printf("%c", c);
        }
    }
}

void changeSymbols()
{
    char c;
    c = 0;
    while ((c = getchar()) != EOF)
    {
        if (c == '\t')
        {
            printf("\\t");
        }
        else if (c == '\b')
        {
            printf("\\b");
        }
        else if (c == '\\')
        {
            printf("\\");
        }
        else
        {
            printf("%c", c);
        }
    }
}

void worldCount()
{
    int c, nl, nw, nc, state;
    state = OUT;
    nl = nw = nc = 0;

    while ((c = getchar()) != EOF)
    {
        ++nc;
        if (c == '\n')
            ++nl;
        if (c == ' ' || c == '\n' || c == '\t')
            state = OUT;
        else if (state == OUT)
        {
            state = IN;
            ++nw;
        }
    }

    printf("%d %d %d\n", nl, nw, nc);
}

void eachWordOnNewLine()
{
    int c, nl, nw, nc, state;

    while ((c = getchar()) != EOF)
    {
        if (c == ' ' || c == '\n' || c == '\t')
            c = '\n';
        printf("%c", c);
    }
}