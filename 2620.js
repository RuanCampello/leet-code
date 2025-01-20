/**
 * @param {number} n
 * @return {Function} counter
 */
var createCounter = function (n) {
    let counter = n;
    return () => counter++;
};

const counter = createCounter(10)
console.log(counter()) // 10
console.log(counter()) // 11
console.log(counter()) // 12