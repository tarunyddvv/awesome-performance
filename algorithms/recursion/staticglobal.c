#include <stdio.h>
#include <stdlib.h>

int fun(int x) {
    static int n = 0;
    if (x > 0) {
        n++;
        return fun(x - 1) + n;
    }
    return 0;
}

int main() {
    int r = 5;
    int res = fun(r);

    printf("res: %d\n", res);
    return 0;    
}