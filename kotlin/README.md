## Компиляция исполняемого файла

    kotlinc {filename}.kt -include-runtime -d bin/{filename}.jar

## Запуск исполняемого файла

    java -jar {filename}.jar

## Компиляция библиотеки

    kotlinc {filename}.kt -d bin/{filename}.jar