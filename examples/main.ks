let a = 10;

function test() {
    let b = 20;

    a = b;

    b = 100;

    debug();
}

test();
println(a);
