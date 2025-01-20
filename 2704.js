/**
 * @param {string} val
 * @return {Object}
 */
var expect = function (val) {
    return {
        toBe: (innerVal) => {
            if (val !== innerVal) throw new Error("Not Equal")
            else return true
        }, notToBe: (innerVal) => {
            if (val === innerVal) throw new Error("Equal")
            else return true
        }
    }
};