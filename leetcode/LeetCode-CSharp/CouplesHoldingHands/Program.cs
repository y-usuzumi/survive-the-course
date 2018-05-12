using System;
using System.Collections.Generic;

namespace CouplesHoldingHands
{
    public class Solution
    {
        int findMyCouple(int me)
        {
            if (me % 2 == 0)
            {
                return me + 1;
            }
            return me - 1;
        }
        public int MinSwapsCouples(int[] row)
        {
            var inverseRow = new int[row.Length];
            var cnt = 0;
            for (int i = 0; i < row.Length; i++)
            {
                inverseRow[row[i]] = i;
            }
            for (int i = 0; i < row.Length; i += 2)
            {
                var me = row[i];
                var other = row[i + 1];
                var myCouple = findMyCouple(me);
                if (other == findMyCouple(me))
                {
                    continue;
                }
                var tmp = other;
                row[i + 1] = myCouple;
                var myCoupleIdx = inverseRow[myCouple];
                row[myCoupleIdx] = other;
                inverseRow[myCouple] = i + 1;
                inverseRow[other] = myCoupleIdx;
                cnt++;
            }
            return cnt;
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            var solution = new Solution();
            var result = solution.MinSwapsCouples(new[] { 0, 2, 1, 3 });
            Console.WriteLine("{0}", result);
        }
    }
}
