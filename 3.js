/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
    let sub = String();
    let maxLength = 0;

    for (const char of s) {
        if (sub.includes(char)) sub = sub.slice(sub.indexOf(char) + 1);
        
        sub += char;
        maxLength = Math.max(maxLength, sub.length);
    }

    return maxLength;
};


console.log(lengthOfLongestSubstring("abcabcbb"));
console.log(lengthOfLongestSubstring("bbbbb"));
console.log(lengthOfLongestSubstring("pwwkew"));