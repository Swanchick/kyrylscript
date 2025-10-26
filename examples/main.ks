function room_new(name: string, people: [{ name: string, age: int }]): {room_name: string, people: [{name: string, age: int}]} {
    return {
        room_name: name,
        people: people
    };
}


let room = room_new(
    "Kitchen",
    [
        {name: "Kyryl", age: 19},
        {name: "Another person", age: 123}
    ]
);

println(room.people[0].name);

