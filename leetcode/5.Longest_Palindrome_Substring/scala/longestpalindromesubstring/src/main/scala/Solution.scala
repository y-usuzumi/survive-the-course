package org.ratina.leetcode.longest_palindrome_substring

import scala.collection.mutable._

object Solution {
    def longestPalindrome(s: String): String = {
        var (maxSize, maxIdx) = (1, 0)
        var chars = new ArrayBuffer[Char]
        for (ch <- s) {
            chars append('*', ch)
        }
        chars append '*'
        var lookup = new OpenHashMap[Int, ArrayBuffer[Int]](chars length)
        for (idx <- 0 until chars.length) {
            lookup += ((idx, ArrayBuffer(1)))
        }
        for (idx <- 1 until chars.length) {
            val sizeLookup = lookup(idx)
            for (palSize <- lookup(idx-1)) {
                val mirrorIdx = idx - palSize - palSize
                if (mirrorIdx >= 0 && chars(mirrorIdx) == chars(idx)) {
                    val newSize = palSize + 1
                    sizeLookup.append(newSize)
                    if (newSize > maxSize) {
                        maxSize = newSize
                        maxIdx = idx
                    }
                }
            }
        }
        val wholeSize = maxSize * 2 - 1
        val result = (for (ch <- chars slice(maxIdx - wholeSize + 1, maxIdx + 1) if ch != '*') yield ch).mkString
        result
    }
}
