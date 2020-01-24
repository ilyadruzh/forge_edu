using System.CodeDom.Compiler;
using System.Collections.Generic;
using System.Collections;
using System.ComponentModel;
using System.Diagnostics.CodeAnalysis;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.Serialization;
using System.Text.RegularExpressions;
using System.Text;
using System;

class Solution
{


    static void Main(string[] args)
    {
        int n = Convert.ToInt32(Console.ReadLine());

        int[] arr = Array.ConvertAll(Console.ReadLine().Split(' '), arrTemp => Convert.ToInt32(arrTemp))
        ;

        int[] newArray = new int[arr.Length];

        for (int x = 0, y = arr.Length - 1; x < arr.Length; x++, y--)
        {
            newArray[x] = arr[y];
        }

        foreach (var item in newArray)
        {
            Console.Out.Write("{0} ", item.ToString());
        }

    }
}
