using System;
using System.Collections.Generic;
using System.IO;

class Solution
{
    static void Main(String[] args)
    {
        int i = 4;
        double d = 4.0;
        string s = "HackerRank ";

        // Declare second integer, double, and String variables.
        int i2;
        double d2;
        string s2;

        // Read and save an integer, double, and String to your variables.
        i2 = System.Convert.ToInt32(Console.In.ReadLine());
        d2 = double.Parse(Console.In.ReadLine());
        s2 = Console.In.ReadLine();

        // Print the sum of both integer variables on a new line.
        Console.Out.WriteLine(i + i2);

        // Print the sum of the double variables on a new line.
        double result2 = d + d2;
        Console.Out.WriteLine(String.Format("{0}", result2));

        // Concatenate and print the String variables on a new line
        Console.Out.WriteLine("{0}{1}", s, s2);

        // The 's' variable above should be printed first.

    }