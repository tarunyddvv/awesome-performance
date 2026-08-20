#include <cstdio>
#include <stdio.h>
#include <iostream>

using namespace std;

struct Rectangle {
  int length;
  int breadth;
  char x;
};

int main() {
    int A[5];
    A[0] = 12;
    A[1] = 14;
    A[2] = 18;

    cout<<sizeof(A)<<endl;
    cout<<A[1]<<endl;

    printf("%d\n", A[2]);

    struct Rectangle r1 = {10, 5, 'a'};

    printf("%lu\n", sizeof(r1));

    cout<<r1.length<<endl;
    cout<<r1.breadth<<endl;

    return 0;
}
