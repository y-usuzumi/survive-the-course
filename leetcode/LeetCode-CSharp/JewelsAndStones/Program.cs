using System;
using System.Collections.Generic;
using System.Net;

namespace JewelsAndStones
{
    public class Solution
    {
        public int NumJewelsInStones(string J, string S)
        {
            int count = 0;
            var hs = new HashSet<char>(J);
            foreach (var ch in S)
            {
                if (hs.Contains(ch))
                {
                    count++;
                }
            }

            return count;
        }
    }
        
    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            string j = "aA";
            string s = "aAAbbbb";
            Console.WriteLine("{0}", solution.NumJewelsInStones(j, s));
        }
    }
}