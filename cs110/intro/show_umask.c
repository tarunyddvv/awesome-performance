#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>

int main() {
  mode_t old_mask = umask(0);
  umask(old_mask);

  printf("umask is set to %03o\n", old_mask);
  return 0;
}
