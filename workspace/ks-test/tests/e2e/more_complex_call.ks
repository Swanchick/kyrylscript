function police(pin1 int, pin2 int) {
  digital_write("A", pin1, true);
  delay(1000);

  digital_write("A", pin2, true);
  delay(1000);

  digital_write("A", pin1, false);
  delay(1000);

  digital_write("A", pin2, false);
  delay(1000);
}

let a = 0;

while a < 10 {
  police(5, 6);

  a++;
}
