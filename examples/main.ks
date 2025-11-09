function test(numbers: [int]) {
    println("========================== Check 1");
    
    let n1 = numbers[0]!;
    numbers[0] = n1;

    // debug();

    let n2 = numbers[0]!;
    numbers[0] = n2;

    // debug();

}

let numbers = [10, 20];
println(numbers);
test(numbers);


// println(numbers);
