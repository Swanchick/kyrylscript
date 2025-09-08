function display(a: [int]) {
    for i in a {
        println(i);
    }
}

let a = [10, 20, 30, 40];
if true {
    let b = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    a = b;
}

display(a);