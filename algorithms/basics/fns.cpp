#include <iostream>
#include <stdio.h>
#include <stdlib.h>

using namespace std;

// INFO: call by reference should be used carefully
// INFO: it should not be used on expensive fns.
void swap(int &x, int &y) {
    int temp;
    temp = x;
    x = y;
    y = temp;
}

// INFO: Arrays are always pass by address; so here we have n as pass by value and A as address.
 void print(int A[], int n) {
     for(int i=0; i < n; i++) {
         cout<<A[i]<<endl;
     }
 }

 int *create(int n) {
     int *p = (int *)malloc(n);
     return p;
 }

int main()
{
    int a, b;
    a = 10;
    b = 20;
    swap(a, b);
    cout<<"a: "<<a<<" b: "<<b<<endl;


    int A[5] = {1, 2, 3, 4, 5};
    print(A, 5);

    int *B = create(5);
    B[0] = 6;
    B[1] = 7;
    B[2] = 8;
    B[3] = 9;
    B[4] = 10;

    print(B, 5);

}
