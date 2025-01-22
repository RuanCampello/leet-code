/**
 * @param {integer} init
 * @return { increment: Function, decrement: Function, reset: Function }
 */
var createCounter = function (init) {
    let counter = init;

    return {
        increment: () => ++counter,
        decrement: () => --counter,
        reset: () => counter = init,
    }
};

const counter = createCounter(5)
console.log(counter.increment())
console.log(counter.reset())
console.log(counter.decrement())