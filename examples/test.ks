while true {
    digital_write("A", 7, true);
    println("Hey 1");
    delay(1000);
    println("Hey 2");
    digital_write("A", 7, false);
    delay(1000);
}
