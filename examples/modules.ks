let person = {
    name: string = "Kyryl",
    age: int = 19,

    function static_function() {
        println("Hello!");
    },

    function show_name(self) {
        println(self.name);
    }
};

person::static_function();
person.show_name();
