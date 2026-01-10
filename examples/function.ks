//
// Function example
//

// If you don't give any type for return of the function, then it will return void by default
function hello(name: string) {
    println("Hello, ", name, "!");
}

function add(a int, b int): int {
    return a + b;
}

function sub(a int, b int): int {
    return a - b;
}

function mult(a int, b int): int {
    return a * b;
}

function divide(a int, b int): float {
    return a / b;
}

println("Add: ", add(10, 20));
println("Subtract: ", sub(10, 20));
println("Multiply: ", mult(10, 20));
println("Divide: ", divide(10, 20));
