#include <stdio.h>
#include <stdlib.h>
#include <iostream>

int main() {
    // INFO: static array - on the stack
    int a[5] = {1, 2, 3, 4, 5};

    for (int i:a) {
        printf("%d\n", i);
    }
    int *p;
    // INFO: memory allocation on heap in C
    p = (int *)malloc(5 * sizeof(int));

    // INFO: memory allocation on heap in C++
    // p = new int[5];

    // deallocation in C++
    // delete []p;

    // INFO: deallocation in C
    free(p);

    return 0;
}