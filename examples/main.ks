function test(numbers: [int]) {
    numbers[0] = numbers[1]!;
    numbers[1] = numbers[0]!;
}



let numbers = [10, 20, 30, 40, 50, 60, 70, 80];
println(numbers);
test(numbers);
println(numbers);
