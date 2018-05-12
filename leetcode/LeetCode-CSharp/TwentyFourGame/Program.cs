using System;
using System.Globalization;
using System.Linq;

namespace TwentyFourGame
{
    public class Solution
    {
        void FillRemaining(double[] target, double[] left, double[] middle, double[] right)
        {
            var idx = 1;
            for (var i = 0; i < left.Length; i++, idx++)
            {
                target[idx] = left[i];
            }
            for (var i = 0; i < middle.Length; i++, idx++)
            {
                target[idx] = middle[i];
            }
            for (var i = 0; i < right.Length; i++, idx++)
            {
                target[idx] = right[i];
            }
        }

        bool JudgePoint24(double[] nums)
        {
            if (nums.Length == 1)
				return Math.Abs(nums[0] - 24) < 1e-6;
            for (var i = 0; i < nums.Length; i++)
            {
                var left = new double[i];
                for (var li = 0; li < i; li++)
                {
                    left[li] = nums[li];
                }
                for (var j = i + 1; j < nums.Length; j++)
                {
                    var middle = new double[j - i - 1];
                    for (int mi = i + 1; mi < j; mi++)
                    {
                        middle[mi - i - 1] = nums[mi];
                    }

                    var right = new double[nums.Length - j - 1];
                    for (var ri = j + 1; ri < nums.Length; ri++)
                    {
                        right[ri - j - 1] = nums[ri];
                    }

                    var n1 = nums[i];
                    var n2 = nums[j];
                    var arr = new double[nums.Length - 1];
                    FillRemaining(arr, left, middle, right);
                    {
                        // Add
                        arr[0] = n1 + n2;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                    {
                        // Subtract 1
                        arr[0] = n1 - n2;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                    {
                        // Subtract 2
                        arr[0] = n2 - n1;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                    {
                        // Multiply
                        arr[0] = n1 * n2;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                    {
                        // Divide 1
                        arr[0] = n1 / n2;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                    {
                        // Divide 2
                        arr[0] = n2 / n1;
                        if (JudgePoint24(arr))
                        {
                            return true;
                        }
                    }
                }
            }

            return false;
        }
        
        public bool JudgePoint24(int[] nums)
        {
            var dNums = nums.Select(n => (double) n).ToArray();
            return JudgePoint24(dNums);
        }
    }
    class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.JudgePoint24(new[] {3, 3, 8, 8});
            Console.WriteLine("{0}", result);
        }
    }
}