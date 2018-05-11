using System;
using System.Collections.Generic;

namespace FizzBuzz
{
    public class Solution {
        public IList<string> FizzBuzz(int n) {
            var ret = new List<string>(n);
            var fizzCnt = 0;
            var buzzCnt = 0;
            for (var i = 1; i <= n; i++)
            {
                fizzCnt++;
                buzzCnt++;
                if (fizzCnt == 3) {
                    if (buzzCnt == 5) {
                        ret.Add("FizzBuzz");
                        buzzCnt = 0;
                    } else {
                        ret.Add("Fizz");
                    }
                    fizzCnt = 0;
                } else if (buzzCnt == 5) {
                    ret.Add("Buzz");
                    buzzCnt = 0;
                } else {
                    ret.Add(i.ToString());
                }
            }
            return ret;
        }
    }
    
    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.FizzBuzz(15);
            foreach (var r in result)
            {
                Console.WriteLine(r);
            }
        }
    }
}