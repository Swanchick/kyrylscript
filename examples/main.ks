function division(a: int, b: int): float {
    if b == 0 {
        return 0f;
    }

    return a / b;
}


println(division(10, 0));