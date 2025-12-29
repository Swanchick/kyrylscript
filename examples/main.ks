function test_recursion(a: int): int {
    if a >= 10 {
        return 10;
    }

    return test_recursion(a + 1);
}
