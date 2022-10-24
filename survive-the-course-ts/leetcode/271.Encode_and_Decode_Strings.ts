// Problem link: https://leetcode.cn/problems/encode-and-decode-strings/

/**
 * Encodes a list of strings to a single string.
 */
function encode(strs: string[]): string {
    let result = "";
    for (const s of strs) {
        for (const ch of s) {
            switch (ch) {
                case '\\':
                    result += '\\\\';
                    break;
                case ';':
                    result += '\\;';
                    break;
                default:
                    result += ch;
            }
        }
        result += ';'
    }
    return result.substring(0, result.length - 1);
};

/**
 * Decodes a single string to a list of strings.
 */
function decode(s: string): string[] {
    const result: string[] = [];
    let str = "";
    let inEscape = false;
    for (const ch of s) {
        switch (ch) {
            case '\\':
                if (inEscape) {
                    str += '\\';
                    inEscape = false;
                } else {
                    inEscape = true;
                }
                break;
            case ';':
                if (inEscape) {
                    str += ';';
                    inEscape = false;
                } else {
                    result.push(str);
                    str = "";
                }
                break;
            default:
                str += ch;
                break;
        }
    }
    result.push(str);
    return result;
};

/**
 * Your functions will be called as such:
 * decode(encode(strs));
 */

export { };