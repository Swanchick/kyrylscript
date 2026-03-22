function curry(a int): function(int): function(int): int {
    return function(b int): function(int): int {
        return function(c int): int {
            return a + b + c;
        };
    };
}

let result = curry(10)(20)(30);
