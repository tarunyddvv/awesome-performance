#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

static const int kWrongArgumentCount = 4;
static const int kReadFailure = 8;
static const int kWriteFailure = 16;
static const int kDefaultPermissions = 0644;

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "%s <source-file> <destination-file>\n", argv[0]);
        return kWrongArgumentCount;
    }

    int fdin = open(argv[1], O_RDONLY);
    if (fdin == -1) {
        perror(argv[1]);
        return kReadFailure;
    }

    int fdout = open(argv[2], O_WRONLY | O_CREAT | O_EXCL,
                     kDefaultPermissions);
    if (fdout == -1) {
        perror(argv[2]);
        close(fdin);
        return kWriteFailure;
    }

    while (true) {
        char buffer[1024];
        ssize_t bytesRead = read(fdin, buffer, sizeof(buffer));
        if (bytesRead == 0) break;
        if (bytesRead == -1) {
            perror("read");
            close(fdin);
            close(fdout);
            return kReadFailure;
        }

        ssize_t bytesWritten = 0;
        while (bytesWritten < bytesRead) {
            ssize_t result = write(fdout, buffer + bytesWritten,
                                   bytesRead - bytesWritten);
            if (result == -1) {
                perror("write");
                close(fdin);
                close(fdout);
                return kWriteFailure;
            }
            bytesWritten += result;
        }
    }

    close(fdin);
    close(fdout);
    return 0;
}
