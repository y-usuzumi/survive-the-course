package org.ratina.leetcode.longest_palindrome_substring

import scala.collection.mutable._

object Solution {
    def longestPalindrome(s: String): String = {
        var (maxSpan, maxIdx)
        var chars = new ArrayBuffer[Char]
        for (ch <- s) {
            chars.append('*', ch);
        }
        chars.append('*')
        var lookup = new HashMap[Int, HashMap[Int, Boolean]]
        for (idx <- 0 until s.length) {
            lookup += ((idx, HashMap(0 -> true)))
        }
        println(lookup)
        s
    }
}
