function bubble_sort(numbers: [int]) {
    for j in range(len(numbers)) {
        for i in range(len(numbers) - j - 1) {        
            let n1 = numbers[i]!;
            let n2 = numbers[i + 1]!;
            if n1 > n2 {
                numbers[i] = n2;
                numbers[i + 1] = n1;
            }
        }
    }
}

let numbers = [32, 45, 12, 76, 34, 23, 56, 23];
println(numbers);
bubble_sort(numbers);
println(numbers);
