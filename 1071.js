/**
 * @param {number} a
 * @param {number} b
 * @return {number}
 */
function gcd(a, b) {
    while (b !== 0) {
        [a, b] = [b, a % b];
    }
    return a
}

/**
 * @param {string} str1
 * @param {string} str2
 * @return {string}
 */
var gcdOfStrings = function (str1, str2) {
    if (str1 + str2 !== str2 + str1) {
        return ''
    }

    return str1.slice(0, gcd(str1.length, str2.length));
};

console.log(gcdOfStrings("ABCABC", "ABC"));
console.log(gcdOfStrings("ABABAB", "ABAB"));
console.log(gcdOfStrings("LEET", "CODE"));
console.log(gcdOfStrings("TAUXXTAUXXTAUXXTAUXXTAUXX", "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX"));
console.log(gcdOfStrings("ABABABAB", "ABAB"));