## Создать новый проект

    dotnet new sln -o {ProjName} - создание solution в папке FSNetCore 
    dotnet new lib -lang F# -o src/Library - создание библиотеки
    dotnet new console -lang F# -o src/App - создание консольного приложения
    dotnet sln add src/App/App.fsproj - добавить в решение ссылку на приложение или библиотеку

## Компиляция

    dotnet build

## Запуск 

    dotnet run .
