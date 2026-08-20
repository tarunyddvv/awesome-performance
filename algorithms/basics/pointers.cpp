#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <stdio.h>
#include <stdlib.h>

using namespace std;

struct Rectangle {
    int length;
    int breadth;
};

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

    int *p3;
    char *p4;
    float *p5;
    double *p6;
    int *p7;
    struct Rectangle *p8;

    cout<<sizeof(p3)<<endl;
    cout<<sizeof(p4)<<endl;
    cout<<sizeof(p5)<<endl;
    cout<<sizeof(p6)<<endl;
    cout<<sizeof(p7)<<endl;
    cout<<sizeof(p8)<<endl;

    struct Rectangle *r;

    r = (struct Rectangle *)malloc(sizeof(struct Rectangle));

    r->length = 10;
    r->breadth = 20;

    printf("length: %d\n", r->length);
    printf("breadth: %d\n", r->breadth);

    return 0;
}
