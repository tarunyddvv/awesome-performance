#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <stdio.h>
#include <stdlib.h>

using namespace std;

int main()
{
    int a=10;
    int *p;
    p=&a;

    cout<<"a: "<<a<<endl;
    cout<<"pointer: "<<p<<endl;
    cout<<"address of a: "<<&a<<endl;
    printf("%d\n", *p);

    // heap allocated array
    int *p2;
    // INFO: p2 = (int *)malloc(5 * sizeof(int));  OR
    // INFO: cpp method of allocating heap memory - dynamic memory allocation
    p2 = new int[5];

    p2[0] = 1;
    p2[1] = 3;
    p2[2] = 5;
    p2[3] = 10;
    p2[4] = 19;

    printf("array\n");
    for(int i=0;i<5;i++) {
        printf("%d\n", p2[i]);
    }

    // INFO: deallocating the heap memory - releasing the dynamically allocated memory
    // INFO: after using it.
    delete []p2;
    // INFO: in C
    // INFO: free(p2) will be used
    return 0;
}
