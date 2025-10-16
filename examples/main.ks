let person = {
    name: "Kyryl",
    age: 19,
    add(a: int, b: int): int {
        return a + b;
    }
};

println(person.add(person.age, 20));
