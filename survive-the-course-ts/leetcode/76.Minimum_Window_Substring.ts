function mapIncr<K>(m: Map<K, number>, k: K) {
    if (m.has(k)) {
        m.set(k, m.get(k)! + 1);
        return;
    }
    m.set(k, 1);
}

function mapDecr<K>(m: Map<K, number>, k: K): boolean {
    if (!m.has(k)) {
        return false;
    }
    const currCount = m.get(k)!;
    if (currCount > 1) {
        m.set(k, currCount - 1);
        return true;
    } else {
        m.delete(k);
        return true;
    }
}

function minWindow(s: string, t: string): string {
    if (t === "") {
        return "";
    }
    if (s === "") {
        return "";
    }

    const charCounter = new Map();
    for (const ch of t) {
        mapIncr(charCounter, ch);
    }

    let left = 0;
    let right = 0;
    const charMatches = new Map(charCounter);
    const overflow = new Map();

    let resultL;
    let resultLength = Number.MAX_SAFE_INTEGER;

    while (right < s.length) {
        const ch = s[right];
        if (charCounter.has(ch)) {
            const decrSucc = mapDecr(charMatches, ch);
            if (!decrSucc) {
                mapIncr(overflow, ch);
            }

            if (charMatches.size === 0) {
                if (resultLength > right - left + 1) {
                    resultL = left;
                    resultLength = right - left + 1;
                }
                while (left <= right) {
                    const lch = s[left];
                    if (charCounter.has(lch)) {
                        if (resultLength > right - left + 1) {
                            resultL = left;
                            resultLength = right - left + 1;
                        }
                        if (overflow.has(lch)) {
                            mapDecr(overflow, lch);
                        } else {
                            mapIncr(charMatches, lch);
                            left++;
                            break;
                        }
                    }
                    left++;
                }
            }
        }
        right++;
    }
    return s.slice(resultL, resultL + resultLength);
};

console.log(minWindow("ADOBECODEBANC", "ABC"));

export {};