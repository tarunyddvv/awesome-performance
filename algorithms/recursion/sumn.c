#include <stdio.h>
#include <stdlib.h>

int sum(int n) {
    if (n == 0) {
        return 0;
    }

    return sum(n - 1) + n;
}

int main() {
    printf("sum: %d\n", sum(5));
    return 0;
}