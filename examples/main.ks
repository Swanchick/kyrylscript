pub function display(list: [int]) {
    for element in list {
        println(element, ",");
    }
}

let a = [10, 20, 30, 40, 50];
display(a);
println("===============");
if true {
    let b = 130;
    a[0] = b;
}

display(a);