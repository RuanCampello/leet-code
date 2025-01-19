/**
 * @param {...(null|boolean|number|string|Array|Object)} args
 * @return {number}
 */
var argumentsLength = function(...args) {
    return args.length
};

console.assert(argumentsLength([5]) === 1);
console.assert(argumentsLength([{}, null, "3"]) === 1);