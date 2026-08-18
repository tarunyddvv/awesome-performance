#include <sys/fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/errno.h>

// INFO: k stands for constant - a C lang thing
const char *kFilename = "my_file";
const int kFileExistsErr = 17;

int main() {
    umask(0); // set to 0 to enable all permissions to be set
    int file_descriptor = open(kFilename, O_WRONLY | O_CREAT | O_EXCL, 0644); // write only, create the file and only create if it does not exists
    if (file_descriptor == -1) {
        printf("There was a problem createing '%s'!\n", kFilename);
        if (errno == kFileExistsErr) {
            printf("The file already exists.\n");
        } else {
            printf("Unknown errorno: %d\n", errno);
        }
        return -1;
    }
    close(file_descriptor);

     return 0;
}
