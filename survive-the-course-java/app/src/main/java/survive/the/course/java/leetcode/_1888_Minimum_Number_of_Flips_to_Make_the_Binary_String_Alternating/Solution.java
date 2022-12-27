package survive.the.course.java.leetcode._1888_Minimum_Number_of_Flips_to_Make_the_Binary_String_Alternating;

// Time complexity: O(n). Space complexity: O(1).
//
// 1. If the length is even: If we can transform the string into alternating
//    sequence by taking certain number of Type-2 operations, then no matter how
//    many Type-1 operations we perform, the string is still alternating.
//    Therefore, we can simply count the number of Type-1 operations we perform
//    to transform it into 0101..., and calculate the number of the same
//    operations to transform it into 1010... by subtracting the result from
//    string length.
//
// 2. If the length is odd: We perform the same steps as 1, but the result will
//    only be the minimum steps for completely alternating sequences (0101...,
//    1010...). We also need to account for the rotation after which the
//    sequence will be a combination of two alternating sequences (e.g.
//    0101+10101). We can imaging removing the char from the beginning and
//    append it last, while updating the current number of flips:
//
//    Consider string '11010'. When we were constructing the sequence 01010...,
//    we have flipped the first bit. Therefore `flips == 1`. By moving it to the
//    end of the string, we actually didn't need to flip it. Therefore we
//    decrement `flips`. Conversely, if we didn't flip a bit in the first run,
//    we will need to flip it now. Therefore we increment `flips`. Each time we
//    update `flips`, we also need to update the result.
public class Solution {
    public int minFlips(String s) {
        char[] chars = s.toCharArray();
        char[] lookup = new char[]{'0', '1'};
        int flips = 0;
        for (int idx = 0; idx < chars.length; idx++) {
            if(chars[idx] != lookup[idx % 2]) {
                flips++;
            }
        }

        int result = Math.min(flips, chars.length - flips);
        if (chars.length % 2 == 0) {
            return result;
        }

        for (int idx = 0; idx < chars.length; idx++) {
            if (chars[idx] != lookup[idx%2]) {
                flips--;
            } else {
                flips++;
            }
            result = Math.min(result, flips);
            result = Math.min(result, chars.length - flips);
        }

        return result;
    }
}
