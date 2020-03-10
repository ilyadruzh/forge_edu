#include <stdio.h>

#define MAXLINE 1000

int getMyline(char line[], int maxline);
void copy(char to[], char from[]);
void reverse(char from[], char to[]);

int main()
{
    int len;
    int max;
    int count;
    char line[MAXLINE];
    char longest[MAXLINE];
    char goodlines[MAXLINE][MAXLINE];
    char reverline[MAXLINE];

    max = 0;
    count = 0;

    int revert = 0;
    int morethaneight = 0;

    while ((len = getMyline(line, MAXLINE)) > 0)
    {
        if (len > max)
        {
            max = len;
            copy(longest, line);
        }

        if (max > 0)
        {
            printf("longest: %s\n", longest);
        }

        if (morethaneight == 1 && len > 80)
        {
            copy(goodlines[count], line);
            ++count;
            for (int x = 0; x < count; ++x)
            {
                printf("more than 80 symbols[%d]: %s\n", x, goodlines[x]);
            }
        }
        
        if (revert == 1)
        {
            reverse(line, reverline);
            printf("%s\n", reverline);
        }
    }

    return 0;
}

int getMyline(char s[], int lim)
{
    int c, i;

    for (i = 0; i < lim - 1 && (c = getchar()) != EOF && c != '\n'; ++i)
    {
        s[i] = c;
    }

    if (c == '\n')
    {
        s[i] = c;
        ++i;
    }
    s[i] = '\0';
    return i;
}

void copy(char to[], char from[])
{
    int i;

    i = 0;

    while ((to[i] = from[i]) != '\0')
    {
        if (from[i] == '\n')
        {
        }
        ++i;
    }
}

void reverse(char from[], char to[])
{
    int start = 0;
    int x;

    int endofline = 0;

    for (int i = 0; i <= MAXLINE; ++i)
    {
        if (from[i] == '\n' || endofline == 1)
        {
            endofline = 1;
            from[i] = 0;
        }
    }

    for (x = MAXLINE; x >= 0; --x)
    {
        if (from[x] == '\n' || from[x] == '\0' || from[x] == 0)
        {
        }
        else
        {
            to[start] = from[x];
            ++start;
        }
    }

    // to[start + 1] = '\0';
}
