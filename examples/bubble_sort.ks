function bubble_sort(numbers [int]) {
    for i in range(len(numbers)) {
        for j in range(len(numbers) - i - 1) {
            let n1 = numbers[j]!;
            let n2 = numbers[j + 1]!;

            if n1 > n2 {
                numbers[j] = n2;
                numbers[j + 1] = n1;
            }
        }
    }
}

let numbers = [83, 209, 59, 68, 30, 60];

println(numbers);
bubble_sort(numbers);
println(numbers);
