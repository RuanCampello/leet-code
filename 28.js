/**
 * @param {string} haystack
 * @param {string} needle
 * @return {number}
 */
var strStr = function (haystack, needle) {
    const regex = new RegExp(needle, 'g');
    const match = haystack.search(regex)

    if (match !== undefined) {
        return match
    } else {
        return -1
    }
};

console.assert(strStr("sadbutsad", "sad") === 0)
