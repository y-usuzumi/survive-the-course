using System;
using System.Collections.Generic;
using System.Linq;

namespace Subsets
{
    public class Solution {
        public IList<IList<int>> Subsets(int[] nums)
        {
            return _Subsets(nums, 0);
        }

        IList<IList<int>> _Subsets(int[] nums, int startIdx)
        {
            var result = new List<IList<int>>();
            if (startIdx >= nums.Length)
            {
                return result;
            }
            if (startIdx == nums.Length - 1)
            {
                result.Add(new List<int>());
                result.Add(new List<int> {nums[startIdx]});
                return result;
            }

            var ss = _Subsets(nums, startIdx + 1);
            
            result.AddRange(ss);
            foreach (var s in ss)
            {
                var curr = new List<int>(s) {nums[startIdx]};
                result.Add(curr);
            }
            
            return result;
        }
    }
    
    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.Subsets(new[] {1, 2, 3});
            Console.WriteLine("START");
            foreach (var r in result)
            {
                Console.WriteLine(String.Join(", ", r.Select(n => n.ToString())));
            }
            Console.WriteLine("DONE");
        }
    }
}