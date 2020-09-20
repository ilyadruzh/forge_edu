#include <iostream>
// #define MAX(a, b, c)                   \
//     {                                  \
//         int arg1 = (a), arg2 = (b);    \
//         c = arg1 > arg2 ? arg1 : arg2; \
//     }

int power(int, unsigned);

int power(int x, unsigned p)
{
    int answer = x;
    if (p == 0)
    {
        return 1;
    }
    else
    {
        for (int a = 1; a < p; a++)
        {
            answer = answer * x;
        }
    }
    return answer;
}

int main()
{
    int i = 42;
    double d = 3.14;
    const char *s = "C-style string";
    bool b = true;

    std::cout << "This is an integer " << i << "\n";
    std::cout << "This is a double " << d << "\n";
    std::cout << "This is a \"" << s << "\"\n";
    std::cout << "This is \"" << b << "\"\n";

    std::cout << "Enter an integer and a double:\n";

    std::cin >> i >> d;

    std::cout << power(i, d);

    std::cout << "Your input is " << i << ", " << d << "\n";

    // std::cout << MAX(4, 5, 3);

    return 0;
}