#include <stdio.h>
#define MAXLINE 1000

int max;
char line[MAXLINE];
char longest[MAXLINE];
char result[MAXLINE];
int tabwidth;

int getmyline(void);
void copy(void);

main()
{
    int len;
    extern int max;
    extern char longest[];

    max = 0;
    while ((len = getmyline()) > 0)
    {
        if (len > max)
        {
            max = len;
            copy();
        }

        if (max > 0)
        {
            detab(longest);
            // printf("%s", longest);
        }
    }

    // if (max > 0)
    // {
    //     printf("%s", longest);
    // }
    return 0;
}

int getmyline(void)
{
    int c, i;

    extern char line[];

    for (i = 0; i < MAXLINE - 1 && (c = getchar()) != EOF && c != '\n'; ++i)
    {
        line[i] = c;
    }
    if (c == '\n')
    {
        line[i] = c;
        ++i;
    }
    line[i] = '\0';
    return i;
}

void copy(void)
{
    int i;
    extern char line[], longest[];

    i = 0;

    while ((longest[i] = line[i]) != '\0')
    {
        ++i;
    }
}

void detab(char s[])
{
    extern char result[MAXLINE];
    extern int tabwidth;

    tabwidth = 4;

    int result_len = 0;

    // stack smashing detected ***: <unknown> terminated
    for (int i = 0; i < MAXLINE - 5; ++i)
    {
        if (s[i] == '\t')
        {
            for (int x = 0; x <= tabwidth; ++x)
            {
                result[result_len + x] = '.';
                printf("result_len: %d\n", result_len);
            }
            result_len = result_len + 4;
        }
        else
        {
            result[result_len] = s[i];
            ++result_len;
        }
    }

    printf("s: %s\n", s);
    printf("result: %s\n", result);
}

void entab(char s[])
{
    // TODO: 1.21
}

void fold(char s[])
{
    // TODO: 1.22
}

void delete_comments(char s[])
{
    // TODO: 1.23
}

void syntax_check(char s[])
{
    // TODO:
}
