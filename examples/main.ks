function create_person(name: string, age: int): { name: string, age: int } {
    return {
        name: name,
        age: age
    };
}


let person1 = create_person("Kyryl", 19);


println(person1.name);

let person2 = create_person("aksjdl", 29);
println(person2.name);
