let numbers = [10, 20, 30, 40, 50, 60, 70, 80];

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
