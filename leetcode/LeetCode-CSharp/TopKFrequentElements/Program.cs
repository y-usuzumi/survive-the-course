using System;
using System.Collections.Generic;
using System.Linq;

namespace TopKFrequentElements
{
    public class Solution
    {
        public IList<int> TopKFrequent(int[] nums, int k) {
            var counter = new Dictionary<int, int>(nums.Length);
            foreach (var num in nums)
            {
                var success = counter.TryAdd(num, 1);
                if (!success)
                {
                    counter[num]++;
                }
            }
            var sorter = new SortedSet<Tuple<int, int>>(
                counter.Select(kvp => new Tuple<int, int>(kvp.Value, kvp.Key))
                );
            return sorter.Skip(counter.Count - k).Select(t => t.Item2).Reverse().ToList();
        }
    }
    static class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.TopKFrequent(new [] {1, 1, 1, 2, 2, 3}, 2);
            Console.WriteLine("{0}", result);
        }
    }
}