function get_length(numbers: [int]): int {
    let length = 0;

    for number in numbers {
        length++;
    }

    return length;
}

let numbers = [10, 20, 30, 40, 50, 60, 70];
let length = get_length(numbers);

println(length);

