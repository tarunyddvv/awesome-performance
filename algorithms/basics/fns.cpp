#include <iostream>
#include <stdio.h>
#include <stdlib.h>

using namespace std;

void swap(int *x, int *y) {
    int temp;
    temp = *x;
    *x = *y;
    *y = temp;
}

int main()
{
    int a, b;
    a = 10;
    b = 20;
    swap(&a, &b);

    cout<<"a: "<<a<<" b: "<<b<<endl;
}
