#include <stdio.h>
#include <stdint.h>
int main() {
  int8_t t0 = 1;
  int8_t t1 = 8;
  int8_t t2 = -t1;
  int8_t t3 = t0 - t2;
  printf("%d\n", t3);
  return 0;
}
