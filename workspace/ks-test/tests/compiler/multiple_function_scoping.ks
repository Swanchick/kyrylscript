let stupid_var = 783;

function foo(): int {
    let var1 = 10;

    function bar(): int {
        let var2 = 20;

        return var2;
    }

    return bar() + var1;
}

let result = foo();
