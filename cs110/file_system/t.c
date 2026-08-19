#include <stdbool.h>
#include <stdio.h>
#include <sys/fcntl.h>
#include <unistd.h>

#define DEFAULT_FLAGS (O_WRONLY | O_CREAT | O_TRUNC)
#define DEFAULT_PERMISSIONS 0644

// INFO: static means only visible in the current file
static void writeall(int fd, const char buffer[], size_t len) {
  size_t numWritten = 0;
  while (numWritten < len) {
    numWritten += write(fd, buffer + numWritten, len - numWritten);
  }
}

int main(int argc, char *argv[]) {
  int fds[argc];
  fds[0] = STDOUT_FILENO;
  for (size_t i = 1; i < argc; i++) {
    fds[i] = open(argv[i], O_WRONLY | O_CREAT | O_TRUNC, 0644);
  }

  while (true) {
    char buffer[2048];
    ssize_t numRead = read(STDIN_FILENO, buffer, sizeof(buffer));
    if (numRead == 0)
      break;
    for (size_t i = 0; i < argc; i++) {
      writeall(fds[i], buffer, numRead);
    }
  }
  for (size_t i = 0; i < argc; i++)
    close(fds[i]);
  return 0;
}
