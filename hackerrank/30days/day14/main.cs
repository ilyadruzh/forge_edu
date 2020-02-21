using System;
using System.Linq;

class Difference
{
    private int[] elements;
    public int maximumDifference;

    // Add your code here
    public Difference(int[] _a)
    {
        elements = _a;
    }

    // 8 19 3 2 7
    public void computeDifference()
    {
        int max = 0;

        for (int x = 0; x < elements.Length; x++)
        {
            for (int y = x; y < elements.Length; y++)
            {
                int tmp = Math.Abs(elements[x] - elements[y]);
                if (tmp > max)
                {
                    max = tmp;
                }
            }
        }

        maximumDifference = max;
    }

} // End of Difference Class

class Solution
{
    static void Main(string[] args)
    {
        Convert.ToInt32(Console.ReadLine());

        int[] a = Console.ReadLine().Split(' ').Select(x => Convert.ToInt32(x)).ToArray();

        Difference d = new Difference(a);

        d.computeDifference();

        Console.Write(d.maximumDifference);
    }
}