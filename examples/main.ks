function test(name: string, age: int): {name: string, age: int} {        
    return {
        name: name,
        age: age
    };
}

let person = test("Kyryl", 19);
println(person);
