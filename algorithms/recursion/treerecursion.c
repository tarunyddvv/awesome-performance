#include <stdio.h>
#include <stdlib.h>

void fun(int x) {
    if (x > 0)
    {
        printf("%d\n", x);
        fun(x-1);
        fun(x-1);
    }
}

int main() {
    fun(3);
    return 0;
}