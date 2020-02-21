using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
class Solution {

    static void Main(String[] args) {
        string S = Console.ReadLine();

        try
        {
            int result = Int32.Parse(S);
            Console.WriteLine(result);
        }
        catch (System.Exception)
        {
            Console.WriteLine("Bad String");
            throw;
        }
    }
}