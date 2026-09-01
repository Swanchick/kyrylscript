let a = 0;

while a < 10 {
  digital_write("A", 5, true);
  delay(1000);

  digital_write("A", 6, true);
  delay(1000);

  digital_write("A", 5, false);
  delay(1000);

  digital_write("A", 6, false);
  delay(1000);

  a++;
}
