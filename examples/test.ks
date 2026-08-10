while true {
    digital_write("A", 7, true);
    delay(1000);

    digital_write("A", 8, true);
    delay(1000);

    digital_write("A", 7, false);
    delay(1000);

    digital_write("A", 8, false);
    delay(1000);
}
