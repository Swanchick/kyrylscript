function swap(numbers [int]) {
  let i = numbers[0]!;
  let j = numbers[1]!;

  numbers[0] = j;
  numbers[1] = i;
}

let numbers = [10, 20];

print(numbers[0], ",", numbers[1], ";");
swap(numbers);
print(numbers[0], ",", numbers[1], ";");
