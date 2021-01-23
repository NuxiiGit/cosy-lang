#include <stdio.h>
int main() {
  signed char t0 = 1;
  signed char t1 = 8;
  signed char t2 = -t1;
  signed char t3 = t0 - t2;
  printf("%d\n", t3);
  return 0;
}
