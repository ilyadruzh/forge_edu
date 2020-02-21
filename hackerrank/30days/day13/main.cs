using System;
using System.Collections.Generic;
using System.IO;
abstract class Book
{
    protected String title;
    protected String author;

    public Book(String t, String a)
    {
        title = t;
        author = a;
    }
    public abstract void display();
}

//Write MyBook class
class MyBook : Book
{
    public String bTitle;
    public String bAuthor;
    public int price;
    public MyBook(string _title, string _author, int _price) : base(_title, _author)
    {
        bTitle = _title;
        bAuthor = _author;
        price = _price;
    }

    public override void display()
    {
        Console.WriteLine("Title: " + bTitle + "\nAuthor: " + bAuthor + "\nPrice: " + price);
    }
}

class Solution
{
    static void Main(String[] args)
    {
        String title = Console.ReadLine();
        String author = Console.ReadLine();
        int price = Int32.Parse(Console.ReadLine());
        Book new_novel = new MyBook(title, author, price);
        new_novel.display();
    }
}