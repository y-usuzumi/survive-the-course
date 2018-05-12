using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Reflection.Metadata.Ecma335;
using System.Text;

namespace DecodeString
{
    public class Solution {
        public string DecodeString(string s)
        {
            var cntStack = new Stack<int>();
            var sStack = new Stack<StringBuilder>();
            var num = 0;
            var currSB = new StringBuilder();
            foreach (var c in s)
            {
                switch (c)
                {
                    case char x when x >= '0' && x <= '9':
                        num *= 10;
                        num += x - '0';
                        break;
                    case '[':
                        cntStack.Push(num);
                        sStack.Push(currSB);
                        currSB = new StringBuilder();
                        num = 0;
                        break;
                    case ']':
                        var repeat = cntStack.Pop();
                        var sb = sStack.Pop();

                        for (var i = 0; i < repeat; i++)
                        {
                            sb.Append(currSB);
                        }
                        currSB = sb;
                        break;
                    default:
                        currSB.Append(c);
                        break;
                }
            }

            return currSB.ToString();
        }
    }
    
    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.DecodeString("3[a2[c]]");
            Console.WriteLine(result);
        }
    }
}