function create_person(name string, age int): {name: string, age: int} {
  return {
    name: name,
    age: age
  };
}

let person = create_person("Kyryl", 20);
print("Name: ", person.name, ", ");
print("Age: ", person.age);
