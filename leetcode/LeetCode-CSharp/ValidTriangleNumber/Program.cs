using System;

namespace ValidTriangleNumber
{
    public class Solution
    {
        public int TriangleNumber(int[] nums)
        {
            var cnt = 0;
            Array.Sort(nums);
            var mIdx = 0;
            while (mIdx <= nums.Length - 1 && nums[mIdx++] == 0) ;
            for (var i = mIdx - 1; i < nums.Length - 2; i++)
            {
                var k = i + 2;
                for (var j = i + 1; j < nums.Length - 1; j++)
                {
                    var max = nums[i] + nums[j];
                    for (; k < nums.Length && nums[k] < max; k++) ;
                    cnt += k - j - 1;
                }
            }

            return cnt;
        }
    }

    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.TriangleNumber(new[] {2, 2, 3, 4});
            Console.WriteLine("{0}", result);
        }
    }
}