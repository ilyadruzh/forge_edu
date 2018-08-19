#include <stdio.h>
#include <stdlib.h>
// Подключим заголовочный файл с описаниями API функций
#include <windows.h>

int main(int argc, char *argv[])
{
    system("chcp 1251");
    printf("Нажмите клавишу управления курсором: ");
    // организуем бесконечный цикл
    while (1)
    {
        // Если нажата клавиша «Вверх»
        if (GetAsyncKeyState(VK_UP))
        {
            keybd_event(VK_UP, 0, KEYEVENTF_KEYUP, 0);
            printf("\nНажата клавиша Вверх\n");
            system("pause");
            system("cls"); // очистить экран
            printf("Нажмите клавишу управления курсором: ");
        }
        // Если нажата клавиша «Вниз»
        if (GetAsyncKeyState(VK_DOWN))
        {
            keybd_event(VK_DOWN, 0, KEYEVENTF_KEYUP, 0);
            printf("\nНажата клавиша Вниз\n");
            system("pause");
            system("cls");
            printf("Нажмите клавишу управления курсором: ");
        }
        // Если нажата клавиша «Вправо»
        if (GetAsyncKeyState(VK_RIGHT))
        {
            keybd_event(VK_RIGHT, 0, KEYEVENTF_KEYUP, 0);
            printf("\nНажата клавиша Вправо\n");
            system("pause");
            system("cls");
            printf("Нажмите клавишу управления курсором: ");
        }
        // Если нажата клавиша «Влево»
        if (GetAsyncKeyState(VK_LEFT))
        {
            keybd_event(VK_LEFT, 0, KEYEVENTF_KEYUP, 0);
            printf("\nНажата клавиша Влево\n");
            system("pause");
            system("cls");
            printf("Нажмите клавишу управления курсором: ");
        }
        // Если нажата клавиша ESC
        if (GetAsyncKeyState(VK_ESCAPE))
        {
            keybd_event(VK_ESCAPE, 0, KEYEVENTF_KEYUP, 0);
            printf("\nКонец работы\n");
            system("pause");
            system("cls");
            break; // закончить выполнение цикла
        }
    }
    return 0;
}