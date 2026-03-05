let test_module = {
    field_1: "Hello World",
    module_inside: {
        field_1: "Hi"
    }
};

test_module.module_inside.field_1;

let c = 20;

function test() {
    let a = 10;
    c = a;

    return a;
}

let b = test();
// b is gonna be error, because a was freed before return
