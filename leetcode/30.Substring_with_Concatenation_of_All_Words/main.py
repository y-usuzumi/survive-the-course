from typing import List
from collections import defaultdict

class Solution:
    def findSubstring(self, s: str, words: List[str]) -> List[int]:
        lwords = len(words)
        if lwords == 0:
            return []
        word_len = len(words[0])
        word_set = defaultdict(int)
        for word in words:
            word_set[word] += 1
        total_size = lwords * word_len
        lv0_idx = lv1_idx = 0
        used_words = word_set.copy()
        result = []
        while lv1_idx <= len(s) - word_len:
            curr_word = s[lv1_idx:lv1_idx+word_len]
            if curr_word in used_words:
                used_words[curr_word] -= 1
                if used_words[curr_word] == 0:
                    del used_words[curr_word]
                if not used_words:
                    result.append(lv0_idx)
                    used_words = word_set.copy()
                    lv0_idx, lv1_idx = lv0_idx+1, lv0_idx+1
                    continue
                lv1_idx += word_len
            else:
                used_words = word_set.copy()
                lv0_idx, lv1_idx = lv0_idx+1, lv0_idx+1
        return result
