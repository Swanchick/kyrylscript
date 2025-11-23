let person1 = {
    name: "Kyryl",
    age: 19
};

let person2 = person1!;
person2.name = "asdasd";
person2.age = 20;

println(person1.name, " ", person1.age);
println(person2.name, " ", person2.age);
