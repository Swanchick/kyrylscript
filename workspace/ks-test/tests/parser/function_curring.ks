function curry(a int): function(int): int {
    return function(b int): int {
        return a + b;
    };
}

let result1 = curry(10)(20);
