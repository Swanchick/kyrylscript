function test(numbers: [int]) {
    let n1 = numbers[0]!;
    let n2 = numbers[1]!;

    // debug();

    numbers[0] = n2;
    numbers[1] = n1;
}


let numbers = [10, 20, 30, 40, 50, 60, 70, 80];
println(numbers);
test(numbers);
println(numbers);
